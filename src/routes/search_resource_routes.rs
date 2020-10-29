use crate::db;
use rocket::Route;

// DB query
use db::querys::TigumPgConn;
use db::models::user::User;
use db::api_response::ApiResponse;
use db::querys::search_resources_query::{find_by_title};

#[get("/search/<title>", format = "application/json")]
pub fn find_resource_with_title(conn: TigumPgConn, title: String, auth_user: User) -> ApiResponse {
    find_by_title(&conn, title.to_string(), auth_user.id)
}

pub fn get_search_resource_routes() -> Vec<Route> {
    routes![find_resource_with_title]
}