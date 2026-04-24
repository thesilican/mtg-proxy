use anyhow::Result;
use backend::init_logger;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger();
    Ok(())
}
