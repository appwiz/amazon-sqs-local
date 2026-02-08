use std::sync::Arc;

use clap::Parser;

mod error;
mod queue;
mod server;
mod state;
mod types;

use state::SqsState;

#[derive(Parser)]
#[command(name = "aws-sqs-local", about = "Local Amazon SQS service")]
struct Args {
    #[arg(long, default_value = "9324")]
    port: u16,
    #[arg(long, default_value = "us-east-1")]
    region: String,
    #[arg(long, default_value = "000000000000")]
    account_id: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let state = Arc::new(SqsState::new(
        args.account_id,
        args.region,
        args.port,
    ));
    let app = server::create_router(state);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port))
        .await
        .unwrap();
    println!("aws-sqs-local listening on port {}", args.port);
    axum::serve(listener, app).await.unwrap();
}
