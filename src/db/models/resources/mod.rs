pub mod article_snippets;
pub mod question;
pub mod link;
pub mod image;
pub mod note;
pub mod video;

#[derive(Debug)]
pub enum ResourceType {
    Snippet,
    Note,
    Video,
    Image,
    Link,
    Code,
}

impl ResourceType {
    pub fn to_table_name(&self) -> &str {
        match self {
            Self::Video => "videos",
            Self::Snippet => "article_snippets",
            Self::Note => "notes",
            Self::Link => "links",
            Self::Code => "code",
            Self::Image => "images"
        }
    }
}


pub struct Resource<T>(T);
