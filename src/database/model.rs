use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::database::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub passkey: String,
    pub email: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::users)]
pub struct NewUser {
    pub username: String,
    pub passkey: String,
    pub email: Option<String>,
}
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::session)]
#[derive(Debug)]
pub struct Session {
    pub user_id: i32,
    pub session_token: Vec<u8>,
}
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::mfa)]
#[derive(Debug)]
pub struct Mfa {
    pub id: i32,
    pub otp: i32,
}
