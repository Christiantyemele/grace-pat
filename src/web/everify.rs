use axum::{routing::get, Json};

use crate::auth::authentication::{get_otp, verify_email};

pub async fn post_veriy_email(Json(otp): Json<String>) {
    verify_email(otp);
}
pub async fn view_otp() -> String {
    get_otp().await 
}