use anyhow::{Context, Result};
use tokio::net::TcpListener;

mod handler;
mod request;

use handler::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221")
        .await
        .context("binding server")?;

    loop {
        let mut client = match listener.accept().await {
            Ok((client, _)) => Client::new(client),
            Err(e) => anyhow::bail!("something went wrong: {e}"),
        };

        let request = client.parse_request().await?;
        match request.line.url.as_str() {
            "/" => {
                client
                    .write_response_raw(b"HTTP/1.1 200 OK\r\n\r\n")
                    .await?
            }
            _ => {
                client
                    .write_response_raw(b"HTTP/1.1 404 Not Found\r\n\r\n")
                    .await?
            }
        }
    }
}
