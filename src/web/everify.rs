use std::clone;

use axum::{
    extract::Multipart,
    response::{IntoResponse, Redirect, Response},
    Extension, Json,
};

use http::StatusCode;
use lettre::{
    transport::smtp::{self, authentication::Credentials},
    Message, SmtpTransport, Transport,
};
use serde::Deserialize;

use crate::{
    auth::authentication::{get_otp, verify_email},
    database::{queries::{create_user, Database}, schema::mfa::otp},
};

pub async fn verify_email_b4_create(
    rotp: Extension<ROtp>,
    conn: &Database,
    username: String,
    password: String,
    email: String,
    lotp: Multipart,
) -> Option<i32> {
    let _ = send_otp(rotp.value.clone(), &email).await;

    Redirect::to(&format!("/get_otp"));
    if verify_email(lotp).await.unwrap() {
        Some(create_user(conn, username, password, email).await.unwrap())
    } else {
        None
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct Otp {
otp: String
}
pub async fn post_otp(code: Json<Otp>) {

}

pub async fn view_otp() -> String {
    get_otp().await
}
#[derive(Debug)]
pub enum Errors {
    Error1(smtp::Error),
    Error2(Response),
    //  Error3(Box<dyn std::error::Error>)
}
#[derive(Deserialize, Clone)]
pub struct ROtp {
    pub value: String,
}

async fn send_otp(otp: String, email: &String) -> Result<Response, Errors> {
    let email = Message::builder()
        .from("yemelechrristian2@gmail.com".parse().unwrap())
        .reply_to("to@example.com".parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Rust Email")
        .body(otp)
        .unwrap();

    // Set up the SMTP client
    let creds = Credentials::new("d006ed6ea804b3".to_string(), "25553583d3558b".to_string());
    // Open a remote connection to gmail

    let mailer = SmtpTransport::starttls_relay("sandbox.smtp.mailtrap.io")
        .unwrap()
        .credentials(creds)
        .build();
    match mailer.send(&email) {
        Ok(_) => Ok(StatusCode::OK.into_response()),
        Err(_) => Err(Errors::Error2(
            StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        )),
    }
}
