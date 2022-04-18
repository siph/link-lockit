use axum::Router;

pub fn router() -> Router{
    Router::new()
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    original_link: String,
    short_link: Option<String>,
    description: Option<String>,
}
