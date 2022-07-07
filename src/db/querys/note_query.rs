use diesel::{QueryDsl, RunQueryDsl};
use rocket_contrib::json::Json;
use rocket::http::Status;
use diesel::ExpressionMethods;
use diesel::Connection;
use diesel::result::Error;
use diesel::dsl::any;

use crate::db::models::resources::note::NoteIds;
use crate::db::querys::topic_query::{remove_from_topic_resource_list, add_to_topic_resource_list, update_topic_mod_date};
use crate::db::models::resources::ResourceType;
use crate::db::models::resources::note::{Note, NewNote};
use crate::db::api_response::ApiResponse;
use crate::schema::notes::dsl::*;



pub fn delete_note(conn: &diesel::PgConnection, note_id: i32, uid: i32) -> ApiResponse {
    let note_to_delete_query = notes.filter(id.eq(note_id)).filter(user_id.eq(uid));
    // TODO use transactions when 2 queries should both happend for success
    let transaction_result = conn.transaction::<_, Error, _>(|| {
        let res = diesel::delete(note_to_delete_query).get_result::<Note>(conn)?;
        remove_from_topic_resource_list(conn, res.topic_id, res.id, ResourceType::Note)?;
        Ok(())
    });
    if transaction_result.is_ok() {
        ApiResponse {
            json: json!({ "msg": format!("Successfully deleted note with id {}", note_id) }),
            status: Status::raw(200)
        }
    } else {
        ApiResponse {
            json: json!({ "error": format!("Failed to delete note with id: {}", note_id) }),
            status: Status::raw(500)
        } 
    }
}

pub fn update_note(conn: &diesel::PgConnection, note_id: i32, note: Json<Note>, uid: i32) -> ApiResponse {
    let note_title = note.title.clone();
    let note_to_update_query = notes.filter(id.eq(note_id)).filter(user_id.eq(uid));
    let res = diesel::update(note_to_update_query)
    .set(title.eq(note_title))
    .get_result::<Note>(conn);
    match res {
        Ok(row) => {
            update_topic_mod_date(conn, row.topic_id);
            ApiResponse {
                json: json!(row),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": format!("Could update note") }),
                status: Status::raw(200)
            }
        }
    }
}

pub fn get_notes(conn: &diesel::PgConnection, note_ids: Json<NoteIds>, uid: i32) -> ApiResponse {
    use crate::schema::notes::dsl::*;
    let ids = note_ids.ids.clone();
    let query_result = notes.filter(id.eq(any(ids))).filter(user_id.eq(uid)).order_by(date_updated.asc()).get_results::<Note>(conn);
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

pub fn get_note(conn: &diesel::PgConnection, note_id: i32) -> ApiResponse {
    use crate::schema::notes::dsl::*;
    let query_result = notes.filter(id.eq(note_id)).get_result::<Note>(conn);
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
            update_topic_mod_date(conn,  new_note.topic_id);
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


pub fn publish_note(conn: &diesel::PgConnection, note_id: i32, uid:i32) -> ApiResponse {
    ApiResponse { json: json!("TEST"), status: Status::raw(200) }
}

pub fn update_note_mod_date(conn: &diesel::PgConnection, note_id: i32) -> ApiResponse {
    use crate::schema::notes::dsl::*;
    use diesel::dsl::now;
    let update = diesel::update(notes.filter(id.eq(note_id))).set(
        date_updated.eq(now)
    ).get_result::<Note>(conn);
    match update {
        Ok(row) => {
            ApiResponse {
                json: json!(row),
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
