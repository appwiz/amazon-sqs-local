use std::sync::Arc;

use clap::Parser;

mod apigateway;
mod cognito;
mod dynamodb;
mod firehose;
mod lambda;
mod memorydb;
mod s3;
mod sns;
mod sqs;

#[derive(Parser)]
#[command(
    name = "aws-inmemory-services",
    about = "Local AWS S3, SNS, SQS, DynamoDB, Lambda, Firehose, MemoryDB, Cognito, and API Gateway services"
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

    let s3_app = s3::server::create_router(s3_state);
    let sns_app = sns::server::create_router(sns_state);
    let sqs_app = sqs::server::create_router(sqs_state);
    let dynamodb_app = dynamodb::server::create_router(dynamodb_state);
    let lambda_app = lambda::server::create_router(lambda_state);
    let firehose_app = firehose::server::create_router(firehose_state);
    let memorydb_app = memorydb::server::create_router(memorydb_state);
    let cognito_app = cognito::server::create_router(cognito_state);
    let apigateway_app = apigateway::server::create_router(apigateway_state);

    let s3_port = args.s3_port;
    let sns_port = args.sns_port;
    let sqs_port = args.sqs_port;
    let dynamodb_port = args.dynamodb_port;
    let lambda_port = args.lambda_port;
    let firehose_port = args.firehose_port;
    let memorydb_port = args.memorydb_port;
    let cognito_port = args.cognito_port;
    let apigateway_port = args.apigateway_port;

    let s3_handle = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", s3_port))
            .await
            .unwrap();
        println!("S3 service listening on port {}", s3_port);
        axum::serve(listener, s3_app).await.unwrap();
    });

    let sns_handle = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", sns_port))
            .await
            .unwrap();
        println!("SNS service listening on port {}", sns_port);
        axum::serve(listener, sns_app).await.unwrap();
    });

    let sqs_handle = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", sqs_port))
            .await
            .unwrap();
        println!("SQS service listening on port {}", sqs_port);
        axum::serve(listener, sqs_app).await.unwrap();
    });

    let dynamodb_handle = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", dynamodb_port))
            .await
            .unwrap();
        println!("DynamoDB service listening on port {}", dynamodb_port);
        axum::serve(listener, dynamodb_app).await.unwrap();
    });

    let lambda_handle = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", lambda_port))
            .await
            .unwrap();
        println!("Lambda service listening on port {}", lambda_port);
        axum::serve(listener, lambda_app).await.unwrap();
    });

    let firehose_handle = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", firehose_port))
            .await
            .unwrap();
        println!("Firehose service listening on port {}", firehose_port);
        axum::serve(listener, firehose_app).await.unwrap();
    });

    let memorydb_handle = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", memorydb_port))
            .await
            .unwrap();
        println!("MemoryDB service listening on port {}", memorydb_port);
        axum::serve(listener, memorydb_app).await.unwrap();
    });

    let cognito_handle = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", cognito_port))
            .await
            .unwrap();
        println!("Cognito service listening on port {}", cognito_port);
        axum::serve(listener, cognito_app).await.unwrap();
    });

    let apigateway_handle = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", apigateway_port))
            .await
            .unwrap();
        println!("API Gateway service listening on port {}", apigateway_port);
        axum::serve(listener, apigateway_app).await.unwrap();
    });

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
    }
}
