use crate::db::querys::TigumPgConn;
use crate::db::api_response::ApiResponse;
use rocket::http::Status;
use crate::db::parsing_util::{row_to_user, parse_topic_result};


pub async fn get_public_topics_for_user(conn: &TigumPgConn, user_name: String) -> ApiResponse {

    // CHECK IF USER EXISTS
    let check_for_user = conn.run(move |c|
        c.query(
            "SELECT * FROM users WHERE name = $1",
            &[&user_name]
        )
    ).await;

    if let Ok(result) = check_for_user {
        if let Some(user) = result.get(0) {
            let parsed_user = row_to_user(user);
            // GET TOPICS FOR USER
            let get_user_public_topics = conn.run(move |c|
                c.query(
                    "SELECT * FROM topics WHERE user_id = $1 AND published = true",
                    &[&parsed_user.id]
                )
            ).await;
            if let Ok(results) = get_user_public_topics {
                let parsed_topics = parse_topic_result(results);
                ApiResponse {
                    status: Status::raw(200),
                    json: json!({ "topics": parsed_topics })
                }
            } else {
                ApiResponse {
                    status: Status::raw(500),
                    json: json!({ "error": "Something went wrong" })
                }
            }
        } else {
            ApiResponse {
                status: Status::raw(404),
                json: json!({ "msg": "Failed to find user" })
            }
        }

    } else {
        ApiResponse {
            status: Status::raw(404),
            json: json!({ "msg": "Failed to find user" })
        }
    }
    
}

pub async fn get_public_notes_in_topic(conn: &TigumPgConn, topic_id: i32) {
    let public_notes = conn.run(move |c|
        c.query(
            "SELECT * FROM notes WHERE topic_id = $1 AND published = true",
            &[&topic_id]
        )
    ).await;
    
    println!("{:?}", public_notes)
}
