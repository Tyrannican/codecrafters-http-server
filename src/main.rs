use anyhow::{Context, Result};
use tokio::{io::AsyncWriteExt, net::TcpListener};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221")
        .await
        .context("binding server")?;

    loop {
        let (mut client, _) = match listener.accept().await {
            Ok(client) => client,
            Err(e) => anyhow::bail!("something went wrong: {e}"),
        };

        client.write(b"HTTP/1.1 200 OK\r\n\r\n").await?;
    }
}
