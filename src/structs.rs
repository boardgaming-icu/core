use rocket::serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct HttpErrResponse {
    error: String
}
impl HttpErrResponse {
    pub fn new (error: String) -> HttpErrResponse { HttpErrResponse { error } }
}

#[derive(Deserialize, Serialize)]
pub struct CaptchaResponse {
        pub success: bool,
        #[serde(rename = "challenge_ts")]
        pub timestamp: String,  // timestamp of the challenge load (ISO format yyyy-MM-dd'T'HH:mm:ssZZ)
        pub hostname: String,         // the hostname of the site where the reCAPTCHA was solved
        #[serde(rename = "error-codes")]
        pub errors: Option<Vec<String>>   
}