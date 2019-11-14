use rocket_contrib::databases;

pub mod note_query;
pub mod topic_query;
pub mod video_query;
pub mod article_snippets_query;
pub mod image_query;

#[database("tigum_db")]
pub struct TigumPgConn(databases::postgres::Connection);
