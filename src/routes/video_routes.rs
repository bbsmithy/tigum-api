use crate::db;
use rocket::Route;

//Use Macros
use rocket_contrib::json::Json;

use db::models::resources::video::{NewVideo};
use db::models::Ids;
use db::api_response::ApiResponse;

use db::models::user::User;
use db::querys::video_query::{create_video, delete_video, get_video, get_videos, update_video, publish_video};
use db::querys::TigumPgConn;

/////////////////////////
//// VIDEO ROUTES ///////
/////////////////////////

#[delete("/videos/<id>")]
pub fn delete_single_video(conn: TigumPgConn, id: i32, auth_user: User) -> ApiResponse {
    delete_video(&*conn, id, auth_user.id)
}

#[put("/videos/<id>", format = "application/json", data = "<video>")]
pub fn update_single_video(
    conn: TigumPgConn,
    id: i32,
    video: Json<NewVideo>,
    auth_user: User,
) -> ApiResponse {
    update_video(&*conn, id, video, auth_user.id)
}

#[put("/videos/publish/<video_id>/<publish_flag>", format = "application/json")]
fn publish_single_video(conn: TigumPgConn, video_id: i32, publish_flag: bool, auth_user: User) -> ApiResponse {
    publish_video(&*conn, video_id, publish_flag, auth_user.id)
}

#[post("/videos/create", format = "application/json", data = "<video>")]
pub fn create_single_video(
    conn: TigumPgConn,
    video: Json<NewVideo>,
    auth_user: User,
) -> ApiResponse {
    create_video(&*conn, video, auth_user.id)
}

#[get("/videos/<id>")]
pub fn single_video(conn: TigumPgConn, id: i32, auth_user: User) -> ApiResponse {
    get_video(&*conn, id, auth_user.id)
}

#[post("/videos", format = "application/json", data = "<ids>")]
pub fn videos(conn: TigumPgConn, ids: Json<Ids>, auth_user: User) -> ApiResponse {
    get_videos(&*conn, ids, auth_user.id)
}

pub fn video_routes() -> Vec<Route> {
    routes![
        create_single_video,
        delete_single_video,
        single_video,
        videos,
        update_single_video,
        publish_single_video
    ]
}
