use std::sync::Arc;

use clap::Parser;

mod apigateway;
mod cloudwatchlogs;
mod cognito;
mod dynamodb;
mod eventbridge;
mod firehose;
mod kinesis;
mod kms;
mod lambda;
mod memorydb;
mod s3;
mod secretsmanager;
mod ses;
mod sns;
mod sqs;
mod ssm;
mod stepfunctions;

#[derive(Parser)]
#[command(
    name = "aws-inmemory-services",
    about = "Local AWS services: S3, SNS, SQS, DynamoDB, Lambda, Firehose, MemoryDB, Cognito, API Gateway, KMS, Secrets Manager, Kinesis, EventBridge, Step Functions, SSM Parameter Store, CloudWatch Logs, SES"
)]
struct Args {
    #[arg(long, default_value = "9000")]
    s3_port: u16,
    #[arg(long, default_value = "9911")]
    sns_port: u16,
    #[arg(long, default_value = "9324")]
    sqs_port: u16,
    #[arg(long, default_value = "8000")]
    dynamodb_port: u16,
    #[arg(long, default_value = "9001")]
    lambda_port: u16,
    #[arg(long, default_value = "4573")]
    firehose_port: u16,
    #[arg(long, default_value = "6379")]
    memorydb_port: u16,
    #[arg(long, default_value = "9229")]
    cognito_port: u16,
    #[arg(long, default_value = "4567")]
    apigateway_port: u16,
    #[arg(long, default_value = "7600")]
    kms_port: u16,
    #[arg(long, default_value = "7700")]
    secretsmanager_port: u16,
    #[arg(long, default_value = "4568")]
    kinesis_port: u16,
    #[arg(long, default_value = "9195")]
    eventbridge_port: u16,
    #[arg(long, default_value = "8083")]
    stepfunctions_port: u16,
    #[arg(long, default_value = "9100")]
    ssm_port: u16,
    #[arg(long, default_value = "9201")]
    cloudwatchlogs_port: u16,
    #[arg(long, default_value = "9300")]
    ses_port: u16,
    #[arg(long, default_value = "us-east-1")]
    region: String,
    #[arg(long, default_value = "000000000000")]
    account_id: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let s3_state = Arc::new(s3::state::S3State::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let sns_state = Arc::new(sns::state::SnsState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let sqs_state = Arc::new(sqs::state::SqsState::new(
        args.account_id.clone(),
        args.region.clone(),
        args.sqs_port,
    ));
    let dynamodb_state = Arc::new(dynamodb::state::DynamoDbState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let lambda_state = Arc::new(lambda::state::LambdaState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let firehose_state = Arc::new(firehose::state::FirehoseState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let memorydb_state = Arc::new(memorydb::state::MemoryDbState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let cognito_state = Arc::new(cognito::state::CognitoState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let apigateway_state = Arc::new(apigateway::state::ApiGatewayState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let kms_state = Arc::new(kms::state::KmsState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let secretsmanager_state = Arc::new(secretsmanager::state::SecretsManagerState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let kinesis_state = Arc::new(kinesis::state::KinesisState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let eventbridge_state = Arc::new(eventbridge::state::EventBridgeState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let stepfunctions_state = Arc::new(stepfunctions::state::SfnState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let ssm_state = Arc::new(ssm::state::SsmState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let cloudwatchlogs_state = Arc::new(cloudwatchlogs::state::CwlState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let ses_state = Arc::new(ses::state::SesState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));

    let s3_app = s3::server::create_router(s3_state);
    let sns_app = sns::server::create_router(sns_state);
    let sqs_app = sqs::server::create_router(sqs_state);
    let dynamodb_app = dynamodb::server::create_router(dynamodb_state);
    let lambda_app = lambda::server::create_router(lambda_state);
    let firehose_app = firehose::server::create_router(firehose_state);
    let memorydb_app = memorydb::server::create_router(memorydb_state);
    let cognito_app = cognito::server::create_router(cognito_state);
    let apigateway_app = apigateway::server::create_router(apigateway_state);
    let kms_app = kms::server::create_router(kms_state);
    let secretsmanager_app = secretsmanager::server::create_router(secretsmanager_state);
    let kinesis_app = kinesis::server::create_router(kinesis_state);
    let eventbridge_app = eventbridge::server::create_router(eventbridge_state);
    let stepfunctions_app = stepfunctions::server::create_router(stepfunctions_state);
    let ssm_app = ssm::server::create_router(ssm_state);
    let cloudwatchlogs_app = cloudwatchlogs::server::create_router(cloudwatchlogs_state);
    let ses_app = ses::server::create_router(ses_state);

    macro_rules! spawn_service {
        ($app:expr, $port:expr, $name:expr) => {{
            let port = $port;
            let app = $app;
            tokio::spawn(async move {
                let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
                    .await
                    .unwrap();
                println!("{} service listening on port {}", $name, port);
                axum::serve(listener, app).await.unwrap();
            })
        }};
    }

    let s3_handle = spawn_service!(s3_app, args.s3_port, "S3");
    let sns_handle = spawn_service!(sns_app, args.sns_port, "SNS");
    let sqs_handle = spawn_service!(sqs_app, args.sqs_port, "SQS");
    let dynamodb_handle = spawn_service!(dynamodb_app, args.dynamodb_port, "DynamoDB");
    let lambda_handle = spawn_service!(lambda_app, args.lambda_port, "Lambda");
    let firehose_handle = spawn_service!(firehose_app, args.firehose_port, "Firehose");
    let memorydb_handle = spawn_service!(memorydb_app, args.memorydb_port, "MemoryDB");
    let cognito_handle = spawn_service!(cognito_app, args.cognito_port, "Cognito");
    let apigateway_handle = spawn_service!(apigateway_app, args.apigateway_port, "API Gateway");
    let kms_handle = spawn_service!(kms_app, args.kms_port, "KMS");
    let secretsmanager_handle =
        spawn_service!(secretsmanager_app, args.secretsmanager_port, "Secrets Manager");
    let kinesis_handle = spawn_service!(kinesis_app, args.kinesis_port, "Kinesis");
    let eventbridge_handle = spawn_service!(eventbridge_app, args.eventbridge_port, "EventBridge");
    let stepfunctions_handle =
        spawn_service!(stepfunctions_app, args.stepfunctions_port, "Step Functions");
    let ssm_handle = spawn_service!(ssm_app, args.ssm_port, "SSM Parameter Store");
    let cloudwatchlogs_handle =
        spawn_service!(cloudwatchlogs_app, args.cloudwatchlogs_port, "CloudWatch Logs");
    let ses_handle = spawn_service!(ses_app, args.ses_port, "SES");

    tokio::select! {
        r = s3_handle => r.unwrap(),
        r = sns_handle => r.unwrap(),
        r = sqs_handle => r.unwrap(),
        r = dynamodb_handle => r.unwrap(),
        r = lambda_handle => r.unwrap(),
        r = firehose_handle => r.unwrap(),
        r = memorydb_handle => r.unwrap(),
        r = cognito_handle => r.unwrap(),
        r = apigateway_handle => r.unwrap(),
        r = kms_handle => r.unwrap(),
        r = secretsmanager_handle => r.unwrap(),
        r = kinesis_handle => r.unwrap(),
        r = eventbridge_handle => r.unwrap(),
        r = stepfunctions_handle => r.unwrap(),
        r = ssm_handle => r.unwrap(),
        r = cloudwatchlogs_handle => r.unwrap(),
        r = ses_handle => r.unwrap(),
    }
}
