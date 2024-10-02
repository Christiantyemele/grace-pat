use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::routing::{delete, post};
use axum::Extension;
use axum::{middleware, response::IntoResponse, routing::get, Router};
use grace::auth::authentication::{auth, verify_email};
use grace::database::connection::establish_connection;
use grace::utils::logout_response;
use grace::web::delete::{self, post_delete_me};
use grace::web::everify::{post_veriy_email, view_otp};
use grace::web::login::post_login;
use grace::web::signup::post_signup;
use rand_chacha::ChaCha8Rng;
use rand_core::{OsRng, RngCore, SeedableRng};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

async fn welcome() -> impl IntoResponse {
    "Welcome to Grace Pattiserie"
}
#[tokio::main]
async fn main() {
    let dbconn_pool = establish_connection().await;

    let mdlw_db = dbconn_pool.clone();
    let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());
    let random = Arc::new(Mutex::new(random));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let router = Router::new()
        .route("/", get(welcome))
        .route("/signup", post(post_signup))
        .route("/login", post(post_login))
        .route("/logout", post(logout_response))
        .route("/delete", delete(post_delete_me))
        .route("/verifyemail", post(post_veriy_email))
        .route("/getopt", get(view_otp))
        .layer(middleware::from_fn(
            move |req: http::Request<axum::body::Body>, next| auth(mdlw_db.clone(), req, next),
        ))
        .layer(CookieManagerLayer::new())
        .layer(Extension(dbconn_pool))
        .layer(Extension(random));

    axum::serve(listener, router).await.unwrap();
}
