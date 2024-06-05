use anyhow::{Context, Result};
use regex::Regex;
use tokio::net::TcpListener;

use crate::{
    connection::handler::Client,
    http::{request::HttpRequest, response::HttpResponse, HttpMethod, HttpStatus},
};

use std::{collections::HashMap, path::PathBuf, sync::Arc};

type EndpointMapping =
    HashMap<String, HashMap<HttpMethod, fn(HttpRequest) -> Result<HttpResponse>>>;

#[derive(Debug, Clone)]
pub(crate) struct ServerContext<T: Clone> {
    inner: HashMap<String, T>,
}

impl<T> ServerContext<T>
where
    T: Clone,
{
    pub(crate) fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub(crate) fn add_context(&mut self, key: impl AsRef<str>, value: &T) {
        self.inner.insert(key.as_ref().to_string(), value.clone());
    }
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
        let mut ctx = ServerContext::new();
        ctx.add_context("workdir", &self.workdir);
        let ctx = Arc::new(ctx);

        loop {
            let mut client = match self.listener.accept().await {
                Ok((client, _)) => Client::new(client),
                Err(e) => anyhow::bail!("something went wrong: {e}"),
            };

            let ctx = Arc::clone(&ctx);
            let endpoints = Arc::clone(&endpoints);
            tokio::task::spawn(async move {
                let request = client.parse_request().await.unwrap();
                let response = parse_endpoint(endpoints, request).unwrap();

                client.send_response(response).await
            });
        }
    }
}

fn parse_endpoint(endpoints: Arc<EndpointMapping>, request: HttpRequest) -> Result<HttpResponse> {
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

    let not_found = HttpResponse::new().status(HttpStatus::NotFound);
    Ok(not_found)
}
