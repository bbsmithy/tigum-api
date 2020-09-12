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
