use anyhow::{Context, Result};
use regex::Regex;
use tokio::net::TcpListener;

use std::{collections::HashMap, sync::Arc};

mod endpoints;
mod handler;
mod http;
mod utils;

use endpoints::get;
use handler::Client;
use http::{request::HttpRequest, response::HttpResponse, HttpMethod};

type EndpointMapping =
    HashMap<String, HashMap<HttpMethod, fn(HttpRequest) -> Result<HttpResponse>>>;

#[derive(Debug)]
pub(crate) struct HttpServer {
    listener: TcpListener,
    endpoints: EndpointMapping,
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

    pub(crate) async fn serve(self) -> Result<()> {
        let endpoints = Arc::new(self.endpoints);
        loop {
            let mut client = match self.listener.accept().await {
                Ok((client, _)) => Client::new(client),
                Err(e) => anyhow::bail!("something went wrong: {e}"),
            };

            let endpoints = Arc::clone(&endpoints);
            tokio::task::spawn(async move {
                let request = client.parse_request().await.unwrap();
                let response = parse_endpoint(endpoints, request).unwrap();
                client.send_response(response).await
            });
        }
    }
}

pub(crate) fn parse_endpoint(
    endpoints: Arc<EndpointMapping>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    for (endpoint, funcs) in endpoints.iter() {
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

#[tokio::main]
async fn main() -> Result<()> {
    let http_server = HttpServer::new("127.0.0.1:4221")
        .await?
        .register_endpoint("/", HttpMethod::Get, get::root)
        .register_endpoint("/echo/[str]", HttpMethod::Get, get::echo)
        .register_endpoint("/user-agent", HttpMethod::Get, get::user_agent);

    http_server.serve().await
}
