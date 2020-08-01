use crate::db;
use rocket::Route;
use rocket::http::Status;

//Use Macros
use rocket_contrib::json::Json;

// Models
use db::models::resources::note::{NewNote, Note, NoteIds};
use db::models::resources::ResourceType;
use db::models::user::User;

use db::api_response::ApiResponse;

// Querys
use db::querys::note_query::{create_note, delete_note, get_note, get_notes, update_note};
use db::querys::topic_query::add_to_topic_resource_list;
use db::querys::TigumPgConn;

/////////////////////
//// NOTE ROUTES ////
/////////////////////

#[delete("/notes/<note_id>")]
fn delete_single_note(conn: TigumPgConn, note_id: i32, auth_user: User) -> Json<String> {
    delete_note(&conn, note_id, auth_user.id)
}

#[put("/notes/<note_id>", format = "application/json", data = "<note>")]
fn update_single_note(conn: TigumPgConn, note_id: i32, note: Json<Note>, auth_user: User) -> Json<Note> {
    update_note(&conn, note_id, note, auth_user.id)
}

#[post("/notes/create-note", format = "application/json", data = "<note>")]
fn create_single_note(conn: TigumPgConn, note: Json<NewNote>, auth_user: User) -> ApiResponse {
    let new_note_query_result = create_note(&conn, &note, auth_user.id);
    match new_note_query_result {
        Ok(new_note) => {
            let query_result = add_to_topic_resource_list(&conn, note.topic_id, new_note.id, ResourceType::Note);
            match query_result {
                Ok(_rows_updated) => ApiResponse { json: json!(new_note), status: Status::raw(200) },
                Err(_error) => ApiResponse {
                    json: json!({ "error": format!("Could not create note {}", new_note.topic_id )}),
                    status: Status::raw(500)
                }
            }
        },  
        Err(_error) => ApiResponse {
            json: json!({
                "error": format!("Could not create snippet {}", note.topic_id )
            }),
            status: Status::raw(500)
        }
    }
}

#[get("/notes/<note_id>")]
fn single_note(conn: TigumPgConn, note_id: i32, _auth_user: User) -> Json<Note> {
    get_note(&conn, note_id)
}

#[post("/notes", format = "application/json", data = "<note_ids>")]
fn notes(conn: TigumPgConn, note_ids: Json<NoteIds>, auth_user: User) -> Json<Vec<Note>> {
    get_notes(&conn, note_ids, auth_user.id)
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
