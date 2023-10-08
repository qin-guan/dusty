use axum::{routing::get, Router};
use log::info;
use std::net::SocketAddr;

use dotenvy::{dotenv, var};

mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let app = Router::new().route("/", get(routes::index::handler));

    let port = var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = SocketAddr::from(([127, 0, 0, 1], port.parse().unwrap()));

    info!("API listening on {}", port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
