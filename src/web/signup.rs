use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Serialize;

use crate::{
    auth::{authentication::signup, error::error_page},
    database::queries::Database,
};
#[derive(Debug, Serialize)]
pub struct SignupPayload {
    username: String,
    password: String,
    email: Option<String>,
}

pub async fn post_signup(
    Extension(database): Extension<Database>,
    Json(signup_payload): Json<SignupPayload>,
) -> impl IntoResponse {
    match signup(
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
