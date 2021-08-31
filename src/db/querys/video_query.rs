//Use Macros
use rocket_contrib::json::Json;
use rocket::http::Status;

use crate::db::models;
use crate::db::querys::TigumPgConn;
use crate::db::api_response::ApiResponse;
use crate::db::querys::topic_query::{
    add_to_topic_resource_list,
    remove_from_topic_resource_list, 
    update_topic_mod_date
};
use crate::db::parsing_util::row_to_video;
use models::resources::video::{NewVideo, Video};
use models::resources::ResourceType;
use models::Ids;

pub async fn delete_video(conn: &TigumPgConn, id: i32, user_id: i32) -> ApiResponse {
    let query_result = conn.run(move |c|
        c.query("DELETE FROM videos WHERE id = $1 AND user_id = $2 RETURNING topic_id", &[&id, &user_id])
    ).await;
    match query_result {
        Ok(rows_removed) => {
            if let Some(topic_row) = rows_removed.get(0) {
                let topic_id = topic_row.get(0);
                match remove_from_topic_resource_list(conn, topic_id, id, ResourceType::Video).await {
                    Ok(_removed_row_count)=> ApiResponse {
                        json: json!({ "msg": format!("Successfully deleted video with id: {}", id) }),
                        status: Status::raw(200)
                    },
                    Err(_error) => ApiResponse {
                        json: json!({ "error": format!("Successfully deleted video with id: {}", id) }),
                        status: Status::raw(200)
                    }
                }
            } else {
                ApiResponse {
                    json: json!({ "error": format!("Failed to delete video with id: {}", id) }),
                    status: Status::raw(500)
                }
            }
        },
        Err(_err) => ApiResponse {
            json: json!({ "error": format!("Failed to delete video with id: {}", id) }),
            status: Status::raw(500)
        }
    }
}

pub async fn update_video(conn: &TigumPgConn, id: i32, video: Json<NewVideo>, user_id: i32) -> ApiResponse {
    let query_result = conn.run(move |c|
        c.query("UPDATE videos SET topic_id = $2, user_id = $3, title = $4, iframe = $5, origin = $6, thumbnail_img = $7 WHERE id = $1 RETURNING *",
        &[&id, &video.topic_id, &user_id, &video.title, &video.iframe, &video.origin, &video.thumbnail_img],
    )).await;
    match query_result {
        Ok(updated_rows) => {
            if let Some(video_row) = updated_rows.get(0) {
                let updated_video = row_to_video(video_row);
                update_topic_from_video(&conn, updated_video).await
            } else {
                ApiResponse {
                    json: json!({"error": format!("Could not update video with id {}", id)}),
                    status: Status::raw(500)
                }
            }
       },
        Err(_err) => {
            ApiResponse {
                json: json!({"error": format!("Could not update video with id {}", id)}),
                status: Status::raw(500)
            }
        }
    }
}

pub async fn get_videos(conn: &TigumPgConn, ids: Json<Ids>, user_id: i32) -> ApiResponse {
    let query_result = conn.run(move |c|
        c.query("SELECT * FROM videos WHERE id = ANY($1) AND user_id = $2 ORDER BY date_updated DESC", &[&ids.ids, &user_id])
    ).await;
    match query_result {
        Ok(rows) => {
            let mut results: Vec<Video> = vec![];
            for row in rows.iter() {
                println!("{:?}", row);
                let video_response = row_to_video(row);
                results.push(video_response);
            }
            ApiResponse {
                json: json!(results),
                status: Status::raw(200)
            }
        },
        Err(_err) => ApiResponse {
            json: json!({ "error": format!("Could not get videos") }),
            status: Status::raw(200)
        }
    }   
}

pub async fn get_video(conn: &TigumPgConn, id: i32, _user_id: i32) -> ApiResponse {
    let query_result = conn.run(move |c|
        c.query("SELECT * FROM videos WHERE id = $1", &[&id])
    ).await;
    match query_result {
        Ok(rows) => {
            if let Some(row) = rows.get(0) {
                let video_response = row_to_video(row);
                ApiResponse {
                    json: json!(video_response),
                    status: Status::raw(200)
                }
            } else {
                ApiResponse {
                    json: json!({ "error": "Could not find video response" }),
                    status: Status::raw(500)
                }
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": "Could not create video" }),
                status: Status::raw(500)
            }
        }
    }
}

pub async fn create_video(conn: &TigumPgConn, video: Json<NewVideo>, user_id: i32) -> ApiResponse {
    let query_result = conn.run(move |c|
        c.query("INSERT INTO videos (topic_id, user_id, title, iframe, origin, thumbnail_img) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            &[
                &video.topic_id,
                &user_id,
                &video.title,
                &video.iframe,
                &video.origin,
                &video.thumbnail_img
            ])
        ).await;
    match query_result {
        Ok(new_video_rows) => {
            if let Some(new_row) = new_video_rows.get(0) {
                let new_video = row_to_video(new_row);
                let query_result = add_to_topic_resource_list(&conn, new_video.topic_id, new_video.id, ResourceType::Video).await;
                match query_result {
                    Ok(_rows_updated) => update_topic_from_video(&conn, new_video).await,
                    Err(_error) => ApiResponse {
                        json: json!({ "error": format!("Could not create video {}", new_video.topic_id )}),
                        status: Status::raw(500)
                    }
                }
                
            } else {
                ApiResponse {
                    json: json!({ "error": format!("Could not create video") }),
                    status: Status::raw(500)
                }
            }
        },  
        Err(_error) => ApiResponse {
            json: json!({
                "error": format!("Could not create video")
            }),
            status: Status::raw(500)
        }
    }
}


async fn update_topic_from_video(conn: &TigumPgConn, video: Video) -> ApiResponse {
    match update_topic_mod_date(conn, video.topic_id).await {
        Ok(_rows) => {
            ApiResponse {
                json: json!(video),
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
}
