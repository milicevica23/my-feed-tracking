#![allow(unused)]

mod config;
mod crypt;
mod error;
//mod log;
mod ctx;
mod model;
mod utils;
mod web;
use std::net::SocketAddr;

use crate::model::ModelManager;
use axum::{response::Html, routing::get, Router};
use tracing::info;
use tracing_subscriber::EnvFilter;
pub mod _dev_utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .without_time() // For early local development.
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    _dev_utils::init_dev().await;
    let mm = ModelManager::new();

    let router_all = Router::new().route("/hello", get(|| async { Html("Hello world") }));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("->> LISTENING on {addr}");

    axum::Server::bind(&addr)
        .serve(router_all.into_make_service())
        .await
        .unwrap()
}
