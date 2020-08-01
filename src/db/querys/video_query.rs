//Use Macros
use rocket_contrib::json::Json;
use rocket_contrib::databases::postgres::Error;

use crate::db::models;
use crate::db::querys::TigumPgConn;

use models::resources::video::{NewVideo, Video};
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

pub fn delete_video(conn: &TigumPgConn, id: i32, user_id: i32) -> Json<String> {
    let update = conn
        .execute("DELETE FROM videos WHERE id = $1 AND user_id = $2", &[&id, &user_id])
        .unwrap();
    Json(format!("{} rows affected", update))
}

pub fn update_video(conn: &TigumPgConn, id: i32, video: Json<NewVideo>, user_id: i32) -> Json<Video> {
    let updated_rows = conn.query(
        "UPDATE videos SET topic_id = $2, user_id = $3, title = $4, iframe = $5, origin = $6, thumbnail_img = $7 WHERE id = $1 RETURNING *",
        &[&id, &video.topic_id, &user_id, &video.title, &video.iframe, &video.origin, &video.thumbnail_img],
    ).unwrap();

    let video_response = row_to_video(updated_rows.get(0));

    Json(video_response)
}

pub fn get_videos(conn: &TigumPgConn, ids: Json<Ids>, user_id: i32) -> Json<Vec<Video>> {
    let query_result = conn
        .query("SELECT * FROM videos WHERE id = ANY($1) AND user_id = $2", &[&ids.ids, &user_id])
        .unwrap();
    let mut results: Vec<Video> = vec![];
    for row in query_result.iter() {
        let video_response = row_to_video(row);
        results.push(video_response);
    }
    Json(results)
}

pub fn get_video(conn: &TigumPgConn, id: i32, _user_id: i32) -> Json<Video> {
    let query_result = conn
        .query("SELECT * FROM videos WHERE id = $1", &[&id])
        .unwrap();
    let video_response = row_to_video(query_result.get(0));
    Json(video_response)
}

pub fn create_video(conn: &TigumPgConn, video: &Json<NewVideo>, user_id: i32) -> Result<Video, Error> {
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
            Ok(row_to_video(new_row))
        },
        Err(error) => Err(error)
    }
}
