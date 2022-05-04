use axum::Router;

mod links;
mod exe_io;

pub fn api_router() -> Router {
    links::router()
}
