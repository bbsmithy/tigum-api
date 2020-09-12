//Use Macros
use rocket_contrib::json::Json;
use rocket::http::Status;

use crate::db::models;
use crate::db::querys::TigumPgConn;
use crate::db::api_response::ApiResponse;
use crate::db::querys::topic_query::{add_to_topic_resource_list, remove_from_topic_resource_list};


use models::resources::video::{NewVideo, Video};
use models::resources::ResourceType;
use models::Ids;

fn row_to_video(row: rocket_contrib::databases::postgres::rows::Row) -> Video {
    Video {
        id: row.get(0),
        topic_id: row.get(6),
        user_id: row.get(7),
        title: row.get(1),
        iframe: row.get(2),
        origin: row.get(3),
        date_created: row.get(4),
        thumbnail_img: row.get(5),
    }
}

pub fn delete_video(conn: &TigumPgConn, id: i32, user_id: i32) -> ApiResponse {
    let query_result = conn.query("DELETE FROM videos WHERE id = $1 AND user_id = $2 RETURNING topic_id", &[&id, &user_id]);
    match query_result {
        Ok(rows_removed) => {
            let topic_id = rows_removed.get(0).get(0);
            match remove_from_topic_resource_list(conn, topic_id, id, ResourceType::Video) {
                Ok(_removed_row_count)=> ApiResponse {
                    json: json!({ "msg": format!("Successfully deleted video with id: {}", id) }),
                    status: Status::raw(200)
                },
                Err(_error) => ApiResponse {
                    json: json!({ "error": format!("Successfully deleted video with id: {}", id) }),
                    status: Status::raw(200)
                }
            }
        },
        Err(_err) => ApiResponse {
            json: json!({ "error": format!("Successfully deleted video with id: {}", id) }),
            status: Status::raw(200)
        }
    }
}

pub fn update_video(conn: &TigumPgConn, id: i32, video: Json<NewVideo>, user_id: i32) -> ApiResponse {
    let query_result = conn.query(
        "UPDATE videos SET topic_id = $2, user_id = $3, title = $4, iframe = $5, origin = $6, thumbnail_img = $7 WHERE id = $1 RETURNING *",
        &[&id, &video.topic_id, &user_id, &video.title, &video.iframe, &video.origin, &video.thumbnail_img],
    );
    match query_result {
        Ok(updated_rows) => {
            let updated_video = row_to_video(updated_rows.get(0));
            ApiResponse {
                json: json!(updated_video),
                status: Status::raw(200)
            }
       },
        Err(_err) => {
            ApiResponse {
                json: json!({"error": format!("Could not update video with id {}", id)}),
                status: Status::raw(200)
            }
        }
    }
}

pub fn get_videos(conn: &TigumPgConn, ids: Json<Ids>, user_id: i32) -> ApiResponse {
    let query_result = conn.query("SELECT * FROM videos WHERE id = ANY($1) AND user_id = $2", &[&ids.ids, &user_id]);
    match query_result {
        Ok(rows) => {
            let mut results: Vec<Video> = vec![];
            for row in rows.iter() {
                let video_response = row_to_video(row);
                results.push(video_response);
            }
            ApiResponse {
                json: json!(results),
                status: Status::raw(200)
            }
        },
        Err(_err) => ApiResponse {
            json: json!({ "error": format!("Could not get videos with ids {:?}", ids.ids) }),
            status: Status::raw(200)
        }
    }   
}

pub fn get_video(conn: &TigumPgConn, id: i32, _user_id: i32) -> ApiResponse {
    let query_result = conn.query("SELECT * FROM videos WHERE id = $1", &[&id]);
    match query_result {
        Ok(rows) => {
            let video_response = row_to_video(rows.get(0));
            ApiResponse {
                json: json!(video_response),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": "Could not create video" }),
                status: Status::raw(200)
            }
        }
    }
}

pub fn create_video(conn: &TigumPgConn, video: &Json<NewVideo>, user_id: i32) -> ApiResponse {
    let query_result = conn.query(
            "INSERT INTO videos (topic_id, user_id, title, iframe, origin, thumbnail_img) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            &[
                &video.topic_id,
                &user_id,
                &video.title,
                &video.iframe,
                &video.origin,
                &video.thumbnail_img
            ],
        );
    match query_result {
        Ok(new_video_rows) => {
            let new_row = new_video_rows.get(0);
            let new_video = row_to_video(new_row);
            let query_result = add_to_topic_resource_list(&conn, new_video.topic_id, new_video.id, ResourceType::Video);
            match query_result {
                Ok(_rows_updated) => ApiResponse { json: json!(new_video), status: Status::raw(200) },
                Err(_error) => ApiResponse {
                    json: json!({ "error": format!("Could not create video {}", new_video.topic_id )}),
                    status: Status::raw(500)
                }
            }
        },  
        Err(_error) => ApiResponse {
            json: json!({
                "error": format!("Could not create video {}", video.topic_id )
            }),
            status: Status::raw(500)
        }
    }
}
