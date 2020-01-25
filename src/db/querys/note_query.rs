//Use Macros
use rocket_contrib::json::Json;

use crate::db::models;
use crate::db::querys::TigumPgConn;

use models::resources::note::{NewNote, Note, NoteIds};

fn parse_note_result(query_result: rocket_contrib::databases::postgres::rows::Rows) -> Vec<Note> {
    let mut results: Vec<Note> = vec![];
    println!("{:?}", query_result);
    for row in query_result.iter() {
        let note = Note::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4));
        results.push(note);
    }
    results
}

pub fn delete_note(conn: &TigumPgConn, note_id: i32, user_id: i32) -> Json<String> {
    let result = conn
        .execute("DELETE FROM notes WHERE id = $1 AND user_id = $2", &[&note_id, &user_id])
        .unwrap();
    Json(format!("{} rows deleted", result))
}

pub fn update_note(conn: &TigumPgConn, note_id: i32, note: Json<Note>, user_id: i32) -> Json<Note> {
    conn.execute(
        "UPDATE notes SET title = ($2) WHERE id = ($1) AND user_id = $3",
        &[&note_id, &note.title, &user_id],
    )
    .unwrap();
    get_note(&conn, note_id)
}

pub fn get_notes(conn: &TigumPgConn, note_ids: Json<NoteIds>, user_id: i32) -> Json<Vec<Note>> {
    let query_result = conn
        .query("SELECT * FROM notes WHERE id = ANY($1) AND user_id = $2", &[&note_ids.ids, &user_id])
        .unwrap();
    let results = parse_note_result(query_result);
    Json(results)
}

pub fn get_note(conn: &TigumPgConn, note_id: i32) -> Json<Note> {
    let query_result = conn
        .query("SELECT * FROM notes WHERE id = $1", &[&note_id])
        .unwrap();
    let note = query_result.get(0);
    let note_response = Note::new(
        note.get(0),
        note.get(1),
        note.get(2),
        note.get(3),
        note.get(4),
    );
    Json(note_response)
}

pub fn create_note(conn: &TigumPgConn, note: &Json<NewNote>, user_id: i32) -> Json<Note> {
    let inserted_rows = conn
        .query(
            "INSERT INTO notes (title, topic_id, user_id) VALUES ($1, $2, $3) RETURNING *",
            &[&note.title, &note.topic_id, &user_id],
        )
        .unwrap();

    let row = inserted_rows.get(0);
    let note_response = Note::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4));

    Json(note_response)
}
