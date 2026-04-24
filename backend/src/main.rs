use backend::{init_logger, run};

#[tokio::main]
async fn main() {
    init_logger();
    if let Err(err) = run().await {
        eprintln!("Server exited unexpectedly: {err}")
    }
}
