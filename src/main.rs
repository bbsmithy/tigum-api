#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;

mod models;

use models::topic::note::{Note, NoteItem};
use models::topic::Topic;

#[get("/note/<note_id>")]
fn note(note_id: u64) -> Json<Note> {
    let first_note_title = "Test title".to_string();
    let first_note_body = "Test body".to_string();

    let note_detail = Note::new(first_note_title, first_note_body, note_id);

    // let test_note_text = NoteItem::NoteText {
    //     body: String::from("Hello")
    // };

    // note_detail.add_note_item(test_note_text);

    Json(note_detail)
}

#[get("/topics")]
fn topics() -> Json<Topic> {
    let first_note_title = "Test title".to_string();
    let first_note_body = "Test body".to_string();
    let second_note_title = "Test title 2".to_string();
    let second_note_body = "Test body 2".to_string();

    let first_note = Note::new(first_note_title, first_note_body, 12);
    let second_note = Note::new(second_note_title, second_note_body, 13);

    let notes: Vec<Note> = vec![first_note, second_note];

    let topic: Topic = Topic {
        topic_id: 12,
        title: String::from("Carl Jung on dreams"),
        date_created: String::from("12th March 2019"),
        my_notes: notes,
    };

    return Json(topic);
}

fn main() {
    rocket::ignite().mount("/", routes![topics, note]).launch();
}
