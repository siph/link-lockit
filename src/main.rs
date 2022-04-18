use anyhow::{Context, Result, Ok};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use url_wrapper::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let config = Config::parse();
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("could not connect to database")?;
    sqlx::migrate!().run(&db).await?;
    Ok(())
}
