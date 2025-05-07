use std::env;

use dotenvy::dotenv;
use server::Server;

mod core;
mod health;
mod images;
mod server;

#[tokio::main]
async fn main() {
    dotenv().unwrap_or_default();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // Initialize tracing subscriber when is debug mode
    if cfg!(debug_assertions) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }

    let server = Server::new().await;

    server.run(&addr).await;
}
