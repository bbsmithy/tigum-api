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
    TigumPgConn, create_topic, create_note, generate_single_note, generate_single_topic, generate_test_notes, generate_test_topics
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

#[post("/create-topic", format = "application/json", data = "<topic>")]
fn create_single_topic(conn: TigumPgConn, topic: Json<Topic>, auth_user: User) -> String {
    println!("Creating topic: {}", topic.title);
    let update = create_topic(&conn, topic);
    update
}

#[get("/topics/<topic_id>")]
fn single_topic(topic_id: i32, _auth_user: User) -> Json<Topic> {
    let topic = generate_single_topic(topic_id);
    return Json(topic);
}

#[get("/topics")]
fn topics(conn: TigumPgConn, _auth_user: User) -> Json<Vec<Topic>> {
    let mut topic_results: Vec<Topic> = vec![];
    for row in &conn.query("SELECT * FROM topics", &[]).unwrap() {
        let result_topic = Topic::new(row.get(1), row.get(2), row.get(0));
        topic_results.push(result_topic);
    }
    return Json(topic_results);
}

// CORS Prelight Request Handler

#[route(OPTIONS, path = "/")]
fn preflight_handler() {
    println!("{}", String::from("Handling preflight"))
}

fn main() {

    rocket::ignite()
        .mount("/", routes![topics, single_topic, create_single_topic, notes, single_note, create_single_note, preflight_handler])
        .attach(cors::CorsFairing)
        .attach(TigumPgConn::fairing())
        .launch();
}
