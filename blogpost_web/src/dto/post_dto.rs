use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct PostDTO {
    username:String,
    main_text:String,
    creation_date:String,
    blog_img_src:Option<String>,
    avatar_img_src:Option<String>
}


impl PostDTO {
    pub fn new(username: String, main_text: String, creation_date: String, blog_img_src: Option<String>, avatar_img_src: Option<String>) -> Self {
        Self { username, main_text, creation_date, blog_img_src, avatar_img_src }
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

    pub fn set_blog_img_src(&mut self, blog_img_src: String) {
        self.blog_img_src = Option::from(blog_img_src);
    }
}