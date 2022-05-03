use anyhow::Result;
use serde::{ Serialize, Deserialize };

/// Returns a result from an exe.io request
/// 
/// # Arguments
///
/// * `api_key` - Access key provided by exe.io
///
/// * `encoded_url` - Url to be processed by exe.io. Must be UTF-8 encoded
pub async fn get_response(
    api_key: &str,
    encoded_url: &str,
) -> Result<ExeIoResponse> {
    let body: String = reqwest::get(
        format!( "https://exe.io/api?api={}&url={}", api_key, encoded_url)
    )
        .await?
        .text()
        .await?;
    let response = serde_json::from_str(&body)?;
    Ok(response)
}

/// Model returned from an exe.io request
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExeIoResponse {
    pub status: String,
    pub shortened_url: String,
    pub message: String,
}
