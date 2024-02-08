use serde::{Deserialize, Serialize};

use errors::APILayerError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APIResponse {
    message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BadWord {
    original: String,
    word: String,
    deviations: i64,
    info: i64,
    #[serde(rename = "replacedLen")]
    replaced_len: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BadWordsResponse {
    pub content: String,
    pub bad_words_total: i64,
    pub bad_words_list: Vec<BadWord>,
    pub censored_content: String,
}

pub async fn create_api_layer_error(
    res: reqwest::Response,
) -> APILayerError {
    APILayerError {
        status: res.status().as_u16(),
        message: res.json::<APIResponse>().await.unwrap().message,
    }
}
