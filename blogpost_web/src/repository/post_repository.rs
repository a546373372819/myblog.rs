use std::sync::Arc;
use sqlx::FromRow;
use crate::models::post::Post;
use crate::traits::db_client::DbClient;

#[derive(Clone)]
pub struct PostRepository {
    db_client: Arc::<dyn DbClient+Send+Sync>,
}

impl PostRepository {

    pub fn new(db_client: Arc::<dyn DbClient>) -> Self {
        Self { db_client }
    }

    pub async fn create_post(&self, post:Post) {

        let blog_img_src=match post.blog_img_src() {
            None => {"NULL"}
            Some(src) => &*{ format!("'{}'", src) }
        };

        let avatar_img_src=match post.avatar_img_src() {
            None => {"NULL"}
            Some(src) => &*{ format!("'{}'", src) }
        };

        let create_query=format!
        (
            "INSERT INTO posts (username, creation_date, main_text,blog_img_src,avatar_img_src)
            VALUES ('{}', '{}', '{}', {},{})",
            post.username(),post.creation_date(),post.main_text(),blog_img_src,avatar_img_src
        );


        self.db_client.create(create_query.as_str()).await;
    }

    pub async fn get_posts(&self)->Vec<Post> {
        let query="SELECT * FROM posts";
        let rows=self.db_client.get_all(query).await;
        let mut posts:Vec<Post> = vec![];
        for row in rows{
            posts.push(Post::from_row(&row).unwrap())
        }
        posts
    }

}