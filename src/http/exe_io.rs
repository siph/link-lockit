use anyhow::Result;

/// Returns a result from an exe.io request
/// 
/// # Arguments
///
/// * 'api_key' - Access key provided by exe.io
///
/// * 'encoded_url' - Url to be processed by exe.io. Must be UTF-8 encoded
pub async fn get_response(
    api_key: String,
    encoded_url: String,
) -> Result<Response> {
    let body: String = reqwest::get(
        format!( "https://exe.io/api?api={}&url={}", &api_key, &encoded_url)
    )
        .await?
        .text()
        .await?;
    let response = serde_json::from_str(&body)?;
    Ok(response)
}

/// Model returned from an exe.io request
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub status: String,
    pub shortened_url: String,
    pub message: String,
}
