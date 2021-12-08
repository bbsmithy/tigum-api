use reqwest;
use rocket_contrib::json::{JsonValue};

const EM_KEY: &str = "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";

fn create_email_body(subject: String, content: String) -> JsonValue {
    let email_body = json!({
        "personalizations": [{
            "to": [{"email": "briansmith.work578@gmail.com"}]
        }],
        "from": { "email": "brian@tigum.io" },
        "subject": subject,
        "content": [{ "type": "text/plain", "value": content }]
    });
    return email_body;
}

pub fn send_beta_signup_email_notify(beta_user_email: String, beta_user_name: String) {
    let user_info = format!("Email: {0} User name: {1}", beta_user_email, beta_user_name);
    let email_body = create_email_body("Beta Signup Requested!".to_string(), user_info);
    let client = reqwest::blocking::Client::new();
    let _res = client.post("https://api.sendgrid.com/v3/mail/send")
    .header("content-type", "application/json")
    .header("Authorization", format!("Bearer {}", EM_KEY))
    .json(&email_body)
    .send();
}

pub fn send_user_feedback(user_id: i32, user_name: String, feedback: &String) {
    let user_info = format!("User ID: {0}, User name: {1}, Feedback: {2}", user_id, user_name, feedback);
    let email_body = create_email_body("User feedback received".to_string(), user_info);
    let client = reqwest::blocking::Client::new();
    let _res = client.post("https://api.sendgrid.com/v3/mail/send")
    .header("content-type", "application/json")
    .header("Authorization", format!("Bearer {}", EM_KEY))
    .json(&email_body)
    .send();
}
