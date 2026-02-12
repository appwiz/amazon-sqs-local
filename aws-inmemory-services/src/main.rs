use std::sync::Arc;

use clap::Parser;

mod s3;
mod sns;
mod sqs;

#[derive(Parser)]
#[command(name = "aws-inmemory-services", about = "Local AWS S3, SNS, and SQS services")]
struct Args {
    #[arg(long, default_value = "9000")]
    s3_port: u16,
    #[arg(long, default_value = "9911")]
    sns_port: u16,
    #[arg(long, default_value = "9324")]
    sqs_port: u16,
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

    let s3_app = s3::server::create_router(s3_state);
    let sns_app = sns::server::create_router(sns_state);
    let sqs_app = sqs::server::create_router(sqs_state);

    let s3_port = args.s3_port;
    let sns_port = args.sns_port;
    let sqs_port = args.sqs_port;

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

    tokio::select! {
        r = s3_handle => r.unwrap(),
        r = sns_handle => r.unwrap(),
        r = sqs_handle => r.unwrap(),
    }
}
