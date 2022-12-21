use crate::structs::CaptchaResponse;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use reqwest::Client;
use serde_json::from_str;
use uuid::Uuid;

pub fn send_mail(to: String, subject: String, mail: String) -> Result<String, String> {
    let email = Message::builder()
        .from(
            format!("{} <{}>", get_env("MAILFROM"), get_env("MAILADDR"))
                .parse()
                .unwrap(),
        )
        .to(to.parse().unwrap())
        .subject(subject)
        .body(mail)
        .unwrap();

    let creds = Credentials::new(get_env("MAILUSER"), get_env("MAILPASS"));
    let mailer = SmtpTransport::relay(&get_env("MAILADDR"))
        .unwrap()
        .credentials(creds)
        .build();

    // Find a better way to send results, other than Ok and Err.
    match mailer.send(&email) {
        Ok(_) => {
            return Ok("Success".to_string());
        }
        Err(_) => {
            return Err("Failure".to_string());
        }
    }
}

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn get_env(key: &str) -> String {
    std::env::var(key).unwrap()
}

pub async fn check_captcha(key: String) -> Option<bool> {
    let client = Client::new();

    // Set up the request parameters
    let params = [("secret", get_env("privkey")), ("response", key)];

    // Send the request to the reCAPTCHA server to validate the token
    let response = client
        .post("https://www.google.com/recaptcha/api/siteverify")
        .form(&params)
        .send()
        .await;

    match response {
        Err(_) => {
            return None;
        }
        Ok(res) => {
            match from_str::<CaptchaResponse>(&res.text().await.unwrap()) {
                Ok(result) => {
                    if result.success {
                        return Some(true)
                    } else {
                        return Some(false)
                    };
                }
                Err(_) => {
                    return None;
                }
            }
        }
    }
}
