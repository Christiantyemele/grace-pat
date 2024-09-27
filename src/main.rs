use axum::{response::IntoResponse, routing::get, Router};
use grace::database::connection::establish_connection;
use tokio::net::TcpListener;

async fn welcome() -> impl IntoResponse {
    "Welcome to Grace Pattiserie"
}
#[tokio::main]
async fn main() {

    let dbconn_pool =|| async{ establish_connection().await};

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let router = Router::new().route("/", get(welcome));
    axum::serve(listener, router).await.unwrap();

}
