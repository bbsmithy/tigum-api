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
use db::models::topic::{Topic, TopicId};
use db::{
    TigumPgConn, create_topic, generate_single_note, generate_single_topic, generate_test_notes, generate_test_topics
};

// Request Gaurds
use guards::User;




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

#[post("/topics", format = "application/json", data = "<topic>")]
fn create_single_topic(conn: TigumPgConn, topic: Json<Topic>, auth_user: User) -> String {
    println!("Creating topic: {}", topic.title);
    let update = create_topic(&conn, topic);
    update
}

#[get("/topics/<topic_id>")]
fn single_topic(topic_id: u64, _auth_user: User) -> Json<Topic> {
    let topic = generate_single_topic(topic_id);
    return Json(topic);
}

#[get("/topics")]
fn topics(conn: TigumPgConn, _auth_user: User) -> Json<Vec<Topic>> {

    let topics: Vec<Topic> = generate_test_topics(10);
    return Json(topics);
}

#[route(OPTIONS, path = "/")]
fn preflight_handler() {
    println!("{}", String::from("Handling preflight"))
}

#[get("/")]
fn home() -> String {
    String::from("Welcome to Tigum API!")
}

fn main() {

    rocket::ignite()
        .mount("/", routes![home, topics, single_topic, create_single_topic, notes, single_note, preflight_handler])
        .attach(cors::CorsFairing)
        .attach(TigumPgConn::fairing())
        .launch();
}
