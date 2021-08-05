//Use Macros
use rocket_contrib::json::Json;
use rocket::http::Status;
use crate::db::querys::TigumPgConn;
use crate::db::querys::topic_query::{remove_from_topic_resource_list, add_to_topic_resource_list, update_topic_mod_date};
use crate::db::models::resources::ResourceType;
use crate::db::api_response::ApiResponse;
use crate::db::parsing_util::{row_to_note, parse_note_result};


pub async fn delete_note(conn: &TigumPgConn, note_id: i32, user_id: i32) -> ApiResponse {
    let result = conn.run(move |c|
        c.query("DELETE FROM notes WHERE id = $1 AND user_id = $2 RETURNING topic_id", &[&note_id, &user_id])
    ).await;
    match result {
        Ok(row) => {
            let result_row = row.get(0);
            if let Some(row) = result_row {
                let topic_id = row.get(0);
                let remove_from_topic_result = remove_from_topic_resource_list(conn, topic_id, note_id, ResourceType::Note).await;
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
            } else {
                ApiResponse {
                    json: json!({ "error": format!("Failed to delete note") }),
                    status: Status::raw(500)
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

pub async fn update_note(conn: &TigumPgConn, note_id: i32, note: Json<Note>, user_id: i32) -> ApiResponse {
    let query_result = conn.run(move |c|
        c.query(
            "UPDATE notes SET title = ($2) WHERE id = ($1) AND user_id = $3 RETURNING *",
            &[&note_id, &note.title, &user_id],
        )
    ).await;
    match query_result {
        Ok(rows) => {
            if let Some(row) = rows.get(0) {
                let updated_note = row_to_note(row);
                match update_topic_mod_date(conn, updated_note.topic_id).await {
                    Ok(_rows) => {
                        ApiResponse {
                            json: json!(updated_note),
                            status: Status::raw(200)
                        }
                    },
                    Err(err) => {
                        println!("{:?}", err);
                        ApiResponse {
                            json: json!({"error": format!("Could not update note")}),
                            status: Status::raw(500)
                        }
                    }
                }
            } else {
                ApiResponse {
                    json: json!({"error": format!("Could not update note")}),
                    status: Status::raw(200)
                }
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

pub async fn get_notes(conn: &TigumPgConn, note_ids: Json<NoteIds>, user_id: i32) -> ApiResponse {
    let query_result = conn.run(move |c|
        c.query("SELECT * FROM notes WHERE id = ANY($1) AND user_id = $2 ORDER BY date_updated ASC", &[&note_ids.ids, &user_id])
    ).await;
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
                json: json!({"error": format!("Could not get notes")}),
                status: Status::raw(500)
            }
        }
    }
}

pub async fn get_note(conn: &TigumPgConn, note_id: i32) -> ApiResponse {
    let query_result = conn.run(move |c|
        c.query("SELECT * FROM notes WHERE id = $1", &[&note_id])
    ).await;
    match query_result {
        Ok(rows) => {
            if let Some(row) = rows.get(0) {
                let note = row_to_note(row);
                ApiResponse {
                    json: json!(note),
                    status: Status::raw(200)
                }
            } else {
                ApiResponse {
                    json: json!({ "error": format!("Could not get note with id {}", note_id) }),
                    status: Status::raw(200)
                }
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

pub async fn create_note(conn: &TigumPgConn, note: Json<NewNote>, user_id: i32) -> ApiResponse {
    let topic_id = note.topic_id;
    let query_result = conn.run(move |c|
        c.query(
            "INSERT INTO notes (title, topic_id, user_id) VALUES ($1, $2, $3) RETURNING *",
            &[&note.title, &note.topic_id, &user_id]
        )
    ).await;
    match query_result {
        Ok(result_rows) => {
            let result = result_rows.get(0);
            if let Some(row) = result {
                let new_note = Note::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4), row.get(5));
                let add_to_topic_result = add_to_topic_resource_list(&conn, topic_id, new_note.id, ResourceType::Note).await;
                match add_to_topic_result {
                    Ok(_rows_updated) => match update_topic_mod_date(conn, new_note.topic_id).await {
                        Ok(_rows) => {
                            ApiResponse {
                                json: json!(new_note),
                                status: Status::raw(200)
                            }
                        },
                        Err(err) => {
                            ApiResponse {
                                json: json!({"error": format!("Could not update note")}),
                                status: Status::raw(500)
                            }
                        }
                    },
                    Err(_error) => ApiResponse {
                        json: json!({ "error": format!("Could not add note to topic resource list")}),
                        status: Status::raw(500)
                    }
                }
            } else {
                ApiResponse {
                    json: json!({
                        "error": format!("Could not create snippet")
                    }),
                    status: Status::raw(500)
                }
            }
        },
        Err(_error) => ApiResponse {
            json: json!({
                "error": format!("Could not create note")
            }),
            status: Status::raw(500)
        }
    }
}

pub async fn update_note_mod_date(conn: &TigumPgConn, note_id: i32) -> ApiResponse {
    let note_mod_date_updated_result = conn.run(move |c|
        c.query(
            "UPDATE notes SET date_updated = CURRENT_TIMESTAMP WHERE id = $1",
            &[&note_id],
        )
    ).await;
    match note_mod_date_updated_result {
        Ok(_mod_row) => {
            ApiResponse {
                status: Status::raw(200),
                json: json!({ "res": format!("success") })
            }
        },
        Err(_err) => {
            ApiResponse {
                status: Status::raw(500),
                json: json!({ "error": format!("Could not update note") })
            }
        }
    }
}
