use std::collections::HashMap;

use axum::response::{IntoResponse, Response};
use cookie::Cookie;
use http::StatusCode;
use tower_cookies::Cookies;
use axum::extract::Multipart;
use crate::{auth::error::MultipartError, database::queries::SessionToken};
pub const AUTH_COOKIE_NAME: &str = "auth_token";


pub async fn login_response(
    jar: Cookies,
    session_tk: SessionToken,
    usrname: &String,
    pwd: &String,
) -> impl IntoResponse {
    let cookie = Cookie::build((AUTH_COOKIE_NAME, session_tk.into_cookie_value()))
        .path("/")
        .secure(is_admin(usrname, pwd))
        .build();
    jar.add(cookie)
}
pub async fn logout_response() -> impl IntoResponse {
    let remove = format!("{}=_; Max-Age=0", AUTH_COOKIE_NAME);
    Cookie::parse(remove).unwrap();
    StatusCode::OK
}
pub fn is_admin(username: &String, password: &String) -> bool {
    if username == &"grace-cati".to_owned() && password == &"Christian".to_owned() {
        true
    } else {
        false
    }
}
pub async fn parse_multipart(mut multipart: Multipart) -> Result<HashMap<String,String>, Response>{
    let mut map = HashMap::new();
    while let Some(field) = multipart.next_field().await.map_err(|_| MultipartError::NoName.json().into_response())? {
        let name = field.name().ok_or(MultipartError::ReadError.json().into_response())?.to_string();
        let data = field.text().await.map_err(|_| MultipartError::InvalidValue.json().into_response())?;
        map.insert(name, data);
        
    }
Ok(map)
}