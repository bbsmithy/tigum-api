pub mod article_snippets;
pub mod code;
pub mod documents;
pub mod image;
pub mod note;
pub mod video;

#[derive(Debug)]
pub enum ResourceType {
    Snippet,
    Note,
    Video,
    Image,
    Document,
    Code,
}
