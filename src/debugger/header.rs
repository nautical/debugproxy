use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct WSConfig {
    pub id: String,
    pub title: String,
    pub r#type: String,
    pub webSocketDebuggerUrl: String,
    pub url: String,
}
