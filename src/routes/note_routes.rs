use crate::db;
use rocket::Route;

//Use Macros
use rocket_contrib::json::Json;

// Models
use db::models::resources::note::{NewNote, Note, NoteIds};
use db::models::resources::ResourceType;
use db::models::user::User;

// Querys
use db::querys::note_query::{create_note, delete_note, get_note, get_notes, update_note};
use db::querys::topic_query::update_topic_resource_list;
use db::querys::TigumPgConn;

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
fn create_single_note(conn: TigumPgConn, note: Json<NewNote>, _auth_user: User) -> Json<Note> {
    let new_note = create_note(&conn, &note);
    update_topic_resource_list(&conn, note.topic_id, new_note.id, ResourceType::Note);
    new_note
}

#[get("/notes/<note_id>")]
fn single_note(conn: TigumPgConn, note_id: i32, _auth_user: User) -> Json<Note> {
    get_note(&conn, note_id)
}

#[post("/notes", format = "application/json", data = "<note_ids>")]
fn notes(conn: TigumPgConn, note_ids: Json<NoteIds>, _auth_user: User) -> Json<Vec<Note>> {
    get_notes(&conn, note_ids)
}

pub fn get_note_routes() -> Vec<Route> {
    routes![
        notes,
        single_note,
        create_single_note,
        update_single_note,
        delete_single_note
    ]
}
