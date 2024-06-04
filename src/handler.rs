use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::request::HttpRequest;

const BUF_SIZE: usize = 4096;

#[derive(Debug)]
pub(crate) struct Client {
    stream: TcpStream,
}

impl Client {
    pub(crate) fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub(crate) async fn parse_request(&mut self) -> Result<HttpRequest> {
        let mut buffer = [0; BUF_SIZE];
        let n = self.stream.read(&mut buffer).await?;
        let buffer = &buffer[..n];
        Ok(HttpRequest::new(buffer)?)
    }

    pub(crate) async fn send_response(&mut self) -> Result<()> {
        Ok(())
    }

    pub(crate) async fn write_response_raw(&mut self, resp: &[u8]) -> Result<()> {
        self.stream.write(resp).await?;
        Ok(())
    }
}
