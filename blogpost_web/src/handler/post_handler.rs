use std::sync::Arc;
use axum::extract::State;
use axum::extract::multipart::Multipart;
use axum::response::IntoResponse;
use bytes::Bytes;
use crate::dto::post_dto::PostDTO;
use crate::service::post_service::PostService;
use std::option::Option;
use axum::Json;

pub async fn create_post(State(post_service): State<Arc<PostService>>, mut multipart: Multipart) ->  impl IntoResponse   {

    let mut post_dto:Option<PostDTO> = None;
    let mut file:Option<Bytes>=None ;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();

        if name == "dto" {
            // Handle JSON field
            post_dto = Some(serde_json::from_str::<PostDTO>(&field.text().await.unwrap()).unwrap());

        } else if name == "file" {
            // Handle file field
            file = Some(field.bytes().await.unwrap());
        }

    };
    
    if post_dto.is_some() {

        post_service.create_post(post_dto.unwrap(),file).await

    }

}

pub async fn get_posts(State(post_service): State<Arc<PostService>>)->Json<Vec<PostDTO>>{
    let posts_dtos:Vec<PostDTO>=post_service.get_posts().await;
    Json(posts_dtos)
}


