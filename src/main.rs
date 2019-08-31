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

// Database Models and Querys
use db::models::topic::note::{Note, NoteId, NoteIds};
use db::models::topic::{Topic, TopicIds};
use db::TigumPgConn;
use db::{create_topic, get_topic, get_topics, update_topic, delete_topic};
use db::{create_note, get_note, get_notes};

// Request Gaurds
use guards::User;



/////////////////////
//// NOTE ROUTES ////
/////////////////////

#[post("/notes/create-note", format = "application/json", data = "<note>")]
fn create_single_note(conn: TigumPgConn, note: Json<Note>) -> String {
    let update = create_note(&conn, note);
    format!("Row affected {}", update)
}

#[get("/notes/<note_id>")]
fn single_note(conn: TigumPgConn, note_id: i32, _auth_user: User) -> Json<Note> {
    let note_response = get_note(&conn, note_id);
    note_response
}

#[post("/notes", format = "application/json", data = "<note_ids>")]
fn notes(conn: TigumPgConn, note_ids: Json<NoteIds>, _auth_user: User) -> Json<Vec<Note>> { 
    let notes_response = get_notes(&conn, note_ids);
    return notes_response
}


//////////////////////
//// TOPIC ROUTES ////
//////////////////////

#[delete("/topics/<topic_id>")]
fn delete_single_topic(conn: TigumPgConn, topic_id: i32) -> String {
    let result = delete_topic(&conn, topic_id);
    return result;
}


#[put("/topics/<topic_id>", format = "application/json", data = "<topic>")]
fn update_single_topic(conn: TigumPgConn, topic_id: i32, topic: Json<Topic>, auth_user: User) -> Json<Topic> {
    let result = update_topic(&conn, topic_id, topic);
    return result;
}


#[post("/topics/create-topic", format = "application/json", data = "<topic>")]
fn create_single_topic(conn: TigumPgConn, topic: Json<Topic>, auth_user: User) -> Json<Topic> {
    let update = create_topic(&conn, topic);
    update
}

#[get("/topics/<topic_id>")]
fn single_topic(conn: TigumPgConn, topic_id: i32, _auth_user: User) -> Json<Topic> {
    let topic = get_topic(&conn, topic_id);
    return topic;
}

#[post("/topics", format = "application/json", data = "<topic_ids>")]
fn topics(conn: TigumPgConn, topic_ids: Json<TopicIds>, _auth_user: User) -> Json<Vec<Topic>> {
    let result_topic = get_topics(&conn, topic_ids);
    return result_topic;
}

// CORS Prelight Request Handler

#[route(OPTIONS, path = "/")]
fn preflight_handler() {
    println!("{}", String::from("Handling preflight"))
}

fn main() {

    rocket::ignite()
        .mount("/", routes![topics, single_topic, create_single_topic, update_single_topic, delete_single_topic, notes, single_note, create_single_note, preflight_handler])
        .attach(cors::CorsFairing)
        .attach(TigumPgConn::fairing())
        .launch();
}
