use crate::db;
use crate::User;
use rocket::Route;

//Use Macros
use rocket_contrib::json::Json;

use db::querys::video_query::{create_video, delete_video, get_video, get_videos, update_video};
use db::querys::TigumPgConn;
use db::models::resources::video::{NewVideo, Video};
use db::models::{Id, Ids};

/////////////////////////
//// VIDEO ROUTES ///////
/////////////////////////

#[delete("/videos/<id>")]
pub fn delete_single_video(conn: TigumPgConn, id: i32, _auth_user: User) -> Json<String> {
    delete_video(&conn, id)
}

#[put("/videos/<id>", format = "application/json", data = "<video>")]
pub fn update_single_video(conn: TigumPgConn, id: i32, video: Json<NewVideo>) -> Json<Video> {
    update_video(&conn, id, video)
}

#[post("/videos/create", format = "application/json", data = "<video>")]
pub fn create_single_video(conn: TigumPgConn, video: Json<NewVideo>) -> Json<Id> {
    println!("{:?}", video);
    create_video(&conn, video)
}

#[get("/videos/<id>")]
pub fn single_video(conn: TigumPgConn, id: i32, _auth_user: User) -> Json<Video> {
    get_video(&conn, id)
}

#[post("/videos", format = "application/json", data = "<ids>")]
pub fn videos(conn: TigumPgConn, ids: Json<Ids>) -> Json<Vec<Video>> {
    println!("{:?}", ids);
    get_videos(&conn, ids)
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