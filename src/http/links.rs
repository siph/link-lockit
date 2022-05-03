use anyhow::Result;
use serde::Deserialize;
use axum::{ 
    routing::post,
    extract::Query,
    http::StatusCode,
    response::{
        IntoResponse,
        Response,
    },
    body::Full,
    Router, 
};
use super::exe_io::{ 
    ExeIoResponse, 
    get_response,
};

#[derive(Deserialize)]
struct RequestParameters {
    api_key: String,
    url: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/api/process", post(process))
}

async fn process(
    request_parameters: Query<RequestParameters>,
) -> impl IntoResponse {
    let response: Result<ExeIoResponse> = get_response(&request_parameters.api_key, &request_parameters.url)
        .await;
    match response {
        // TODO: figure out how to return the response without axum stopping me at every possible opportunity
        Ok(_response) => {
            Response::builder()
                .status(StatusCode::OK)
                .body(Full::from("ok"))
                .unwrap()
        },
        Err(_) => {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from(""))
                .unwrap()
        },
    }
}
