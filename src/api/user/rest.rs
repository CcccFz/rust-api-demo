use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json
};

use super::dto;


// entity
struct User {
    user_id: u64,
    user_name: String
}

static mut USERS: Vec<User> = Vec::new();

pub async fn user_create(Json(req): Json<dto::UserCreateReq>) -> impl IntoResponse {
    unsafe {
        USERS.push(User{
            user_id: req.user_id,
            user_name: req.user_name,
        });
    }
    
    (StatusCode::OK, format!("Success!\nAdd User: {}", req.user_id))
}

pub async fn user_detail(Json(req): Json<dto::UserReq>) -> impl IntoResponse {
    _ = req.user_id;

    unsafe {
        let rsp = Json(dto::UserDetailRsp{
            user_id: USERS[USERS.len()-1].user_id,
            user_name: USERS[USERS.len()-1].user_name.clone(),
        });

        return (StatusCode::OK, rsp);
    }
}
