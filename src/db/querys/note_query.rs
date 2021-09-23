use diesel::{QueryDsl, RunQueryDsl};
use rocket_contrib::json::Json;
use rocket::http::Status;
use diesel::ExpressionMethods;
use diesel::Connection;
use diesel::result::Error;

use crate::db::querys::TigumPgConn;
use crate::db::querys::topic_query::{remove_from_topic_resource_list, add_to_topic_resource_list, update_topic_mod_date};
use crate::db::models::resources::ResourceType;
use crate::db::models::resources::note::{Note, NewNote, NoteIds};
use crate::db::api_response::ApiResponse;
use crate::schema::notes::dsl::*;


pub fn delete_note(conn: &TigumPgConn, note_id: i32, uid: i32) -> ApiResponse {

    ApiResponse {
        json: json!("All good"),
        status: Status::raw(200)
    }
    // let result = conn.run(move |c|
    //     c.query("DELETE FROM notes WHERE id = $1 AND user_id = $2 RETURNING topic_id", &[&note_id, &user_id])
    // );
    // match result {
    //     Ok(row) => {
    //         let result_row = row.get(0);
    //         if let Some(row) = result_row {
    //             let topic_id = row.get(0);
    //             let remove_from_topic_result = remove_from_topic_resource_list(conn, topic_id, note_id, ResourceType::Note);
    //             match remove_from_topic_result {
    //                 Ok(_rows_removed) => {
    //                     ApiResponse {
    //                         json: json!({ "msg": format!("Successfully deleted note with id {}", note_id) }),
    //                         status: Status::raw(200)
    //                     }
    //                 },
    //                 Err(_err) => {
    //                     ApiResponse {
    //                         json: json!({ "error": format!("Failed to delete note with id: {}", note_id) }),
    //                         status: Status::raw(500)
    //                     }
    //                 }
    //             }
    //         } else {
    //             ApiResponse {
    //                 json: json!({ "error": format!("Failed to delete note") }),
    //                 status: Status::raw(500)
    //             }
    //         }
    //     },
    //     Err(_err) => {
    //         ApiResponse {
    //             json: json!({ "error": format!("Failed to delete note with id: {}", note_id) }),
    //             status: Status::raw(500)
    //         }
    //     }
    // }
}

pub fn update_note(conn: &TigumPgConn, note_id: i32, note: Json<Note>, uid: i32) -> ApiResponse {
    ApiResponse {
        json: json!("All good"),
        status: Status::raw(200)
    }
    // let query_result = conn.run(move |c|
    //     c.query(
    //         "UPDATE notes SET title = ($2) WHERE id = ($1) AND user_id = $3 RETURNING *",
    //         &[&note_id, &note.title, &user_id],
    //     )
    // );
    // match query_result {
    //     Ok(rows) => {
    //         if let Some(row) = rows.get(0) {
    //             let updated_note = row_to_note(row);
    //             match update_topic_mod_date(conn, updated_note.topic_id) {
    //                 Ok(_rows) => {
    //                     ApiResponse {
    //                         json: json!(updated_note),
    //                         status: Status::raw(200)
    //                     }
    //                 },
    //                 Err(err) => {
    //                     println!("{:?}", err);
    //                     ApiResponse {
    //                         json: json!({"error": format!("Could not update note")}),
    //                         status: Status::raw(500)
    //                     }
    //                 }
    //             }
    //         } else {
    //             ApiResponse {
    //                 json: json!({"error": format!("Could not update note")}),
    //                 status: Status::raw(200)
    //             }
    //         }
    //     },
    //     Err(_err) => {
    //         ApiResponse {
    //             json: json!({"error": format!("Could not update note with id {}", note_id)}),
    //             status: Status::raw(500)
    //         }
    //     }
    // }
}

pub fn get_notes(conn: TigumPgConn, note_ids: Json<NoteIds>, uid: i32) -> ApiResponse {
    use crate::schema::notes::dsl::*;
    let query_result = notes.filter(user_id.eq(uid)).get_results::<Note>(&*conn);
    match query_result {
        Ok(rows) => {
            ApiResponse {
                json: json!(rows),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": format!("Could not get notes") }),
                status: Status::raw(500)
            }
        }
    }
}

pub fn get_note(conn: TigumPgConn, note_id: i32) -> ApiResponse {
    use crate::schema::notes::dsl::*;
    let query_result = notes.filter(id.eq(note_id)).get_result::<Note>(&*conn);
    match query_result {
        Ok(row) => {
            ApiResponse {
                json: json!(row),
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

pub fn create_note(conn: &diesel::PgConnection, note: Json<NewNote>, uid: i32) -> ApiResponse {

    let note_title = note.title.clone();

    // TODO use transactions when 2 queries should both happend for success
    let transaction_result = conn.transaction::<Note, Error, _>(|| {
        let new_note = diesel::insert_into(notes).values((
            title.eq(note_title), 
            topic_id.eq(note.topic_id), 
            user_id.eq(uid)
        )).get_result::<Note>(conn)?;
        add_to_topic_resource_list(
            conn, 
            note.topic_id, 
            new_note.id, 
            ResourceType::Note
        )?;
        Ok(new_note)
    });

    match transaction_result {
        Ok(new_note) => {
            ApiResponse {
                json: json!(new_note),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": format!("Could not create note") }),
                status: Status::raw(500)
            }
        }
    }
}

pub fn update_note_mod_date(conn: &TigumPgConn, note_id: i32) -> ApiResponse {
    ApiResponse {
        json: json!("All good"),
        status: Status::raw(200)
    }
//     let note_mod_date_updated_result = conn.run(move |c|
//         c.query(
//             "UPDATE notes SET date_updated = CURRENT_TIMESTAMP WHERE id = $1",
//             &[&note_id],
//         )
//     );
//     match note_mod_date_updated_result {
//         Ok(_mod_row) => {
//             ApiResponse {
//                 status: Status::raw(200),
//                 json: json!({ "res": format!("success") })
//             }
//         },
//         Err(_err) => {
//             ApiResponse {
//                 status: Status::raw(500),
//                 json: json!({ "error": format!("Could not update note") })
//             }
//         }
//     }
}
