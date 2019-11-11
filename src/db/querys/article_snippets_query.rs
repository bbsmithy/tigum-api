//Use Macros
use rocket_contrib::json::Json;

use crate::util::date::create_fake_date;

use crate::db::models;
use crate::db::querys::TigumPgConn;

use models::resources::article_snippets::{ArticleSnippet, NewArticleSnippet};
use models::{Id, Ids};

fn row_to_article_snippet(row: rocket_contrib::databases::postgres::rows::Row) -> ArticleSnippet {
    println!("{:#?}", row);
    ArticleSnippet {
        id: 42,
        topic_id: 78,
        user_id: 1234,
        content: "Hello name".to_string(),
        origin: "orgin".to_string(),
        date_created: create_fake_date(),
    }
}

// pub fn delete_video(conn: &TigumPgConn, id: i32) -> Json<String> {
//     let update = conn
//         .execute("DELETE FROM videos WHERE id = $1", &[&id])
//         .unwrap();
//     Json(format!("{} rows affected", update))
// }

// pub fn update_video(conn: &TigumPgConn, id: i32, video: Json<NewVideo>) -> Json<Video> {
//     let updated_rows = conn.query(
//         "UPDATE videos SET topic_id = $2, user_id = $3, title = $4, iframe = $5, origin = $6, thumbnail_img = $7 WHERE id = $1 RETURNING *",
//         &[&id, &video.topic_id, &video.user_id, &video.title, &video.iframe, &video.origin, &video.thumbnail_img],
//     ).unwrap();

//     let video_response = row_to_video(updated_rows.get(0));

//     Json(video_response)
// }

pub fn get_article_snippets(conn: &TigumPgConn, ids: Json<Ids>) -> Json<Vec<ArticleSnippet>> {
    println!("{:?}", ids);
    let query_result = conn
        .query(
            "SELECT * FROM article_snippets WHERE id = ANY($1)",
            &[&ids.ids],
        )
        .unwrap();
    let mut results: Vec<ArticleSnippet> = vec![];
    for row in query_result.iter() {
        let article_snippet_response = row_to_article_snippet(row);
        results.push(article_snippet_response);
    }
    Json(results)
}

// pub fn get_video(conn: &TigumPgConn, id: i32) -> Json<Video> {
//     let query_result = conn
//         .query("SELECT * FROM videos WHERE id = $1", &[&id])
//         .unwrap();
//     let video_response = row_to_video(query_result.get(0));
//     Json(video_response)
// }

pub fn create_article_snippet(
    conn: &TigumPgConn,
    article_snippet: Json<NewArticleSnippet>,
) -> Json<Id> {
    let inserted_row = conn
        .query(
            "INSERT INTO article_snippets (content, origin, topic_id, user_id) VALUES ($1, $2, $3, $4) RETURNING id",
            &[
                &article_snippet.content,
                &article_snippet.origin,
                &article_snippet.topic_id,
                &article_snippet.user_id,
            ],
        )
        .unwrap();
    let row = inserted_row.get(0);
    println!("{:#?}", row);
    let id: i32 = row.get(0);

    let id_response = Id { id: id };

    Json(id_response)
}
