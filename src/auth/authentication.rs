use std::sync::{Arc, Mutex};

use diesel::row;
use pbkdf2::{password_hash::{PasswordHash, PasswordVerifier, SaltString}, Pbkdf2};
use rand_chacha::{rand_core::OsRng, ChaCha8Rng};
use pbkdf2::password_hash::PasswordHasher;
use rand_core::le;

use crate::{auth::error::SignupError, database::{queries::{create_user, get_id_pwd_by, get_user_by, Database, SessionToken}, schema::session::user_id}};

use super::error::LoginError;

type Random = Arc<Mutex<ChaCha8Rng>>;

pub async fn signup(mut database: Database, username: String, email: Option<String>, password: String) -> Result<(), SignupError>{

    fn valid_username(username: &str) -> bool {
        (1..20).contains(&username.len())
            && username
                .chars()
                .all(|c| matches!(c, 'a' ..='z' | '0'..='9' | '-' |'_' | 'A' ..='z'))
    }

    if !valid_username(&username) {
        return Err(SignupError::InvalidUsername);
    }
    if get_user_by(&mut database, &username).await.is_ok_and(|user| user.is_some()) {
        return Err(SignupError::UserNameTaken);
    }
    else {
        
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
pub async fn login(mut database: Database, username: String, random: Random, password: String) -> Result<SessionToken, LoginError> {
    let row = get_id_pwd_by(&mut database, username).await;
    let (uid, hashed_password) = if let Some(row) = row {
        row
    }else {
        return Err(LoginError::UserDoesNotExist);
    };
    let verified_hash = PasswordHash::new(&hashed_password).unwrap();
    if let Err(err) = Pbkdf2::verify_password(password.as_bytes(), verified_hash) {
        
    }
}