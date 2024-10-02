use axum::{response::IntoResponse, Extension};
use http::StatusCode;

use crate::{auth::{authentication::AuthState, error::LoginError}, database::queries::{delete_logged_in, Database}};

pub async fn post_delete_me(
    Extension(mut database): Extension<Database>,
    Extension(authstate): Extension<AuthState>,
) -> impl IntoResponse {
    if authstate.logged_in() {
        let state = authstate.0.unwrap();
        delete_logged_in(&mut database, state.0).await.unwrap();
        StatusCode::OK
    } else {
        return Err(LoginError::NotLogging).unwrap();
    }
}
