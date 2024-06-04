use anyhow::{Context, Result};
use regex::Regex;
use request::HttpMethod;
use tokio::net::TcpListener;

use std::collections::HashMap;

mod handler;
mod request;
mod response;
mod utils;

use handler::Client;
use request::HttpRequest;

#[derive(Debug)]
pub(crate) struct HttpServer {
    listener: TcpListener,
    endpoints: HashMap<String, HashMap<HttpMethod, fn(HttpRequest) -> usize>>,
}

impl HttpServer {
    pub(crate) async fn new(addr: &str) -> Result<Self> {
        let listener = TcpListener::bind(addr).await.context("binding server")?;

        Ok(Self {
            listener,
            endpoints: HashMap::new(),
        })
    }

    pub(crate) fn register_endpoint(
        mut self,
        endpoint: impl AsRef<str>,
        method: HttpMethod,
        func: fn(HttpRequest) -> usize,
    ) -> Self {
        let endpoint = endpoint.as_ref().replace("[str]", "(\\w+)");
        let entry = self.endpoints.entry(endpoint).or_insert(HashMap::new());
        entry.insert(method, func);

        self
    }

    pub(crate) fn parse_endpoint(&self, request: HttpRequest) -> Result<()> {
        for (endpoint, funcs) in self.endpoints.iter() {
            let regex = Regex::new(&endpoint)?;
            if !regex.is_match(&request.url) {
                continue;
            }

            match funcs.get(&request.method) {
                Some(func) => {
                    func(request);
                    return Ok(());
                }
                None => 1,
            };
        }

        Ok(())
    }

    pub(crate) async fn serve(&mut self) -> Result<()> {
        loop {
            let mut client = match self.listener.accept().await {
                Ok((client, _)) => Client::new(client),
                Err(e) => anyhow::bail!("something went wrong: {e}"),
            };

            let request = client.parse_request().await?;
            let response = self.parse_endpoint(request)?;
        }
    }
}

fn echo(req: HttpRequest) -> usize {
    0
}

fn root(req: HttpRequest) -> usize {
    0
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut http_server = HttpServer::new("127.0.0.1:4221")
        .await?
        .register_endpoint("/", HttpMethod::Get, root)
        .register_endpoint("/echo/[str]", HttpMethod::Get, echo);

    http_server.serve().await
}
