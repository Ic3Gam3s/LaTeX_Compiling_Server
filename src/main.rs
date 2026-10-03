pub mod FileHandling;
pub mod compiling;
pub mod health;

use crate::FileHandling::ZipHandler::ZipHandler;
use crate::compiling::CompilingHandler::CompilingHandler;
use axum::{
    Router,
    body::Body,
    extract::{Multipart, Query},
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::Deserialize;
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
};
use tower_http::cors::CorsLayer;

async fn notImplementedString() -> &'static str {
    return "Not Implemented";
}
const SERVER_ADDRESS: &str = "0.0.0.0:8000";

#[tokio::main]
async fn main() {
    // Initialize uploads directory
    fs::create_dir_all("./uploads/temp").unwrap();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST]);

    // Init Handlers
    let zip_handler = ZipHandler::new();
    let compiling_handler = CompilingHandler::new(zip_handler);

    let compileRouter = Router::new()
        .route("/compile", post(CompilingHandler::compile))
        .with_state(compiling_handler);

    let healthRouter = Router::new().route("/", get(notImplementedString));

    let app = Router::new().merge(compileRouter).merge(healthRouter);

    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind(SERVER_ADDRESS)
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await;
}
