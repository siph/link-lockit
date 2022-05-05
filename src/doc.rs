use utoipa::{ OpenApi, openapi };
use entity::links::Model as links_model;
use axum::{
    extract::Path,
    response::IntoResponse,
    Router, 
    routing, 
    Json,
    Extension,
};
use hyper::StatusCode;
use std::sync::Arc;
use utoipa_swagger_ui::Config;

#[derive(OpenApi)]
#[openapi(
    handlers(
        crate::http::links::get_links,
        crate::http::links::process,
    ),
    components(links_model),
    tags(
        (name = "url-wrapper", description = "Wraps urls using external link locker service")
        )
)]
pub struct ApiDoc;

pub fn router() -> Router {
    let api_doc = ApiDoc::openapi();
    let config = Arc::new(Config::from("/api-doc/openapi.json"));
    Router::new()
        .route(
            "/api-doc/openapi.json",
            routing::get({
                let doc = api_doc.clone();
                move || async {Json(doc)}
            })
        )
        .route(
            "/swagger-ui/*tail",
            routing::get(serve_swagger_ui).layer(Extension(config)),
            )
}

async fn serve_swagger_ui(
    Path(tail): Path<String>,
    Extension(state): Extension<Arc<Config<'static>>>,
) -> impl IntoResponse {
    match utoipa_swagger_ui::serve(&tail[1..], state) {
        Ok(file) => file
            .map(|file| {
                (
                    StatusCode::OK,
                    [("Content-Type", file.content_type)],
                    file.bytes,
                )
                    .into_response()
            })
            .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response()),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
