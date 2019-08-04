#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde;

use rocket_contrib::json::Json;

mod cors;
mod models;

use models::generate_test_notes;
use models::generate_test_topics;

use models::topic::note::{Note};
use models::topic::Topic;
use models::user::User;

#[get("/note/<topic_id>")]
fn note(topic_id: u64) -> Json<Vec<Note>> {
    println!("Fetching notes for topic: {}", topic_id);
    let notes: Vec<Note> = generate_test_notes(10);
    Json(notes)
}

#[post("/topics", format = "application/json", data = "<user>")]
fn topics(user: Json<User>) -> Json<Vec<Topic>> {
    println!("User Id {}", user.user_id);

    let topics: Vec<Topic> = generate_test_topics(10);

    return Json(topics);
}

#[get("/")]
fn home() -> String {
    String::from("Welcome to Tigum API!")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![home, topics, note])
        .attach(cors::CorsFairing)
        .launch();
}
