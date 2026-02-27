use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use super::error::SfnError;
use super::types::*;

struct StateMachine {
    arn: String,
    name: String,
    definition: String,
    role_arn: String,
    machine_type: String,
    created: f64,
    tags: HashMap<String, String>,
}

struct Execution {
    arn: String,
    state_machine_arn: String,
    name: String,
    status: String, // RUNNING, SUCCEEDED, FAILED, ABORTED
    start_date: f64,
    stop_date: Option<f64>,
    input: Option<String>,
    output: Option<String>,
    history: Vec<HistoryEvent>,
}

struct SfnStateInner {
    state_machines: HashMap<String, StateMachine>,
    executions: HashMap<String, Execution>,
    account_id: String,
    region: String,
}

pub struct SfnState {
    inner: Arc<Mutex<SfnStateInner>>,
}

impl SfnState {
    pub fn new(account_id: String, region: String) -> Self {
        SfnState {
            inner: Arc::new(Mutex::new(SfnStateInner {
                state_machines: HashMap::new(),
                executions: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    fn now() -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }

    pub async fn create_state_machine(
        &self,
        req: CreateStateMachineRequest,
    ) -> Result<CreateStateMachineResponse, SfnError> {
        let mut state = self.inner.lock().await;
        let arn = format!(
            "arn:aws:states:{}:{}:stateMachine:{}",
            state.region, state.account_id, req.name
        );
        if state.state_machines.contains_key(&arn) {
            return Err(SfnError::StateMachineAlreadyExists(format!(
                "State machine already exists: {}", arn
            )));
        }
        let machine_type = req.machine_type.unwrap_or_else(|| "STANDARD".to_string());
        let created = Self::now();
        let mut tags = HashMap::new();
        if let Some(t) = req.tags {
            for tag in t { tags.insert(tag.key, tag.value); }
        }
        state.state_machines.insert(arn.clone(), StateMachine {
            arn: arn.clone(),
            name: req.name,
            definition: req.definition,
            role_arn: req.role_arn,
            machine_type,
            created,
            tags,
        });
        Ok(CreateStateMachineResponse {
            state_machine_arn: arn,
            creation_date: created,
        })
    }

    pub async fn delete_state_machine(&self, req: DeleteStateMachineRequest) -> Result<(), SfnError> {
        let mut state = self.inner.lock().await;
        if state.state_machines.remove(&req.state_machine_arn).is_none() {
            return Err(SfnError::StateMachineDoesNotExist(format!(
                "State machine does not exist: {}", req.state_machine_arn
            )));
        }
        Ok(())
    }

    pub async fn describe_state_machine(
        &self,
        req: DescribeStateMachineRequest,
    ) -> Result<DescribeStateMachineResponse, SfnError> {
        let state = self.inner.lock().await;
        let sm = state.state_machines.get(&req.state_machine_arn)
            .ok_or_else(|| SfnError::StateMachineDoesNotExist(format!(
                "State machine does not exist: {}", req.state_machine_arn
            )))?;
        Ok(DescribeStateMachineResponse {
            state_machine_arn: sm.arn.clone(),
            name: sm.name.clone(),
            status: "ACTIVE".to_string(),
            definition: sm.definition.clone(),
            role_arn: sm.role_arn.clone(),
            machine_type: sm.machine_type.clone(),
            creation_date: sm.created,
        })
    }

    pub async fn list_state_machines(
        &self,
        req: ListStateMachinesRequest,
    ) -> Result<ListStateMachinesResponse, SfnError> {
        let state = self.inner.lock().await;
        let mut machines: Vec<StateMachineListItem> = state.state_machines.values().map(|sm| StateMachineListItem {
            state_machine_arn: sm.arn.clone(),
            name: sm.name.clone(),
            machine_type: sm.machine_type.clone(),
            creation_date: sm.created,
        }).collect();
        machines.sort_by(|a, b| a.name.cmp(&b.name));
        let limit = req.max_results.unwrap_or(1000);
        let has_more = machines.len() > limit;
        machines.truncate(limit);
        Ok(ListStateMachinesResponse {
            state_machines: machines,
            next_token: if has_more { Some("next".to_string()) } else { None },
        })
    }

    pub async fn start_execution(
        &self,
        req: StartExecutionRequest,
    ) -> Result<StartExecutionResponse, SfnError> {
        let mut state = self.inner.lock().await;
        if !state.state_machines.contains_key(&req.state_machine_arn) {
            return Err(SfnError::StateMachineDoesNotExist(format!(
                "State machine does not exist: {}", req.state_machine_arn
            )));
        }
        let exec_name = req.name.unwrap_or_else(|| Uuid::new_v4().to_string());
        let sm_name = req.state_machine_arn.rsplit(':').next().unwrap_or("unknown");
        let exec_arn = format!(
            "arn:aws:states:{}:{}:execution:{}:{}",
            state.region, state.account_id, sm_name, exec_name
        );
        if state.executions.contains_key(&exec_arn) {
            return Err(SfnError::ExecutionAlreadyExists(format!(
                "Execution already exists: {}", exec_arn
            )));
        }
        let now = Self::now();
        let history = vec![
            HistoryEvent {
                id: 1,
                event_type: "ExecutionStarted".to_string(),
                timestamp: now,
                previous_event_id: 0,
                execution_started_event_details: Some(serde_json::json!({
                    "input": req.input.as_deref().unwrap_or("{}"),
                    "roleArn": state.state_machines[&req.state_machine_arn].role_arn.clone(),
                })),
                execution_succeeded_event_details: None,
            },
        ];
        state.executions.insert(exec_arn.clone(), Execution {
            arn: exec_arn.clone(),
            state_machine_arn: req.state_machine_arn,
            name: exec_name,
            status: "RUNNING".to_string(),
            start_date: now,
            stop_date: None,
            input: req.input,
            output: None,
            history,
        });
        Ok(StartExecutionResponse {
            execution_arn: exec_arn,
            start_date: now,
        })
    }

    pub async fn stop_execution(
        &self,
        req: StopExecutionRequest,
    ) -> Result<StopExecutionResponse, SfnError> {
        let mut state = self.inner.lock().await;
        let exec = state.executions.get_mut(&req.execution_arn)
            .ok_or_else(|| SfnError::ExecutionDoesNotExist(format!(
                "Execution does not exist: {}", req.execution_arn
            )))?;
        let now = Self::now();
        exec.status = "ABORTED".to_string();
        exec.stop_date = Some(now);
        Ok(StopExecutionResponse { stop_date: now })
    }

    pub async fn describe_execution(
        &self,
        req: DescribeExecutionRequest,
    ) -> Result<DescribeExecutionResponse, SfnError> {
        let state = self.inner.lock().await;
        let exec = state.executions.get(&req.execution_arn)
            .ok_or_else(|| SfnError::ExecutionDoesNotExist(format!(
                "Execution does not exist: {}", req.execution_arn
            )))?;
        Ok(DescribeExecutionResponse {
            execution_arn: exec.arn.clone(),
            state_machine_arn: exec.state_machine_arn.clone(),
            name: exec.name.clone(),
            status: exec.status.clone(),
            start_date: exec.start_date,
            stop_date: exec.stop_date,
            input: exec.input.clone(),
            output: exec.output.clone(),
        })
    }

    pub async fn list_executions(
        &self,
        req: ListExecutionsRequest,
    ) -> Result<ListExecutionsResponse, SfnError> {
        let state = self.inner.lock().await;
        let mut execs: Vec<ExecutionListItem> = state.executions.values()
            .filter(|e| {
                req.state_machine_arn.as_ref().map(|arn| &e.state_machine_arn == arn).unwrap_or(true)
                && req.status_filter.as_ref().map(|s| &e.status == s).unwrap_or(true)
            })
            .map(|e| ExecutionListItem {
                execution_arn: e.arn.clone(),
                state_machine_arn: e.state_machine_arn.clone(),
                name: e.name.clone(),
                status: e.status.clone(),
                start_date: e.start_date,
                stop_date: e.stop_date,
            })
            .collect();
        execs.sort_by(|a, b| b.start_date.partial_cmp(&a.start_date).unwrap());
        let limit = req.max_results.unwrap_or(1000);
        let has_more = execs.len() > limit;
        execs.truncate(limit);
        Ok(ListExecutionsResponse {
            executions: execs,
            next_token: if has_more { Some("next".to_string()) } else { None },
        })
    }

    pub async fn get_execution_history(
        &self,
        req: GetExecutionHistoryRequest,
    ) -> Result<GetExecutionHistoryResponse, SfnError> {
        let state = self.inner.lock().await;
        let exec = state.executions.get(&req.execution_arn)
            .ok_or_else(|| SfnError::ExecutionDoesNotExist(format!(
                "Execution does not exist: {}", req.execution_arn
            )))?;
        let mut events = exec.history.clone();
        if req.reverse_order.unwrap_or(false) {
            events.reverse();
        }
        Ok(GetExecutionHistoryResponse {
            events,
            next_token: None,
        })
    }

    pub async fn send_task_success(&self, _req: SendTaskSuccessRequest) -> Result<(), SfnError> {
        // Stub: accept any token
        Ok(())
    }

    pub async fn send_task_failure(&self, _req: SendTaskFailureRequest) -> Result<(), SfnError> {
        Ok(())
    }

    pub async fn send_task_heartbeat(&self, _req: SendTaskHeartbeatRequest) -> Result<(), SfnError> {
        Ok(())
    }

    pub async fn tag_resource(&self, req: TagResourceRequest) -> Result<(), SfnError> {
        let mut state = self.inner.lock().await;
        if let Some(sm) = state.state_machines.get_mut(&req.resource_arn) {
            for tag in req.tags { sm.tags.insert(tag.key, tag.value); }
            return Ok(());
        }
        Err(SfnError::InvalidArn(format!("Resource not found: {}", req.resource_arn)))
    }

    pub async fn untag_resource(&self, req: UntagResourceRequest) -> Result<(), SfnError> {
        let mut state = self.inner.lock().await;
        if let Some(sm) = state.state_machines.get_mut(&req.resource_arn) {
            for key in &req.tag_keys { sm.tags.remove(key); }
            return Ok(());
        }
        Err(SfnError::InvalidArn(format!("Resource not found: {}", req.resource_arn)))
    }

    pub async fn list_tags_for_resource(
        &self,
        req: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, SfnError> {
        let state = self.inner.lock().await;
        if let Some(sm) = state.state_machines.get(&req.resource_arn) {
            let tags = sm.tags.iter().map(|(k, v)| Tag { key: k.clone(), value: v.clone() }).collect();
            return Ok(ListTagsForResourceResponse { tags });
        }
        Err(SfnError::InvalidArn(format!("Resource not found: {}", req.resource_arn)))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = SfnState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_delete_state_machine_not_found() {
        let state = SfnState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteStateMachineRequest::default();
        let result = state.delete_state_machine(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_send_task_success() {
        let state = SfnState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = SendTaskSuccessRequest::default();
        let _ = state.send_task_success(req).await;
    }
    #[tokio::test]
    async fn test_send_task_failure() {
        let state = SfnState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = SendTaskFailureRequest::default();
        let _ = state.send_task_failure(req).await;
    }
    #[tokio::test]
    async fn test_send_task_heartbeat() {
        let state = SfnState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = SendTaskHeartbeatRequest::default();
        let _ = state.send_task_heartbeat(req).await;
    }
    #[tokio::test]
    async fn test_tag_resource() {
        let state = SfnState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = TagResourceRequest::default();
        let _ = state.tag_resource(req).await;
    }
    #[tokio::test]
    async fn test_untag_resource() {
        let state = SfnState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = UntagResourceRequest::default();
        let _ = state.untag_resource(req).await;
    }

    fn make_state() -> SfnState {
        SfnState::new("123456789012".to_string(), "us-east-1".to_string())
    }

    async fn create_sm(state: &SfnState) -> String {
        let req = CreateStateMachineRequest {
            name: "test-sm".to_string(),
            definition: r#"{"StartAt":"Hello","States":{"Hello":{"Type":"Pass","End":true}}}"#.to_string(),
            role_arn: "arn:aws:iam::123456789012:role/sfn-role".to_string(),
            ..Default::default()
        };
        state.create_state_machine(req).await.unwrap().state_machine_arn
    }

    #[tokio::test]
    async fn test_create_state_machine() {
        let state = make_state();
        let arn = create_sm(&state).await;
        assert!(arn.contains("test-sm"));
    }

    #[tokio::test]
    async fn test_describe_state_machine() {
        let state = make_state();
        let arn = create_sm(&state).await;
        let req = DescribeStateMachineRequest { state_machine_arn: arn };
        let result = state.describe_state_machine(req).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "test-sm");
    }

    #[tokio::test]
    async fn test_describe_state_machine_not_found() {
        let state = make_state();
        let req = DescribeStateMachineRequest { state_machine_arn: "arn:fake".to_string() };
        assert!(state.describe_state_machine(req).await.is_err());
    }

    #[tokio::test]
    async fn test_list_state_machines() {
        let state = make_state();
        create_sm(&state).await;
        let req = ListStateMachinesRequest::default();
        let result = state.list_state_machines(req).await.unwrap();
        assert_eq!(result.state_machines.len(), 1);
    }

    #[tokio::test]
    async fn test_delete_state_machine_success() {
        let state = make_state();
        let arn = create_sm(&state).await;
        let req = DeleteStateMachineRequest { state_machine_arn: arn };
        assert!(state.delete_state_machine(req).await.is_ok());
    }

    #[tokio::test]
    async fn test_start_execution() {
        let state = make_state();
        let arn = create_sm(&state).await;
        let req = StartExecutionRequest {
            state_machine_arn: arn,
            input: Some(r#"{"key":"value"}"#.to_string()),
            ..Default::default()
        };
        let result = state.start_execution(req).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_execution() {
        let state = make_state();
        let sm_arn = create_sm(&state).await;
        let exec = state.start_execution(StartExecutionRequest {
            state_machine_arn: sm_arn,
            ..Default::default()
        }).await.unwrap();
        let req = DescribeExecutionRequest { execution_arn: exec.execution_arn };
        let result = state.describe_execution(req).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_stop_execution() {
        let state = make_state();
        let sm_arn = create_sm(&state).await;
        let exec = state.start_execution(StartExecutionRequest {
            state_machine_arn: sm_arn,
            ..Default::default()
        }).await.unwrap();
        let req = StopExecutionRequest { execution_arn: exec.execution_arn };
        assert!(state.stop_execution(req).await.is_ok());
    }

    #[tokio::test]
    async fn test_list_executions() {
        let state = make_state();
        let sm_arn = create_sm(&state).await;
        state.start_execution(StartExecutionRequest {
            state_machine_arn: sm_arn.clone(),
            ..Default::default()
        }).await.unwrap();
        let req = ListExecutionsRequest { state_machine_arn: Some(sm_arn), ..Default::default() };
        let result = state.list_executions(req).await.unwrap();
        assert_eq!(result.executions.len(), 1);
    }

    #[tokio::test]
    async fn test_get_execution_history() {
        let state = make_state();
        let sm_arn = create_sm(&state).await;
        let exec = state.start_execution(StartExecutionRequest {
            state_machine_arn: sm_arn,
            ..Default::default()
        }).await.unwrap();
        let req = GetExecutionHistoryRequest {
            execution_arn: exec.execution_arn,
            ..Default::default()
        };
        let result = state.get_execution_history(req).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_tag_and_list_tags() {
        let state = make_state();
        let arn = create_sm(&state).await;
        state.tag_resource(TagResourceRequest {
            resource_arn: arn.clone(),
            tags: vec![Tag { key: "env".to_string(), value: "test".to_string() }],
        }).await.unwrap();
        let result = state.list_tags_for_resource(ListTagsForResourceRequest { resource_arn: arn }).await.unwrap();
        assert_eq!(result.tags.len(), 1);
    }
}
