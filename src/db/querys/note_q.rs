//Use Macros
use rocket_contrib::json::Json;

use crate::db::models;
use crate::db::querys::TigumPgConn;

use models::resources::note::{Note, NoteIds, NewNote};
use models::Id;

fn parse_note_result(query_result: rocket_contrib::databases::postgres::rows::Rows) -> Vec<Note> {
    let mut results: Vec<Note> = vec![];
    for row in query_result.iter() {
        let note = Note::new(row.get(1), row.get(0), row.get(2), row.get(3));
        results.push(note);
    }
    results
}

pub fn delete_note(conn: &TigumPgConn, note_id: i32) -> Json<String> {
    let result = conn
        .execute("DELETE FROM notes WHERE id = $1", &[&note_id])
        .unwrap();
    Json(format!("{} rows deleted", result))
}

pub fn update_note(conn: &TigumPgConn, note_id: i32, note: Json<Note>) -> Json<Note> {
    conn.execute(
        "UPDATE notes SET title = ($2), note_content = ($3) WHERE id = ($1)",
        &[&note_id, &note.title, &note.note_content],
    )
    .unwrap();
    get_note(&conn, note_id)
}

pub fn get_notes(conn: &TigumPgConn, note_ids: Json<NoteIds>) -> Json<Vec<Note>> {
    let query_result = conn
        .query("SELECT * FROM notes WHERE id = ANY($1)", &[&note_ids.ids])
        .unwrap();
    let results = parse_note_result(query_result);
    Json(results)
}

pub fn get_note(conn: &TigumPgConn, note_id: i32) -> Json<Note> {
    let query_result = conn
        .query("SELECT * FROM notes WHERE id = $1", &[&note_id])
        .unwrap();
    let note = query_result.get(0);
    let note_response = Note::new(note.get(1), note.get(0), note.get(2), note.get(3));
    Json(note_response)
}

pub fn create_note(conn: &TigumPgConn, note: Json<NewNote>) -> Json<Id> {
    let inserted_rows = conn
        .query(
            "INSERT INTO notes (title, note_content) VALUES ($1, $2) RETURNING id",
            &[&note.title, &note.note_content],
        )
        .unwrap();

    let row = inserted_rows.get(0);
    let note_id: i32 = row.get(0);

    let id_response = Id { id: note_id };

    Json(id_response)
}