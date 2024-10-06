use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use lettre_email::error;
use serde_json::{json, Value};
use thiserror::Error as Err;

#[derive(Debug, Err)]
pub enum SignupError {
    #[error("Invalid Username")]
    InvalidUsername,
    #[error("Username already Taken")]
    UserNameTaken,
    #[error("Invalid password")]
    PasswordError,
    #[error("Internal Server Error")]
    InternalError,
}
#[derive(Debug, Err)]
pub enum LoginError {
    #[error("User Does not exist")]
    UserDoesNotExist,
    #[error("Wrong Password")]
    WrongPassword,
    #[error("Not Logged in")]
    NotLogging
}
#[derive(Debug, Err)]
pub enum MultipartError {
    #[error("No Named field in multipart")]
    NoName,
    #[error("Invalid value found in multipart")]
    InvalidValue,
    #[error("Reading error")]
    ReadError
}
impl SignupError {
    /// Converts the error to an axum JSON representation.
    pub fn json(&self) -> Json<Value> {
        Json(json!({
            "error": self.to_string()
        }))
    }
}
impl MultipartError {
    pub fn json(&self) -> Json<Value> {
        Json(json!({
            "error": self.to_string()
        }))
    }
}

impl From<MultipartError> for Json<Value> {
    fn from(error: MultipartError) -> Self {
        error.json()
    }
}
impl From<SignupError> for Json<Value> {
    fn from(error: SignupError) -> Self {
        error.json()
    }
}
impl LoginError {
    /// Converts the error to an axum JSON representation.
    pub fn json(&self) -> Json<Value> {
        Json(json!({
            "error": self.to_string()
        }))
    }
}
impl From<LoginError> for Json<Value> {
    fn from(error: LoginError) -> Self {
        error.json()
    }
}
pub fn error_page(e: &dyn std::error::Error) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(format!("error: {}", e))
        .unwrap()
}
