use anyhow::{Context, Result};
use regex::Regex;
use tokio::net::TcpListener;

use crate::{
    connection::handler::Client,
    http::{request::HttpRequest, response::HttpResponse, HttpMethod, HttpStatus},
};

use std::{collections::HashMap, path::PathBuf, sync::Arc};

type EndpointCall = fn(HttpRequest) -> Result<HttpResponse>;
type EndpointMapping = HashMap<String, HashMap<HttpMethod, EndpointCall>>;

#[derive(Debug, Clone)]
pub(crate) struct ServerContext {
    pub(crate) workdir: Option<PathBuf>,
}

#[derive(Debug)]
pub(crate) struct HttpServer {
    listener: TcpListener,
    workdir: Option<PathBuf>,
    endpoints: EndpointMapping,
}

impl HttpServer {
    pub(crate) async fn new(addr: &str, workdir: Option<String>) -> Result<Self> {
        let listener = TcpListener::bind(addr).await.context("binding server")?;
        let workdir = if let Some(wd) = workdir {
            Some(PathBuf::from(wd))
        } else {
            None
        };

        Ok(Self {
            listener,
            workdir,
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
        let ctx = Arc::new(ServerContext {
            workdir: self.workdir,
        });

        loop {
            let mut client = match self.listener.accept().await {
                Ok((client, _)) => Client::new(client),
                Err(e) => anyhow::bail!("something went wrong: {e}"),
            };

            let endpoints = Arc::clone(&endpoints);
            let ctx = Arc::clone(&ctx);
            tokio::task::spawn(async move {
                let request = client.parse_request(ctx).await.unwrap();
                let response = if let Some(func) = parse_endpoint(endpoints, &request) {
                    func(request).unwrap()
                } else {
                    HttpResponse::new().status(HttpStatus::NotFound)
                };

                client.send_response(response).await
            });
        }
    }
}

fn parse_endpoint(endpoints: Arc<EndpointMapping>, request: &HttpRequest) -> Option<EndpointCall> {
    for (endpoint, funcs) in endpoints.iter() {
        let regex_str = format!("^{endpoint}$");
        // NOTE: always a valid regex, no magic
        let regex = Regex::new(&regex_str).unwrap();
        if !regex.is_match(&request.url) {
            continue;
        }

        match funcs.get(&request.method) {
            Some(func) => return Some(*func),
            None => continue,
        };
    }

    None
}
