use reqwest;

pub fn send_evervault_verify_email() {
    let body: &str = "{'to': 'briansmith.work578@gmail.com','type': 'verify'}";
    let client = reqwest::blocking::Client::new();
    let res = client.post("https://cage.run/tigum-signup-cage")
    .header("content-type", "application/json")
    .header("api-key", "Mzg0:5yYuebamwUvq14PCzNowT4wzJtMVHTCsiZj26oqDQqNLx1VMQz5mtunt97YFkk8se")
    .body(body)
    .send();
    println!("{:?}", res);
}
