use async_graphql::{Object, Response, Result};
use reqwest::Client;

use crate::models::{DeletePostResponse, Post, PostInput, UpdatePostInput};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        "world".to_string()
    }

    async fn get_posts(&self) -> Result<Vec<Post>, reqwest::Error> {
        let client = Client::new();
        let res = client
            .get("https://jsonplaceholder.typicode.com/posts")
            .send()
            .await?;
        match res.error_for_status() {
            Ok(res) => {
                let posts: Vec<Post> = res.json().await?;
                Ok(posts)
            }
            Err(e) => {
                println!("Error: {}", e);
                Err(e)
            }
        }
    }
    async fn get_post(&self, id: i32) -> Result<Post, reqwest::Error> {
        let client = Client::new();
        let res = client
            .get(format!("https://jsonplaceholder.typicode.com/posts/{}", id))
            .send()
            .await?;
        match res.error_for_status() {
            Ok(res) => {
                let post: Post = res.json().await?;
                Ok(post)
            }
            Err(e) => {
                println!("Error: {}", e);
                Err(e)
            }
        }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn update_post(&self, input: UpdatePostInput) -> Result<Post, reqwest::Error> {
        let client = Client::new();
        let res = client
            .put(format!(
                "https://jsonplaceholder.typicode.com/posts/{}",
                input.id
            ))
            .json(&input)
            .send()
            .await?;
        match res.error_for_status() {
            Ok(res) => {
                let post: Post = res.json().await?;
                Ok(post)
            }
            Err(e) => {
                println!("Request Error: {}", e);
                Err(e)
            }
        }
    }

    async fn create_post(&self, input: PostInput) -> Result<Post, reqwest::Error> {
        let client = Client::new();
        let res = client
            .post("https://jsonplaceholder.typicode.com/posts")
            .json(&input)
            .send()
            .await?;
        match res.error_for_status() {
            Ok(res) => {
                let post: Post = res.json().await?;
                Ok(post)
            }
            Err(e) => {
                println!("Request Error: {}", e);
                Err(e)
            }
        }
    }

    async fn delete_post(&self, id: i32) -> Result<DeletePostResponse, reqwest::Error> {
        let client = Client::new();
        let res = client
            .delete(format!("https://jsonplaceholder.typicode.com/posts/{}", id))
            .send()
            .await?;
        match res.error_for_status() {
            Ok(_) => Ok(DeletePostResponse {
                message: "Post deleted".to_string(),
            }),
            Err(e) => {
                println!("Request Error: {}", e);
                Err(e)
            }
        }
    }
}
