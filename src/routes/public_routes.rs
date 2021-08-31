use crate::db;
use rocket::Route;

// Models
use db::api_response::ApiResponse;

// Querys
use db::querys::TigumPgConn;
use db::querys::public_query;


///////////////////////
//// PUBLIC ROUTES ////
///////////////////////

#[get("/profile/<user_name>", format = "application/json")]
async fn get_profile(conn: TigumPgConn, user_name: String) -> ApiResponse {
    public_query::get_public_topics_for_user(&conn, user_name).await
}

#[get("/profile/notes/<topic_id>", format = "application/json")]
async fn get_notes(conn: TigumPgConn, topic_id: i32) -> ApiResponse {
    public_query::get_public_notes_in_topic(&conn, topic_id).await
}

#[get("/profile/videos/<topic_id>", format = "application/json")]
async fn get_videos(conn: TigumPgConn, topic_id: i32) -> ApiResponse {
    public_query::get_public_videos_in_topic(&conn, topic_id).await
}

#[get("/profile/snippets/<topic_id>", format = "application/json")]
async fn get_snippets(conn: TigumPgConn, topic_id: i32) -> ApiResponse {
    public_query::get_public_snippets_in_topic(&conn, topic_id).await
}

#[get("/profile/links/<topic_id>", format = "application/json")]
async fn get_links(conn: TigumPgConn, topic_id: i32) -> ApiResponse {
    public_query::get_public_links_in_topic(&conn, topic_id).await
}

pub fn get_public_routes() -> Vec<Route> {
    routes![get_profile, get_notes, get_videos, get_snippets, get_links]
}
