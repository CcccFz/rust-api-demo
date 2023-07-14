use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub code: i64,
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Empty {}
