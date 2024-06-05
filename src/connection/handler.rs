use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::http::{request::HttpRequest, response::HttpResponse};

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

    pub(crate) async fn send_response(&mut self, response: HttpResponse) -> Result<()> {
        let bytes = response.into_bytes()?;
        self.stream.write_all(&bytes).await?;
        Ok(())
    }
}
