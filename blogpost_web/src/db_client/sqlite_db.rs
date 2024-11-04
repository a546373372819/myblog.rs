use axum::async_trait;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};
use sqlx::sqlite::SqliteRow;
use traits::db_client::DbClient;
use crate::models::post::Post;
use crate::traits;

pub struct SqliteDatabase {
    url:String,
}

impl SqliteDatabase {
    pub fn new(url:&str) -> Self {
        Self{url:url.into()}
    }
}

#[async_trait]
impl DbClient for SqliteDatabase {

    async fn init(&self) {

        if !Sqlite::database_exists(self.url.as_str()).await.unwrap_or(false) {
            match Sqlite::create_database(self.url.as_str()).await {
                Ok(_) => { } ,
                Err(error) => panic!("error: {}", error),
            }
        }

        let db = SqlitePool::connect(self.url.as_str()).await.unwrap();
        let result = sqlx::query(include_str!("../query/db_init.sql")).execute(&db).await.unwrap();

    }

    async fn create(&self, query: &str) {

        let db = SqlitePool::connect(self.url.as_str()).await.unwrap();

        let result = sqlx::query(query)
            .execute(&db)
            .await
            .unwrap();

    }

    async fn get_all(&self, query: &str) -> Vec<SqliteRow> {

        let db = SqlitePool::connect(self.url.as_str()).await.unwrap();

        let result = sqlx::query(query)
            .fetch_all(&db).await.unwrap();

        result

    }


}