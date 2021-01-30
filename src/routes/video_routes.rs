use crate::db;
use rocket::Route;

//Use Macros
use rocket_contrib::json::Json;

use db::models::resources::video::{NewVideo};
use db::models::Ids;
use db::api_response::ApiResponse;

use db::models::user::User;
use db::querys::video_query::{create_video, delete_video, get_video, get_videos, update_video};
use db::querys::TigumPgConn;

/////////////////////////
//// VIDEO ROUTES ///////
/////////////////////////

#[delete("/videos/<id>")]
pub async fn delete_single_video(conn: TigumPgConn, id: i32, auth_user: User) -> ApiResponse {
    delete_video(&conn, id, auth_user.id).await
}

#[put("/videos/<id>", format = "application/json", data = "<video>")]
pub async fn update_single_video(
    conn: TigumPgConn,
    id: i32,
    video: Json<NewVideo>,
    auth_user: User,
) -> ApiResponse {
    update_video(&conn, id, video, auth_user.id).await
}

#[post("/videos/create", format = "application/json", data = "<video>")]
pub async fn create_single_video(
    conn: TigumPgConn,
    video: Json<NewVideo>,
    auth_user: User,
) -> ApiResponse {
    create_video(&conn, video, auth_user.id).await
}

#[get("/videos/<id>")]
pub async fn single_video(conn: TigumPgConn, id: i32, auth_user: User) -> ApiResponse {
    get_video(&conn, id, auth_user.id).await
}

#[post("/videos", format = "application/json", data = "<ids>")]
pub async fn videos(conn: TigumPgConn, ids: Json<Ids>, auth_user: User) -> ApiResponse {
    get_videos(&conn, ids, auth_user.id).await
}

pub fn video_routes() -> Vec<Route> {
    routes![
        create_single_video,
        delete_single_video,
        single_video,
        videos,
        update_single_video,
    ]
}
