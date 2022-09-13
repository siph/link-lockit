use std::net::SocketAddr;
use std::str::FromStr;
use axum::{
    Server,
    extract::Extension,
    Router,
};
use tower::ServiceBuilder;
use link_lockit::config::Config;
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
        .merge(link_lockit::http::api_router())
        .merge(link_lockit::doc::router())
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

