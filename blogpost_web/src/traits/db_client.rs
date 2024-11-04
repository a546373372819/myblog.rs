use std::println;
use axum::async_trait;
use sqlx::sqlite::SqliteRow;
use sqlx::SqlitePool;
use crate::models::post::Post;

#[async_trait]
pub  trait DbClient:Send + Sync{
        async fn init(&self);
        async fn create(&self,query: &str);
        async fn get_all(&self, query: &str)->Vec<SqliteRow>;
}



