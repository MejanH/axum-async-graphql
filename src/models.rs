use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, InputObject)]
pub struct PostInput {
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, InputObject)]
pub struct UpdatePostInput {
    pub id: i32,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct DeletePostResponse {
    pub message: String,
}
