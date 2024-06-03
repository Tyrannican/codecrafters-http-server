use anyhow::{Context, Result};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221")
        .await
        .context("binding server")?;

    loop {
        let _client = match listener.accept().await {
            Ok(client) => client,
            Err(e) => anyhow::bail!("something went wrong: {e}"),
        };
    }
}
