use axum::{
    routing::any,
    Router,
};
use reqwest::Client;
use std::net::SocketAddr;

use crate::handler::proxy_handler;
mod handler;

const DBG_MODE: bool = true;

#[tokio::main]
async fn main() {
    let client = Client::new(); // for NestJS

    let app = Router::new()
        .route("/{*path}", any(proxy_handler)) // Catch every route
        .with_state(client);

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("\n\t󰞀 Fanouni Security Guard running on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

