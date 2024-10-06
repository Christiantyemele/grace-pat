use axum::{extract::Multipart, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};

use crate::{
    auth::{authentication::signup, error::error_page},
    database::queries::Database,
};

use super::everify::EmailOtp;
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SignupPayload {
    username: String,
    password: String,
    email: String,
}
#[axum::debug_handler]
pub async fn post_signup(
    Extension(database): Extension<Database>,
    Json(signup_payload): Json<SignupPayload>,
    rotp: Extension<EmailOtp>,
    lotp: Multipart
) -> impl IntoResponse {
   
    match signup(
        rotp,
        lotp,
        database,
        signup_payload.username,
        signup_payload.email,
        signup_payload.password,
    )
    .await
    {
        Ok(_) => Ok(StatusCode::ACCEPTED),
        Err(e) => Err(error_page(&e)),
    }
}
