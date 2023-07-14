use axum::{
    Router,
    routing::post,
};

use crate::user;

#[tokio::main]
pub async fn register_and_run() {
    let router = Router::new()
        .route("/api/user/create", post(user::handler::user_create))
        .route("/api/user/detail", post(user::handler::user_detail));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}