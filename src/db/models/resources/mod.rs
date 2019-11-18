pub mod article_snippets;
pub mod note;
pub mod video;
pub mod image;
pub mod documents;

#[derive(Debug)]
pub enum ResourceType {
    Snippet,
    Note,
    Video,
    Image,
    Document
}
