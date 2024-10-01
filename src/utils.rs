use axum::response::IntoResponse;
use cookie::Cookie;
use http::StatusCode;
use tower_cookies::Cookies;

use crate::database::queries::SessionToken;
pub const AUTH_COOKIE_NAME: &str = "auth_token";


pub async fn login_response(cookie: Cookies, session_tk: SessionToken) -> impl IntoResponse {
    cookie.add(Cookie::new(
        AUTH_COOKIE_NAME,
        session_tk.into_cookie_value(),
    ))
}
pub async fn logout_response() -> impl IntoResponse {
    let remove = format!("{}=_; Max-Age=0", AUTH_COOKIE_NAME);
    Cookie::parse(remove).unwrap();
    StatusCode::OK
}
