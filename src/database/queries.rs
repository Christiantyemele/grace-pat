use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use crate::database::{
    model::{NewUser, Users},
    schema,
};
use diesel::prelude::*;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};
use rand_chacha::{rand_core::RngCore, ChaCha8Rng};
use totp_rs::{Algorithm, TOTP, Secret};

use super::{error::DatabaseError, model::Session};

pub type Database = Pool<AsyncPgConnection>;
type Random = Arc<Mutex<ChaCha8Rng>>;
pub struct Otp(u16);
impl FromStr for Otp {
    type Err = <u16 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}
impl Otp {
    fn into_database_value(self) -> i32 {
        self.0 as i32
    }
    fn generate_new(self) -> String {
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            20,
            Secret::Encoded("KRSXG5CTMVRXEZLUKN2XAZLSKNSWG4TFOQ".to_string()).to_bytes().unwrap(),
        ).unwrap();
       totp.generate_current().unwrap()
    }
}
pub struct SessionToken(u128);
impl FromStr for SessionToken {
    type Err = <u128 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}
impl SessionToken {
    pub fn into_cookie_value(self) -> String {

        self.0.to_string()
    }
    pub fn into_database_value(&self) -> Vec<u8> {

        self.0.to_be_bytes().to_vec()
    }
    pub fn generate_new(random: Random) -> Self {

        let mut u128_pool = [0u8; 16];
        random.lock().unwrap().fill_bytes(&mut u128_pool);
        Self(u128::from_le_bytes(u128_pool))
    }
}
pub async fn create_user(
    conn: &mut Database,
    username: String,
    passkey: String,
    email: Option<String>,
) -> Result<i32, diesel::result::Error> {
    let mut conn = conn.get().await.unwrap();

    let new_user = NewUser {
        username,
        passkey,
        email,
    };
    let result: Result<i32, diesel::result::Error> = diesel::insert_into(schema::users::table)
        .values(&new_user)
        .returning(schema::users::id)
        .get_result::<i32>(&mut *conn)
        .await;
    result
}
pub async fn delete_user_by(
    conn: &mut Database,
    usernam: String,
) -> Result<(), diesel::result::Error> {
    let mut conn = conn.get().await.unwrap();
    use schema::users::dsl::*;
    diesel::delete(users.filter(username.like(usernam)))
        .execute(&mut conn)
        .await?;

    Ok(())
}
pub async fn get_user_by(
    conn: &mut Database,
    usernam: &String,
) -> Result<Option<Users>, diesel::result::Error> {
    use schema::users::dsl::*;
    let mut conn = conn.get().await.unwrap();
    let result = users
        .filter(username.eq(usernam))
        .select(Users::as_select())
        .first(&mut *conn)
        .await?;

    Ok(Some(result))
}
pub async fn get_all_users(conn: &mut Database) -> Result<Vec<Users>, diesel::result::Error> {
    let mut conn = conn.get().await.unwrap();
    use schema::users::dsl::*;
    let all_users = users
        .select(Users::as_select())
        .get_results(&mut *conn)
        .await?;

    Ok(all_users)
}
pub async fn create_session(conn: &mut Database, token: &SessionToken, uid: i32) -> Vec<u8> {
    let mut conn = conn.get().await.unwrap();
    let new_session = Session {
        user_id: uid,
        session_token: token.into_database_value(),
    };
    let result = diesel::insert_into(schema::session::table)
        .values(&new_session)
        .returning(schema::session::dsl::session_token)
        .get_result::<Vec<u8>>(&mut *conn)
        .await
        .unwrap();
    result
}
pub async fn create_otp() {

}

pub async fn get_id_pwd_by_username(conn: &mut Database, usernam: String) -> Option<(i32, String)> {

    let mut conn = conn.get().await.unwrap();
    use schema::users::dsl::*;
    match users
        .filter(username.eq(usernam))
        .select((id, passkey))
        .get_result::<(i32, String)>(&mut *conn)
        .await
    {
        Ok((user_id, password)) => Some((user_id, password)),
        Err(_) => None,
    }
}
pub async fn get_id_pwd_by_email(conn: &mut Database, mail_addr: String) -> Option<(i32, String)> {

    let mut conn = conn.get().await.unwrap();
    use schema::users::dsl::*;
    match users
        .filter(email.eq(mail_addr))
        .select((id, passkey))
        .get_result::<(i32, String)>(&mut *conn)
        .await
    {
        Ok((user_id, password)) => Some((user_id, password)),
        Err(_) => None,
    }
}
pub async fn delete_logged_in(
    conn: &mut Database,
    session_tk: SessionToken,
) -> Result<usize, diesel::result::Error> {

    let mut conn = conn.get().await.unwrap();
    use schema::session::dsl::*;
    use schema::users::dsl::*;
    let target_user_id: i32 = session
        .filter(session_token.eq(session_tk.into_database_value()))
        .select(user_id)
        .first::<i32>(&mut *conn)
        .await?;

    diesel::delete(users.filter(id.eq(target_user_id)))
        .execute(&mut *conn)
        .await
}

