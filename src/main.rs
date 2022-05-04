use std::net::SocketAddr;
use std::str::FromStr;
use axum::{
    Server,
    extract::Extension,
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
    let app = url_wrapper::http::api_router()
        .layer(
            ServiceBuilder::new()
                .layer(Extension(conn))
        );
    let server_url = format!("{}:{}", &config.ip, &config.port);
    let addr = SocketAddr::from_str(&server_url).unwrap();
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

