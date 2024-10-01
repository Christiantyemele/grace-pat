use axum::{response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

use crate::{auth::{authentication::{login, Random}, error::error_page}, database::queries::Database, utils::login_response};
#[derive(Clone, Serialize, Deserialize)]
pub struct LoginPayload {
 //   #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
 //   #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    password: String
}

pub async fn post_login(
    Extension(cookie): Extension<Cookies>,
    Extension(database): Extension<Database>,
    Extension(random): Extension<Random>,
    Json(login_payload): Json<LoginPayload>,
) -> impl IntoResponse {
    match login(
        database,
        login_payload.username,
        login_payload.email,
        random,
        login_payload.password,
    )
    .await
    {
        Ok(session_tk) => Ok(login_response(cookie, session_tk).await),
        Err(err) => Err(error_page(&err)),
    }
}