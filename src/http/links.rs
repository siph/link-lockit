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
        delete,
    },
    extract::{
        Query,
        Extension, Path,
    },
    Router, 
};
use super::exe_io::{ 
    ExeIoResponse, 
    get_response,
};
use chrono::offset::Utc;

#[derive(Deserialize)]
pub struct ProcessRequestParameters {
    api_key: String,
    url: String,
}

#[derive(Deserialize)]
pub struct Pagination {
    target_page: usize, 
    page_size: usize, 
}

pub fn router() -> Router {
    Router::new()
        .route("/api/process", post(process))
        .route("/api/links", get(get_links))
        .route("/api/delete", delete(delete_link))
}

#[utoipa::path(
    get,
    path = "/api/links",
    responses(
        (status = 200, description = "Retrieved links succesfully", body = links_model),
        ),
        params(
            ("target_page" = usize, path, description = "Page number"),
            ("page_size" = usize, path, description = "Number of links per page"),
            ),
    )]
pub async fn get_links(
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

#[utoipa::path(
    post,
    path = "/api/process",
    responses(
        (status = 200, description = "Processed link succesfully", body = links_model),
        ),
        params(
            ("api_key" = String, path, description = "Api key provided by exe.io"),
            ("url" = usize, path, description = "Encoded url to be processed"),
            ),
    )]
pub async fn process(
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

#[utoipa::path(
    delete,
    path = "/api/delete",
    responses(
        (status = 200, description = "Deleted link succesfully", body = links_model),
        ),
        params(
            ("id" = String, path, description = "Target link id"),
            ),
    )]
pub async fn delete_link(
    Extension(ref conn): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
) -> String {
    let result = links_entity::delete_by_id(id).exec(conn).await;
    result.is_ok().to_string()
}

async fn get_model(active_model: links::ActiveModel) -> links::Model {
    links::Model {
        links_id: active_model.links_id.unwrap(),
        short: active_model.short.unwrap(),
        original: active_model.original.unwrap(),
        description: active_model.description.unwrap(),
    }
}

