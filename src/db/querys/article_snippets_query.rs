//Use Macros
use rocket_contrib::json::Json;

use crate::db::models;
use crate::db::querys::TigumPgConn;

use models::resources::article_snippets::{ArticleSnippet, NewArticleSnippet};
use models::Ids;

fn row_to_article_snippet(row: rocket_contrib::databases::postgres::rows::Row) -> ArticleSnippet {
    ArticleSnippet {
        id: row.get(0),
        topic_id: row.get(4),
        user_id: row.get(5),
        content: row.get(1),
        origin: row.get(2),
        date_created: row.get(3),
    }
}

pub fn delete_article_snippet(conn: &TigumPgConn, id: i32) -> Json<String> {
    let update = conn
        .execute("DELETE FROM article_snippets WHERE id = $1", &[&id])
        .unwrap();
    Json(format!("{} rows affected", update))
}

pub fn update_article_snippet(
    conn: &TigumPgConn,
    id: i32,
    article_snippet: Json<NewArticleSnippet>,
) -> Json<ArticleSnippet> {
    let updated_rows = conn.query(
        "UPDATE article_snippets SET topic_id = $2, user_id = $3, content = $4, origin = $5 WHERE id = $1 RETURNING *",
        &[&id, &article_snippet.topic_id, &article_snippet.user_id, &article_snippet.content, &article_snippet.origin],
    ).unwrap();

    let article_snippet_response = row_to_article_snippet(updated_rows.get(0));

    Json(article_snippet_response)
}

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

pub fn get_article_snippet(conn: &TigumPgConn, id: i32) -> Json<ArticleSnippet> {
    let query_result = conn
        .query("SELECT * FROM article_snippets WHERE id = $1", &[&id])
        .unwrap();
    println!("{:#?}", query_result);
    let article_snippet_response = row_to_article_snippet(query_result.get(0));
    Json(article_snippet_response)
}

pub fn create_article_snippet(
    conn: &TigumPgConn,
    article_snippet: &Json<NewArticleSnippet>,
) -> Json<ArticleSnippet> {
    let inserted_row = conn
        .query(
            "INSERT INTO article_snippets (content, origin, topic_id, user_id) VALUES ($1, $2, $3, $4) RETURNING *",
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
    let article_snippet = row_to_article_snippet(row);
    Json(article_snippet)
}
