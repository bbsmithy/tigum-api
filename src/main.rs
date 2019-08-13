#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde;
extern crate rocket_contrib;

use rocket_contrib::json::Json;

use std::env;



mod cors;
mod guards;
mod db;

use db::models::topic::note::Note;
use db::models::topic::{Topic, TopicId};
use db::{
    generate_single_note, generate_single_topic, generate_test_notes, generate_test_topics
};


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

#[get("/topics/<topic_id>")]
fn single_topic(topic_id: u64, _auth_user: User) -> Json<Topic> {
    let topic = generate_single_topic(topic_id);
    return Json(topic);
}

#[get("/topics")]
fn topics(_auth_user: User) -> Json<Vec<Topic>> {
    // println!("{}",conn.0.name);
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

    let key = "RUST_BACKTRACE";
    env::set_var(key, "1");

    rocket::ignite()
        .mount("/", routes![home, topics, single_topic, notes, single_note, preflight_handler])
        .attach(cors::CorsFairing)
        .launch();
}
