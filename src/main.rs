use std::net::SocketAddr;
use std::str::FromStr;
use entity::links;
use links::Entity as link_entity;
use sea_orm::{DatabaseConnection, ActiveValue::NotSet, Set, ActiveModelTrait, EntityTrait};
use axum::{
    routing::{
        get,
    },
    Router, 
    Server,
    extract::{
        Extension,
    }
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
        .route("/json", get(get_json))
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

async fn hello(
    Extension(ref conn): Extension<DatabaseConnection>,
) -> String {
    let link = links::ActiveModel {
        links_id: NotSet,
        original: Set("original".to_owned()),
        short: Set("short".to_owned()),
        description: Set("".to_owned()),
    };
    let link = link
        .save(conn)
        .await;
    link.is_ok().to_string()
}

async fn get_json(
    Extension(ref conn): Extension<DatabaseConnection>,
) -> axum::response::Json<entity::links::Model>{
    let link: links::Model = link_entity::find()
        .one(conn)
        .await
        .unwrap()
        .unwrap();
    axum::Json(link)
}




