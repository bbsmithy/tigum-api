use crate::db;
use rocket::Route;

//Use Macros
use rocket_contrib::json::Json;

// Models
use db::models::resources::note::{NewNote, Note, NoteIds};
use db::models::user::User;

use db::api_response::ApiResponse;

// Querys
use db::querys::note_query::{create_note, delete_note, get_note, get_notes, update_note, update_note_mod_date};
use db::querys::TigumPgConn;

/////////////////////
//// NOTE ROUTES ////
/////////////////////


#[put("/notes/updated-note/<note_id>")]
fn update_note_mod(conn: TigumPgConn, note_id: i32) -> ApiResponse {
    update_note_mod_date(&*conn, note_id)
}

#[delete("/notes/<note_id>")]
fn delete_single_note(conn: TigumPgConn, note_id: i32, auth_user: User) -> ApiResponse {
    delete_note(&*conn, note_id, auth_user.id)
}

#[put("/notes/<note_id>", format = "application/json", data = "<note>")]
fn update_single_note(conn: TigumPgConn, note_id: i32, note: Json<Note>, auth_user: User) -> ApiResponse {
    update_note(&*conn, note_id, note, auth_user.id)
}

#[post("/notes/create-note", format = "application/json", data = "<note>")]
fn create_single_note(conn: TigumPgConn, note: Json<NewNote>, auth_user: User) -> ApiResponse {
    create_note(&*conn, note, auth_user.id)
}

#[get("/notes/<note_id>")]
fn single_note(conn: TigumPgConn, note_id: i32, _auth_user: User) -> ApiResponse {
    get_note(&*conn, note_id)
}

#[post("/notes", format = "application/json", data = "<ids>")]
fn notes(conn: TigumPgConn, ids: Json<NoteIds>, auth_user: User) -> ApiResponse {
    get_notes(&*conn, ids, auth_user.id)
}

pub fn get_note_routes() -> Vec<Route> {
    routes![
        notes,
        single_note,
        create_single_note,
        update_single_note,
        delete_single_note,
        update_note_mod
    ]
}
