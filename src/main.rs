use anyhow::{Context, Result};
use regex::Regex;
use tokio::net::TcpListener;

use std::collections::HashMap;

mod endpoints;
mod handler;
mod http;
mod utils;

use endpoints::get;
use handler::Client;
use http::{request::HttpRequest, response::HttpResponse, HttpMethod};

#[derive(Debug)]
pub(crate) struct HttpServer {
    listener: TcpListener,
    endpoints: HashMap<String, HashMap<HttpMethod, fn(HttpRequest) -> Result<HttpResponse>>>,
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
        func: fn(HttpRequest) -> Result<HttpResponse>,
    ) -> Self {
        let endpoint = endpoint.as_ref().replace("[str]", "(\\w+)");
        let entry = self.endpoints.entry(endpoint).or_insert(HashMap::new());
        entry.insert(method, func);

        self
    }

    pub(crate) fn parse_endpoint(&self, request: HttpRequest) -> Result<HttpResponse> {
        for (endpoint, funcs) in self.endpoints.iter() {
            let regex_str = format!("^{endpoint}$");
            let regex = Regex::new(&regex_str)?;
            if !regex.is_match(&request.url) {
                continue;
            }

            match funcs.get(&request.method) {
                Some(func) => return func(request),
                None => continue,
            };
        }

        let not_found = HttpResponse::new().status(http::HttpStatus::NotFound);
        Ok(not_found)
    }

    pub(crate) async fn serve(&mut self) -> Result<()> {
        loop {
            let mut client = match self.listener.accept().await {
                Ok((client, _)) => Client::new(client),
                Err(e) => anyhow::bail!("something went wrong: {e}"),
            };

            let request = client.parse_request().await?;
            let response = self.parse_endpoint(request)?;
            client.send_response(response).await?;
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut http_server = HttpServer::new("127.0.0.1:4221")
        .await?
        .register_endpoint("/", HttpMethod::Get, get::root)
        .register_endpoint("/echo/[str]", HttpMethod::Get, get::echo);

    http_server.serve().await
}
