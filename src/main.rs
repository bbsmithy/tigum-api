#![feature(proc_macro_hygiene, decl_macro)]

// Macros
#[macro_use]
extern crate rocket;
extern crate serde;
#[macro_use] extern crate rocket_contrib;

//Use Macros
use rocket_contrib::json::Json;

// Main modules
mod cors;
mod guards;
mod db;

// Database Models and Functions
use db::models::topic::note::Note;
use db::models::topic::{Topic, TopicId, TopicIds};
use db::{
    TigumPgConn, create_topic, get_topic, get_topics, update_topic, create_note, generate_single_note,  generate_test_notes
};

// Request Gaurds
use guards::User;



// Note Routes

#[post("/create-note", format = "application/json", data = "<note>")]
fn create_single_note(conn: TigumPgConn, note: Json<Note>) -> String {
    let update = create_note(&conn, note);
    format!("Row affected {}", update)
}

#[get("/notes/<topic_id>")]
fn single_note(topic_id: u64, _auth_user: User) -> Json<Note> {
    println!("Fetching notes for topic: {}", topic_id);
    let note_response = generate_single_note();
    return Json(note_response);
}

#[post("/notes", format = "application/json", data = "<topic_id>")]
fn notes(topic_id: Json<TopicId>, _auth_user: User) -> Json<Vec<Note>> {
    println!("{}", topic_id.topic_id.to_string());
    let notes: Vec<Note> = generate_test_notes(10);
    return Json(notes);
}


// Topic Routes

#[put("/topics/<topic_id>", format = "application/json", data = "<topic>")]
fn update_single_topic(conn: TigumPgConn, topic_id: i32, topic: Json<Topic>, auth_user: User) -> Json<Topic> {
    let result = update_topic(&conn, topic_id, topic);
    return result;
}


#[post("/topics/create-topic", format = "application/json", data = "<topic>")]
fn create_single_topic(conn: TigumPgConn, topic: Json<Topic>, auth_user: User) -> Json<Vec<Topic>> {
    let update = create_topic(&conn, topic);
    update
}

#[get("/topics/<topic_id>")]
fn single_topic(conn: TigumPgConn, topic_id: i32, _auth_user: User) -> Json<Vec<Topic>> {
    let topic = get_topic(&conn, topic_id);
    return topic;
}

#[post("/topics", format = "application/json", data = "<topic_ids>")]
fn topics(conn: TigumPgConn, topic_ids: Json<TopicIds>, _auth_user: User) -> Json<Vec<Topic>> {
    let result_topics = get_topics(&conn, topic_ids);
    return result_topics;
}

// CORS Prelight Request Handler

#[route(OPTIONS, path = "/")]
fn preflight_handler() {
    println!("{}", String::from("Handling preflight"))
}

fn main() {

    rocket::ignite()
        .mount("/", routes![topics, single_topic, create_single_topic, update_single_topic, notes, single_note, create_single_note, preflight_handler])
        .attach(cors::CorsFairing)
        .attach(TigumPgConn::fairing())
        .launch();
}
