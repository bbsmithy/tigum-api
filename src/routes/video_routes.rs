use crate::db;
use rocket::Route;
use rocket::http::Status;

//Use Macros
use rocket_contrib::json::Json;

use db::models::resources::video::{NewVideo, Video};
use db::models::resources::ResourceType;
use db::models::Ids;
use db::api_response::ApiResponse;

use db::models::user::User;
use db::querys::topic_query::add_to_topic_resource_list;
use db::querys::video_query::{create_video, delete_video, get_video, get_videos, update_video};
use db::querys::TigumPgConn;

/////////////////////////
//// VIDEO ROUTES ///////
/////////////////////////

#[delete("/videos/<id>")]
pub fn delete_single_video(conn: TigumPgConn, id: i32, auth_user: User) -> Json<String> {
    delete_video(&conn, id, auth_user.id)
}

#[put("/videos/<id>", format = "application/json", data = "<video>")]
pub fn update_single_video(
    conn: TigumPgConn,
    id: i32,
    video: Json<NewVideo>,
    auth_user: User,
) -> Json<Video> {
    update_video(&conn, id, video, auth_user.id)
}

#[post("/videos/create", format = "application/json", data = "<video>")]
pub fn create_single_video(
    conn: TigumPgConn,
    video: Json<NewVideo>,
    auth_user: User,
) -> ApiResponse {
    let new_video_query_result = create_video(&conn, &video, auth_user.id);
    match new_video_query_result {
        Ok(new_video) => {
            let query_result = add_to_topic_resource_list(&conn, video.topic_id, new_video.id, ResourceType::Video);
            match query_result {
                Ok(_rows_updated) => ApiResponse { json: json!(new_video), status: Status::raw(200) },
                Err(_error) => ApiResponse {
                    json: json!({ "error": format!("Could not update topic resource list for video {}", new_video.topic_id )}),
                    status: Status::raw(500)
                }
            }
        },  
        Err(_error) => ApiResponse {
            json: json!({
                "error": format!("Could not create video {}", video.topic_id )
            }),
            status: Status::raw(500)
        }
    }
}

#[get("/videos/<id>")]
pub fn single_video(conn: TigumPgConn, id: i32, auth_user: User) -> Json<Video> {
    get_video(&conn, id, auth_user.id)
}

#[post("/videos", format = "application/json", data = "<ids>")]
pub fn videos(conn: TigumPgConn, ids: Json<Ids>, auth_user: User) -> Json<Vec<Video>> {
    get_videos(&conn, ids, auth_user.id)
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
