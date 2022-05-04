use anyhow::Result;
use serde::Deserialize;
use entity::links::{
    Entity as links_entity, 
    self,
};
use sea_orm::{
    DatabaseConnection, 
    ActiveValue::NotSet, 
    Set, 
    ActiveModelTrait, 
    EntityTrait, 
    QueryOrder, 
    PaginatorTrait, 
};
use axum::{ 
    routing::{ 
        post, 
        get,
    },
    http::StatusCode,
    response::{
        IntoResponse,
        Response,
    },
    extract::{
        Query,
        Extension,
    },
    body::Full,
    Router, 
};
use super::exe_io::{ 
    ExeIoResponse, 
    get_response,
};
use chrono::offset::Utc;

#[derive(Deserialize)]
struct ProcessRequestParameters {
    api_key: String,
    url: String,
}

#[derive(Deserialize)]
struct Pagination {
    target_page: usize, 
    page_size: usize, 
}

pub fn router() -> Router {
    Router::new()
        .route("/api/process", post(process))
        .route("/api/links", get(get_links))
}

async fn get_links(
    Extension(ref conn): Extension<DatabaseConnection>,
    pagination: Query<Pagination>,
) -> axum::response::Json<Vec<entity::links::Model>> {
    let links: Vec<entity::links::Model> = links_entity::find()
        .order_by_asc(links::Column::LinksId)
        .paginate(conn, pagination.0.page_size)
        .fetch_page(pagination.0.target_page - 1)
        .await
        .unwrap_or(vec![]);
    axum::Json(links)
}

async fn process(
    Extension(ref conn): Extension<DatabaseConnection>,
    request_parameters: Query<ProcessRequestParameters >,
) -> axum::response::Json<links::Model> {
    let api_key = request_parameters.0.api_key;
    let url = request_parameters.0.url;
    let response: ExeIoResponse = get_response(&api_key, &url)
        .await
        .unwrap();
    let link = links::ActiveModel {
        links_id: NotSet,
        original: Set(url),
        short: Set(response.shortened_url.clone()),
        description: Set(Utc::now().to_rfc3339()),
    };
    let link = link.save(conn).await.unwrap();
    axum::Json(get_model(link).await)
}

async fn get_model(active_model: links::ActiveModel) -> links::Model {
    links::Model {
        links_id: active_model.links_id.unwrap(),
        short: active_model.short.unwrap(),
        original: active_model.original.unwrap(),
        description: active_model.description.unwrap(),
    }
}

