use std::sync::{Arc, Mutex};


use axum::extract::Request;
use pbkdf2::password_hash::PasswordHasher;
use pbkdf2::{
    password_hash::{PasswordHash, PasswordVerifier, SaltString},
    Pbkdf2,
};
use rand_chacha::{rand_core::OsRng, ChaCha8Rng};
use totp_rs::TOTP;


use crate::database::queries::{get_id_pwd_by_username, Otp};
use crate::utils::AUTH_COOKIE_NAME;
use crate::{
    auth::error::SignupError,
    database::queries::{
            create_session, create_user, get_id_pwd_by_email, get_user_by, Database,
            SessionToken,
        },
};

#[derive(Clone)]
pub struct User {
    pub username: String,
    pub email: Option<String>
}

use super::error::LoginError;
#[derive(Clone)]
pub struct AuthState(pub Option<(SessionToken, Option<User>, Database, Admin)>);
impl AuthState {
    pub fn logged_in(&self) -> bool {
        self.0.is_some()
    }
}
type Admin = bool;
pub type Random = Arc<Mutex<ChaCha8Rng>>;

pub async fn signup(
    mut database: Database,
    username: String,
    email: Option<String>,
    password: String,
) -> Result<(), SignupError> {
    fn valid_username(username: &str) -> bool {
        (1..20).contains(&username.len())
            && username
                .chars()
                .all(|c| matches!(c, 'a' ..='z' | '0'..='9' | '-' |'_' | 'A' ..='z'))
    }

    if !valid_username(&username) {
        return Err(SignupError::InvalidUsername);
    }
    if get_user_by(&mut database, &username)
        .await
        .is_ok_and(|user| user.is_some())
    {
        return Err(SignupError::UserNameTaken);
    } else {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Pbkdf2.hash_password(password.as_bytes(), &salt);
        let hashed_password = if let Ok(password) = password_hash {
            password.to_string()
        } else {
            return Err(SignupError::PasswordError);
        };
        let result = create_user(&mut database, username, hashed_password, email).await;
        let _new_user_id = match result {
            Ok(uid) => uid,

            _ => {
                return Err(SignupError::InternalError);
            }
        };
        Ok(())
    }
}
pub async fn login(
    mut database: Database,
    username: Option<&String>,
    email: Option<String>,
    random: Random,
    password: &String,
) -> Result<SessionToken, LoginError> {

    let row = if username.is_some() {
        get_id_pwd_by_username(&mut database, username.unwrap()).await
    } 
    else {
        get_id_pwd_by_email(&mut database, email.unwrap()).await
    };
    let (uid, hashed_password) = if let Some(row) = row {
        row
    } else {
        return Err(LoginError::UserDoesNotExist);
    };
    let verified_hash = PasswordHash::new(&hashed_password).unwrap();
    if let Err(_err) = Pbkdf2.verify_password(password.as_bytes(), &verified_hash) {
        return Err(LoginError::WrongPassword);
    }
    Ok(new_session(database, random, uid).await)
}
pub async fn new_session(mut database: Database, random: Random, uid: i32) -> SessionToken {
    let session_token = SessionToken::generate_new(random);
    create_session(&mut database, &session_token, uid).await;
    session_token
}
pub fn verify_email(token: String) -> bool {
    let totp= Otp::new();
    totp.check_current(token.as_str()).unwrap()

}
pub async fn get_otp() -> String{
    Otp::generate_new()
}
// **AUTH MIDDLEWARE**
pub async fn auth(
    database: Database,
    mut req: Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let session_tk = req
        .headers()
        .get_all("Cookie")
        .iter()
        .filter_map(|cookie| {
            cookie
                .to_str()
                .ok()
                .and_then(|cookie| cookie.parse::<cookie::Cookie>().ok())
        })
        .find_map(|cookie| {
            let admin = cookie.secure();
            (cookie.name() == AUTH_COOKIE_NAME).then(move || (cookie.value().to_owned(), admin))
        })
        .and_then(|cookie_value| {
            Some((cookie_value.0.parse::<SessionToken>().ok(), cookie_value.1))
        });
    req.extensions_mut().insert(AuthState(
        session_tk.map(|v| (v.0.unwrap(), None, database, v.1.unwrap_or_else(|| false))),
    ));
    next.run(req).await
}