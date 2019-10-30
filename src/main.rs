#![feature(proc_macro_hygiene, decl_macro)]

// Macros
#[macro_use]
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate rocket_contrib;

//Use Macros
use rocket_contrib::json::Json;

// Main modules
mod cors;
mod db;
mod guards;

// Database Models
use db::models::resources::note::{NewNote, NewResource, Note, NoteIds, Resource};
use db::models::resources::video::{NewVideo, Video};
use db::models::topic::{NewTopic, Topic, TopicIds};
use db::models::{Id, Ids};

//Database Querys
use db::querys::note_q::{create_note, delete_note, get_note, get_notes, update_note};
use db::querys::topic_q::{create_topic, delete_topic, get_topic, get_topics, update_topic};
use db::querys::video_q::{create_video, delete_video, get_video, get_videos, update_video};
use db::querys::TigumPgConn;
use db::querys::{create_resource, delete_resource, get_resource, get_resources, update_resource};

// Request Gaurds
use guards::User;

/////////////////////////
//// VIDEO ROUTES ////
/////////////////////////

#[delete("/videos/<id>")]
fn delete_single_video(conn: TigumPgConn, id: i32, _auth_user: User) -> Json<String> {
    delete_video(&conn, id)
}

#[put("/videos/<id>", format = "application/json", data = "<video>")]
fn update_single_video(conn: TigumPgConn, id: i32, video: Json<NewVideo>) -> Json<Video> {
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
fn videos(conn: TigumPgConn, ids: Json<Ids>) -> Json<Vec<Video>> {
    println!("{:?}", ids);
    get_videos(&conn, ids)
}

/////////////////////////
//// RESOURCE ROUTES ////
/////////////////////////

#[delete("/resources/<resource_id>")]
fn delete_single_resource(conn: TigumPgConn, resource_id: i32, _auth_user: User) -> Json<String> {
    delete_resource(&conn, resource_id)
}

#[put(
    "/resources/<resource_id>",
    format = "application/json",
    data = "<resource>"
)]
fn update_single_resource(
    conn: TigumPgConn,
    resource_id: i32,
    resource: Json<Resource>,
) -> Json<Resource> {
    update_resource(&conn, resource_id, resource)
}

#[post(
    "/resources/create-resource",
    format = "application/json",
    data = "<resource>"
)]
pub fn create_single_resource(conn: TigumPgConn, resource: Json<NewResource>) -> Json<Id> {
    create_resource(&conn, resource)
}

#[get("/resources/<resource_id>")]
pub fn single_resource(conn: TigumPgConn, resource_id: i32, _auth_user: User) -> Json<Resource> {
    get_resource(&conn, resource_id)
}

#[post("/resources", format = "application/json", data = "<resource_ids>")]
fn resources(conn: TigumPgConn, resource_ids: Json<Ids>) -> Json<Vec<Resource>> {
    get_resources(&conn, resource_ids)
}

/////////////////////
//// NOTE ROUTES ////
/////////////////////

#[delete("/notes/<note_id>")]
fn delete_single_note(conn: TigumPgConn, note_id: i32, _auth_user: User) -> Json<String> {
    delete_note(&conn, note_id)
}

#[put("/notes/<note_id>", format = "application/json", data = "<note>")]
fn update_single_note(conn: TigumPgConn, note_id: i32, note: Json<Note>) -> Json<Note> {
    update_note(&conn, note_id, note)
}

#[post("/notes/create-note", format = "application/json", data = "<note>")]
fn create_single_note(conn: TigumPgConn, note: Json<NewNote>, _auth_user: User) -> Json<Id> {
    create_note(&conn, note)
}

#[get("/notes/<note_id>")]
fn single_note(conn: TigumPgConn, note_id: i32, _auth_user: User) -> Json<Note> {
    get_note(&conn, note_id)
}

#[post("/notes", format = "application/json", data = "<note_ids>")]
fn notes(conn: TigumPgConn, note_ids: Json<NoteIds>, _auth_user: User) -> Json<Vec<Note>> {
    get_notes(&conn, note_ids)
}

//////////////////////
//// TOPIC ROUTES ////
//////////////////////

#[delete("/topics/<topic_id>")]
fn delete_single_topic(conn: TigumPgConn, topic_id: i32) -> String {
    delete_topic(&conn, topic_id)
}

#[put("/topics/<topic_id>", format = "application/json", data = "<topic>")]
fn update_single_topic(
    conn: TigumPgConn,
    topic_id: i32,
    topic: Json<Topic>,
    _auth_user: User,
) -> Json<Topic> {
    update_topic(&conn, topic_id, topic)
}

#[post("/topics/create-topic", format = "application/json", data = "<topic>")]
fn create_single_topic(conn: TigumPgConn, topic: Json<NewTopic>, _auth_user: User) -> Json<Topic> {
    create_topic(&conn, topic)
}

#[get("/topics/<topic_id>")]
fn single_topic(conn: TigumPgConn, topic_id: i32, _auth_user: User) -> Json<Topic> {
    get_topic(&conn, topic_id)
}

#[post("/topics", format = "application/json", data = "<topic_ids>")]
fn topics(conn: TigumPgConn, topic_ids: Json<TopicIds>, _auth_user: User) -> Json<Vec<Topic>> {
    println!("{:?}", topic_ids);
    get_topics(&conn, topic_ids)
}

// CORS Prelight Request Handler

#[route(OPTIONS, path = "/")]
fn preflight_handler() -> String {
    let res: String = String::from("Handling preflight");
    res
}

fn create_routes() -> Vec<rocket::Route> {
    let app_routes = routes![
        topics,
        single_topic,
        create_single_topic,
        update_single_topic,
        delete_single_topic,
        notes,
        single_note,
        create_single_note,
        update_single_note,
        delete_single_note,
        resources,
        single_resource,
        create_single_resource,
        update_single_resource,
        delete_single_resource,
        create_single_video,
        delete_single_video,
        single_video,
        videos,
        update_single_video,
        preflight_handler
    ];
    app_routes
}

fn main() {
    let routes = create_routes();
    rocket::ignite()
        .mount("/", routes)
        .attach(cors::CorsFairing)
        .attach(TigumPgConn::fairing())
        .launch();
}
