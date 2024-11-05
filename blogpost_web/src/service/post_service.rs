use std::fmt::format;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::dto::post_dto::PostDTO;
use crate::repository::post_repository::PostRepository;
use crate::models::post::Post;

#[derive(Clone)]
pub(crate) struct PostService {
    post_repository: PostRepository,
}
impl PostService {
    pub fn new(post_repository: PostRepository) -> Self {
        Self{post_repository}
    }

    pub async fn create_post(&self, post_dto: PostDTO, file: Option<bytes::Bytes>) {


        let avatar_img_num=SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut avatar_dest_path:Option<String>=None;
        match post_dto.avatar_img_src() {
            None => {}
            Some(img_src) => {
                avatar_dest_path=Some(format!("{}{}{}", "uploads/avatar/",avatar_img_num.to_string(),".png"));
                Self::save_avatar_png(img_src.as_str(),avatar_dest_path.as_ref().unwrap()).await
            }
        }

        let blog_img_num=SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();
        let mut blog_dest_path:Option<String>=None;//format!("{}{}{}", "./static/uploads/blog/",blog_img_num.to_string(),".png")
        match file {
            None => {}
            Some(img) => {
                blog_dest_path = Some(format!("{}{}{}", "uploads/blog/",blog_img_num.to_string(),".png"));
                println!("{}",blog_dest_path.as_ref().unwrap());
                Self::save_blog_png(img,blog_dest_path.as_ref().unwrap().as_str());
            }
        }


        let post:Post= Post::new(
            None,
            post_dto.username().to_string(),
            post_dto.main_text().to_string(),
            post_dto.creation_date().to_string(),
            blog_dest_path,
            avatar_dest_path
        );

        self.post_repository.create_post(post).await;
    }

    pub async fn get_posts(&self)->Vec<PostDTO>{
        let posts:Vec<Post> =self.post_repository.get_posts().await;
        let mut posts_dtos:Vec<PostDTO>=vec![];

        for post in posts{
            posts_dtos.push(PostDTO::new(
                post.username().to_string(),
                post.main_text().to_string(),
                post.creation_date().to_string(),
                post.blog_img_src().clone(),
                post.avatar_img_src().clone()
            ))
        }

        posts_dtos
    }

    async fn save_avatar_png(img_src:&str,dest_path:&str) {
        let img=reqwest::get(img_src).await.unwrap().bytes().await.unwrap();
        let res=image::load_from_memory_with_format(&img, image::ImageFormat::Png).unwrap();
        let ress=res.save(format!("{}{}","./static/",dest_path));
        ress.unwrap();

    }

    fn save_blog_png(file: bytes::Bytes,dest_path:&str) {

        let res=image::load_from_memory_with_format(&file, image::ImageFormat::Png).unwrap();
        let ress=res.save(format!("{}{}","./static/",dest_path));
        ress.unwrap();

    }







}

