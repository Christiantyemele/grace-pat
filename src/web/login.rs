use std::borrow::Borrow;

use axum::{response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

use crate::{auth::{authentication::{login, Random}, error::error_page}, database::queries::Database, utils::login_response};
#[derive(Clone, Serialize, Deserialize)]
pub struct LoginPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    password: String
}

pub async fn post_login(
    Extension(cookie): Extension<Cookies>,
    Extension(database): Extension<Database>,
    Extension(random): Extension<Random>,
    Json(login_payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let username = &login_payload.username.unwrap();
    let password = &login_payload.password;
    match login(
        database,
        Some(username),// todo fix
        login_payload.email,
        random,
        password,
    )
    .await
    {
        Ok(session_tk) => Ok(login_response(cookie, session_tk, username, password).await),
        Err(err) => Err(error_page(&err)),
    }
}