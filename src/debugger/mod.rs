mod header;
use header::*;
use reqwest::Error;

#[tokio::main]
pub async fn get_ws_url_tokio() -> Result<Vec<WSConfig>, Error> {
    return get_ws_url().await;
}

pub async fn get_ws_url() -> Result<Vec<WSConfig>, Error> {
    let request_url = "http://localhost:9229/json/list";
    let response = reqwest::get(request_url).await?;
    let users: Vec<WSConfig> = response.json().await?;
    Ok(users)
}
