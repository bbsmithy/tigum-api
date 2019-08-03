#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde;

use rocket_contrib::json::Json;


mod cors;
mod models;

use models::topic::note::{Note, NoteItem};
use models::topic::Topic;
use models::user::User;

#[get("/note/<note_id>")]
fn note(note_id: u64) -> Json<Note> {
    let first_note_title = "Test title".to_string();
    let first_note_body = "Test body".to_string();

    let mut note_detail = Note::new(first_note_title, first_note_body, note_id);

    let test_note_text = NoteItem::new(String::from("text"), 1234, String::from("some content"));

    note_detail.add_note_item(test_note_text);

    println!("{}", note_detail.note_content[0].content);

    Json(note_detail)
}

#[post("/topics", format = "application/json", data = "<user>")]
fn topics(user: Json<User>) -> Json<Topic> {
    println!("User Id {}", user.user_id);

    let first_note_title = "Test title".to_string();
    let first_note_body = "Test body".to_string();
    let second_note_title = "Test title 2".to_string();
    let second_note_body = "Test body 2".to_string();

    let first_note = Note::new(first_note_title, first_note_body, 12);
    let mut second_note = Note::new(second_note_title, second_note_body, 13);

    let test_note_text = NoteItem::new(String::from("text"), 1234, String::from("some content"));
    second_note.add_note_item(test_note_text);

    let notes: Vec<Note> = vec![first_note, second_note];

    let topic: Topic = Topic {
        topic_id: 12,
        title: String::from("Carl Jung on dreams"),
        date_created: String::from("12th March 2019"),
        my_notes: notes,
    };

    return Json(topic);
}

#[get("/")]
fn home() -> String {
    String::from("Welcome to Tigum!")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![topics, note])
        .attach(cors::CorsFairing)
        .launch();
}
