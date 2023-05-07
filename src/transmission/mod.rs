use serde::{Deserialize, Serialize};

pub mod client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request<T> {
    method: String,
    tag: u32,
    arguments: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    arguments: T,
    result: String,
    tag: u32,
}
