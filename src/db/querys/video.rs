//Use Macros
use chrono::{NaiveDate, NaiveDateTime};
use rocket_contrib::json::Json;

use crate::db::models;
use crate::db::querys::TigumPgConn;

use models::resources::video::{NewVideo, Video};
use models::{Id, Ids};

pub fn delete_video(conn: &TigumPgConn, id: i32) -> Json<String> {
    let update = conn
        .execute("DELETE FROM videos WHERE id = $1", &[&id])
        .unwrap();
    Json(format!("{} rows affected", update))
}

// pub fn update_video(
//     conn: &TigumPgConn,
//     id: i32,
//     video: Json<Video>,
// ) -> Json<Video> {
//     conn.execute(
//         "UPDATE videos SET content = $2 WHERE id = $1",
//         &[&id, &video.content],
//     )
//     .unwrap();
//     get_video(conn, id)
// }

// pub fn get_videos(conn: &TigumPgConn, ids: Json<Ids>) -> Json<Vec<Video>> {
//     let query_result = conn
//         .query(
//             "SELECT * FROM videos WHERE id = ANY($1)",
//             &[&ids.ids],
//         )
//         .unwrap();
//     let mut results: Vec<Video> = vec![];
//     for row in query_result.iter() {
//         let resource = Video {
//             id: row.get(0),
//             date_created: row.get(4),
//             content_type: row.get(1),
//             content: row.get(2),
//             generated_by: row.get(3),
//             thumbnail_img: row.get(6),
//             title: row.get(5),
//         };
//         results.push(resource);
//     }
//     Json(results)
// }

pub fn get_video(conn: &TigumPgConn, id: i32) -> Json<Video> {
    let query_result = conn
        .query("SELECT * FROM videos WHERE id = $1", &[&id])
        .unwrap();
    let row = query_result.get(0);
    let video_response = Video {
        id: row.get(0),
        topic_id: row.get(6),
        user_id: row.get(7),
        title: row.get(1),
        iframe: row.get(2),
        origin: row.get(3),
        date_created: row.get(4),
        thumbnail_img: row.get(5),
    };
    Json(video_response)
}

pub fn create_video(conn: &TigumPgConn, video: Json<NewVideo>) -> Json<Id> {
    let inserted_row = conn
        .query(
            "INSERT INTO videos (topic_id, user_id, title, iframe, origin, thumbnail_img) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
            &[
                &video.topic_id,
                &video.user_id,
                &video.title,
                &video.iframe,
                &video.origin,
                &video.thumbnail_img
            ],
        )
        .unwrap();
    let row = inserted_row.get(0);
    let id: i32 = row.get(0);

    let id_response = Id { id: id };

    Json(id_response)
}
