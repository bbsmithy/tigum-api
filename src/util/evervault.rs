use reqwest;
use std::collections::hash_map::HashMap;

pub fn send_evervault_verify_email(encrypted_email: String, verify_hash: String, user_name: String) {
    let mut map = HashMap::new();
    map.insert("to", encrypted_email);
    map.insert("type", "verify".to_string());
    map.insert("verify_hash", verify_hash);
    map.insert("user_name", user_name);
    let client = reqwest::blocking::Client::new();
    let _res = client.post("https://cage.run/tigum-signup-cage")
    .header("content-type", "application/json")
    .header("api-key", "Mzg0:5yYuebamwUvq14PCzNowT4wzJtMVHTCsiZj26oqDQqNLx1VMQz5mtunt97YFkk8se")
    .json(&map)
    .send();
}
