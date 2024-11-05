use std::fs;
use std::sync::Arc;
use axum::{response::IntoResponse, routing::get, Router};
use axum::extract::DefaultBodyLimit;
use axum::response::Html;
use axum::routing::post;
use db_client::sqlite_db::SqliteDatabase;
use traits::db_client::DbClient;
use tower_http::services::ServeDir;
use crate::repository::post_repository::PostRepository;
use crate::service::post_service::PostService;

mod db_client;
mod handler;
mod repository;
mod models;
mod traits;
mod dto;
mod service;


const DB_URL: &str = "sqlite://database/app.db";

#[tokio::main]
async fn main() {

    let avatar_path = "./static/uploads/avatar";
    let blog_img_path="./static/uploads/blog";

    fs::create_dir_all(avatar_path).unwrap();
    fs::create_dir_all(blog_img_path).unwrap();


    let database=SqliteDatabase::new(DB_URL);
    database.init().await;

    let rep=PostRepository::new(Arc::new(database));
    let serv=Arc::new(PostService::new(rep));


    let app=Router::new()
        .route("/home", get(render_html))
        .route("/post",post(handler::post_handler::create_post))
        .route("/posts",get(handler::post_handler::get_posts))
        .with_state(serv)
        .nest_service("/", ServeDir::new("./static"))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024));



    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    }


async fn render_html() -> impl IntoResponse {
    // Specify the path to your HTML file
    let html_content=include_str!("../static/client/home.html");
    Html(html_content) // Return the HTML content
}