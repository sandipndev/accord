use accorde_server::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::run().await
}
