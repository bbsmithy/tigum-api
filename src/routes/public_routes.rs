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
async fn get_notes(conn: TigumPgConn, topic_id: i32) {
    public_query::get_public_notes_in_topic(&conn, topic_id).await;
}

pub fn get_public_routes() -> Vec<Route> {
    routes![get_profile, get_notes]
}
