use rocket_contrib::databases;

pub mod article_snippets_query;
pub mod link_query;
pub mod note_query;
pub mod topic_query;
pub mod user_query;
pub mod video_query;
pub mod search_resources_query;
pub mod public_query;

#[database("tigum_db")]
pub struct TigumPgConn(databases::diesel::PgConnection);
