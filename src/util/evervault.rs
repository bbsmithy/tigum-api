use reqwest;
use std::collections::hash_map::HashMap;

pub async fn send_evervault_verify_email(encrypted_email: String) {
    let mut map = HashMap::new();
    map.insert("to", encrypted_email);
    map.insert("type", "verify".to_string());
    let client = reqwest::Client::new();
    let res = client.post("https://cage.run/tigum-signup-cage")
    .header("content-type", "application/json")
    .header("api-key", "Mzg0:5yYuebamwUvq14PCzNowT4wzJtMVHTCsiZj26oqDQqNLx1VMQz5mtunt97YFkk8se")
    .json(&map)
    .send().await;
}
