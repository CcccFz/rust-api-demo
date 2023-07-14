use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json
};

use crate::http;
use super::dto;
use super::entity;

pub async fn user_create(Json(req): Json<dto::UserCreateReq>) -> impl IntoResponse {
    unsafe {
        entity::USERS.push(entity::User{
            user_id: req.user_id,
            user_name: req.user_name,
        });
    }
    
    (StatusCode::OK, Json(http::Response {
        code: 0,
        data: http::Empty {},
    }))
}

pub async fn user_detail(Json(req): Json<dto::UserReq>) -> impl IntoResponse {
    _ = req.user_id;

    unsafe {
        let len = entity::USERS.len();

        let rsp = Json(http::Response {
            code: 0,
            data: dto::UserDetailRsp{
                user_id: entity::USERS[len-1].user_id,
                user_name: entity::USERS[len-1].user_name.clone(),
            }
        });

        (StatusCode::OK, rsp)
    }
}
