use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UserReq {
    pub user_id: u64
}

#[derive(Debug, Deserialize)]
pub struct UserCreateReq {
    pub user_id: u64,
    pub user_name: String
}

#[derive(Debug, Serialize)]
pub struct UserDetailRsp {
    pub user_id: u64,
    pub user_name: String
}