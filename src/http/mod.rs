use axum::Router;

pub mod links;
pub mod exe_io;

pub fn api_router() -> Router {
    links::router()
}
