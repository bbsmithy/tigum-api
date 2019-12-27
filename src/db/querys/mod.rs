use rocket_contrib::databases;

pub mod article_snippets_query;
pub mod code_query;
pub mod image_query;
pub mod link_query;
pub mod note_query;
pub mod topic_query;
pub mod user_query;
pub mod video_query;

#[database("tigum_db")]
pub struct TigumPgConn(databases::postgres::Connection);
