use pbkdf2::{password_hash::SaltString, Pbkdf2};
use rand_chacha::rand_core::OsRng;
use pbkdf2::password_hash::PasswordHasher;

use crate::{auth::error::SignupError, database::queries::{get_user_by, Database}};



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
    if get_user_by(&mut database, username).await.is_ok_and(|user| user.is_some()) {
        return Err(SignupError::UserNameTaken);
    }
    else {
        
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Pbkdf2.hash_password(password.as_bytes(), &salt);
        let hashed_password = if let Ok(password) = password_hash {
            password.to_string()
        } else {
            return Err(SignupError::InvalidPassword);
        };
        let result = create_user(&mut database, username.clone(), hashed_password).await;
        let _new_user_id = match result {
            Ok(uid) => uid,

            _ => {
                return Err(SignupError::InternalError);
            }
        };
        Ok(())
    }
}