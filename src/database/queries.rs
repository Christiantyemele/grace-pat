
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};
use diesel::{prelude::*, result::Error};
use crate::database::{model::NewUser, schema};

use super::error::DatabaseError;

pub type Database = Pool<AsyncPgConnection>;

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
pub async fn delete_user(conn: &mut Database, usernam: String) -> Result<(), Error>{
    let mut conn = conn.get().await.unwrap();
    use schema::users::dsl::*;
    diesel::delete(users.filter(username.like(usernam)))
        .execute(&mut conn)
        .await.map_err(|_| DatabaseError::DeletionError)?
}
