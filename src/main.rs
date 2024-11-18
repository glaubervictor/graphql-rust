mod graphql;
mod repository;
mod server;
mod auth;

use dotenv::dotenv;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL: Variável de ambiente não definida".to_string())
        .unwrap();

    let app = server::create_app(database_url).await;
    println!("🚀 Server running at http://localhost:3000");

    axum::serve(TcpListener::bind(("0.0.0.0", 3000)).await.unwrap(), app)
        .await
        .unwrap()
}
