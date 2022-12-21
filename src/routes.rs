use crate::{helpers, structs};
use rocket::{http::Status, serde::json::Json};

#[post("/signup")]
pub fn signup() {

}

#[get("/signin")]
pub fn signin() {

}

#[get("/request_verification?<to>&<captchakey>")]
pub async fn req_verif_code(to: String, captchakey: String) -> Result<Status, (Status, Json<structs::HttpErrResponse>)> {
    match crate::helpers::check_captcha(captchakey).await {
        Some(t) => {
            if t == true {
                return Err((Status::Forbidden, Json(structs::HttpErrResponse::new("Captcha invalid".to_string()))))
            } else {
                let verif_code = crate::helpers::generate_uuid();
                crate::helpers::send_mail(to, "Your boardgaming.icu verification code".to_string(), format!("
                    Thank you for signing up to {}
                    To verify your account - please click the link below
                    {}/request_verification?code={}
                ", helpers::get_env("SITENAME"), helpers::get_env("SITEADDR"), &verif_code)).unwrap();
                return Ok(Status::NoContent)
            }
        },
        None => {
            return Err((Status::Forbidden, Json(structs::HttpErrResponse::new("No captcha code".to_string()))))
        }
    }
}

#[get("/request_verification?<code>")]
pub fn res_verif_code(code: String) {
    
    // check DB and then update
}