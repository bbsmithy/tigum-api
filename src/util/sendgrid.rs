use reqwest;

const EM_KEY: &str = "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";

pub fn send_beta_signup_email_notify(beta_user_email: String, beta_user_name: String) {
    let user_info = format!("Email: {0} User name: {1}", beta_user_email, beta_user_name);
    let email_body = json!({
        "personalizations": [{
            "to": [{"email": "briansmith.work578@gmail.com"}]
        }],
        "from": { "email": "brian@tigum.io" },
        "subject": "Beta User Sign Up!",
        "content": [{ "type": "text/plain", "value": user_info }]
    });
    let client = reqwest::blocking::Client::new();
    let _res = client.post("https://api.sendgrid.com/v3/mail/send")
    .header("content-type", "application/json")
    .header("Authorization", format!("Bearer {}", EM_KEY))
    .json(&email_body)
    .send();
}
