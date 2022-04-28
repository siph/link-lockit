use std::net::SocketAddr;
use std::str::FromStr;

use axum::{
    routing::{
        get,
    },
    Router, 
    AddExtensionLayer, 
    Server,
};
use tower::ServiceBuilder;
use url_wrapper::config::Config;
use clap::Parser;
use sea_orm::Database;
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let config = Config::parse();
    let conn = Database::connect(&config.database_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();
    let app = Router::new()
        .route("/hello", get(hello))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(conn))
        );
    let server_url = format!("{}:{}", &config.ip, &config.port);
    let addr = SocketAddr::from_str(&server_url).unwrap();
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn hello() -> String{
    String::from("hello")
}
