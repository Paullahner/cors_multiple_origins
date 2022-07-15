use std::net::SocketAddr;

use axum::{http::HeaderValue, routing::get, Router, Server};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    //start_with_single_origin().await;
    start_with_multiple_origins().await;
}

async fn root() -> &'static str {
    "hello World"
}

async fn start_with_multiple_origins() {
    let origins = [
        "*".parse().unwrap(),
        "http://localhost:3000".parse().unwrap(),
    ];

    let cors = CorsLayer::new().allow_origin(origins);
    let router = Router::new().route("/", get(root)).layer(cors);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Server::bind(&address)
        .serve(router.into_make_service())
        .await;

    match app {
        Ok(app) => app,
        Err(_) => std::process::exit(1),
    }
}

#[allow(dead_code)]
async fn start_with_single_origin() {
    let cors = CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap());
    let router = Router::new().route("/", get(root)).layer(cors);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Server::bind(&address)
        .serve(router.into_make_service())
        .await;

    match app {
        Ok(app) => app,
        Err(_) => std::process::exit(1),
    }
}
