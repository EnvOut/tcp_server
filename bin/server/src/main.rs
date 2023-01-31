use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use errors::ServerResult;

use crate::routes::{get_quote, AppRoutes};
use crate::server::{run, Server};
use dotenv::dotenv;

mod errors;
mod models;
mod routes;
mod server;
mod services;

#[tokio::main]
async fn main() -> ServerResult<()> {
    dotenv().ok();

    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:8080".to_string());

    let mut routes = AppRoutes::default();
    routes.create_route("quotes", get_quote);
    let server = Server::new(server_address, routes);
    run(server).await
}
