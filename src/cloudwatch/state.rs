use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::CloudwatchError;
use super::types::*;

#[allow(dead_code)]
struct CloudwatchStateInner {
    alarms: HashMap<String, AlarmInfo>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
pub struct CloudwatchState {
    inner: Arc<Mutex<CloudwatchStateInner>>,
}

impl CloudwatchState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        CloudwatchState {
            inner: Arc::new(Mutex::new(CloudwatchStateInner {
                alarms: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_alarm(&self, name: String) -> Result<AlarmInfo, CloudwatchError> {
        let mut state = self.inner.lock().await;
        if state.alarms.contains_key(&name) {
            return Err(CloudwatchError::ResourceAlreadyExistsException(format!("Alarm {} already exists", name)));
        }
        let arn = format!("arn:aws:cloudwatch:{}:{}:alarms/{}", state.region, state.account_id, name);
        let info = AlarmInfo {
            alarm_name: name.clone(),
            alarm_arn: arn,
            status: "ACTIVE".to_string(),
        };
        state.alarms.insert(name, info.clone());
        Ok(info)
    }

    #[allow(dead_code)]
    pub async fn describe_alarm(&self, name: &str) -> Result<AlarmInfo, CloudwatchError> {
        let state = self.inner.lock().await;
        state.alarms.get(name).cloned()
            .ok_or_else(|| CloudwatchError::ResourceNotFoundException(format!("Alarm {} not found", name)))
    }

    #[allow(dead_code)]
    pub async fn list_alarms(&self) -> Result<Vec<AlarmInfo>, CloudwatchError> {
        let state = self.inner.lock().await;
        Ok(state.alarms.values().cloned().collect())
    }

    #[allow(dead_code)]
    pub async fn delete_alarm(&self, name: &str) -> Result<(), CloudwatchError> {
        let mut state = self.inner.lock().await;
        state.alarms.remove(name)
            .ok_or_else(|| CloudwatchError::ResourceNotFoundException(format!("Alarm {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = CloudwatchState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_alarm() {
        let state = CloudwatchState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.create_alarm("test-resource".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_alarm_duplicate() {
        let state = CloudwatchState::new("123456789012".to_string(), "us-east-1".to_string());
        state.create_alarm("dup".to_string()).await.unwrap();
        let result = state.create_alarm("dup".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_alarm_not_found() {
        let state = CloudwatchState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.describe_alarm("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_alarms() {
        let state = CloudwatchState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_alarms().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_alarm_not_found() {
        let state = CloudwatchState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_alarm("nonexistent").await;
        assert!(result.is_err());
    }
}
