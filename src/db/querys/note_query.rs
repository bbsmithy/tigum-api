//Use Macros
use rocket_contrib::json::Json;
use rocket::http::Status;

use crate::db::models;
use crate::db::querys::TigumPgConn;
use crate::db::querys::topic_query::{remove_from_topic_resource_list, add_to_topic_resource_list};
use crate::db::models::resources::ResourceType;
use crate::db::api_response::ApiResponse;

use models::resources::note::{NewNote, Note, NoteIds};


fn row_to_note(row: rocket_contrib::databases::postgres::rows::Row) -> Note {
    Note::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4))
}

fn parse_note_result(query_result: rocket_contrib::databases::postgres::rows::Rows) -> Vec<Note> {
    let mut results: Vec<Note> = vec![];
    for row in query_result.iter() {
       
        let note = Note::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4));
        println!("{:?}", note.date_created);
        results.push(note);
    }
    results
}

pub fn delete_note(conn: &TigumPgConn, note_id: i32, user_id: i32) -> ApiResponse {
    let result = conn.query("DELETE FROM notes WHERE id = $1 AND user_id = $2 RETURNING topic_id", &[&note_id, &user_id]);
    match result {
        Ok(row) => {
            let result_row = row.get(0);
            let topic_id = result_row.get(0);
            let remove_from_topic_result = remove_from_topic_resource_list(conn, topic_id, note_id, ResourceType::Note);
            match remove_from_topic_result {
                Ok(_rows_removed) => {
                    ApiResponse {
                        json: json!({ "msg": format!("Successfully deleted note with id {}", note_id) }),
                        status: Status::raw(200)
                    }
                },
                Err(_err) => {
                    ApiResponse {
                        json: json!({ "error": format!("Failed to delete note with id: {}", note_id) }),
                        status: Status::raw(500)
                    }
                }
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": format!("Failed to delete note with id: {}", note_id) }),
                status: Status::raw(500)
            }
        }
    }
}

pub fn update_note(conn: &TigumPgConn, note_id: i32, note: Json<Note>, user_id: i32) -> ApiResponse {
    let query_result = conn.query(
        "UPDATE notes SET title = ($2) WHERE id = ($1) AND user_id = $3 RETURNING *",
        &[&note_id, &note.title, &user_id],
    );
    match query_result {
        Ok(rows) => {
            let updated_note = row_to_note(rows.get(0));
            ApiResponse {
                json: json!(updated_note),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({"error": format!("Could not update note with id {}", note_id)}),
                status: Status::raw(500)
            }
        }
    }
}

pub fn get_notes(conn: &TigumPgConn, note_ids: Json<NoteIds>, user_id: i32) -> ApiResponse {
    let query_result = conn.query("SELECT * FROM notes WHERE id = ANY($1) AND user_id = $2 ORDER BY date_created ASC", &[&note_ids.ids, &user_id]);
    match query_result {
        Ok(rows) => {
            let results = parse_note_result(rows);
            ApiResponse {
                json: json!(results),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({"error": format!("Could not get notes with ids {:?}", note_ids.ids)}),
                status: Status::raw(500)
            }
        }
    }
}

pub fn get_note(conn: &TigumPgConn, note_id: i32) -> ApiResponse {
    let query_result = conn.query("SELECT * FROM notes WHERE id = $1", &[&note_id]);
    match query_result {
        Ok(rows) => {
            let note = row_to_note(rows.get(0));
            ApiResponse {
                json: json!(note),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": format!("Could not get note with id {}", note_id) }),
                status: Status::raw(500)
            }
        }
    }
}

pub fn create_note(conn: &TigumPgConn, note: &Json<NewNote>, user_id: i32) -> ApiResponse {
    let query_result = conn.query(
        "INSERT INTO notes (title, topic_id, user_id) VALUES ($1, $2, $3) RETURNING *",
        &[&note.title, &note.topic_id, &user_id]
    );
    match query_result {
        Ok(result_rows) => {
            let row = result_rows.get(0);
            let new_note = Note::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4));
            let add_to_topic_result = add_to_topic_resource_list(&conn, note.topic_id, new_note.id, ResourceType::Note);
            match add_to_topic_result {
                Ok(_rows_updated) => {
                    ApiResponse { 
                        json: json!(new_note), 
                        status: Status::raw(200),
                    } 
                },
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
