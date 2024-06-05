use anyhow::Result;

mod cli;
mod connection;
mod endpoints;
mod http;
mod utils;

use cli::Cli;
use connection::server::HttpServer;
use endpoints::get;
use http::HttpMethod;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let http_server = HttpServer::new("127.0.0.1:4221", cli.directory)
        .await?
        .register_endpoint("/", HttpMethod::Get, get::root)
        .register_endpoint("/echo/[str]", HttpMethod::Get, get::echo)
        .register_endpoint("/user-agent", HttpMethod::Get, get::user_agent);

    http_server.serve().await
}
