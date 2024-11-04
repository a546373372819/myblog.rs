use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
pub struct Post{
    id: Option<i32>,
    username:String,
    main_text:String,
    creation_date:String,
    blog_img_src:Option<String>,
    avatar_img_src:Option<String>
}

impl Post {
    pub fn new(id: Option<i32>, username: String, main_text: String, creation_date: String, blog_img_src: Option<String>, avatar_img_src: Option<String>) -> Self {
        Self { id:id, username, main_text, creation_date, blog_img_src, avatar_img_src }
    }

    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn main_text(&self) -> &str {
        &self.main_text
    }

    pub fn creation_date(&self) -> &str {
        &self.creation_date
    }

    pub fn blog_img_src(&self) -> &Option<String> {
        &self.blog_img_src
    }

    pub fn avatar_img_src(&self) -> &Option<String> {
        &self.avatar_img_src
    }
}