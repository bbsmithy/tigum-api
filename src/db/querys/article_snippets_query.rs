//Use Macros
use rocket_contrib::json::{Json};
use rocket_contrib::databases::postgres::Error;
use rocket::http::Status;

use crate::db::models;
use crate::db::querys::TigumPgConn;
use crate::db::querys::topic_query::{remove_from_topic_resource_list, add_to_topic_resource_list};
use crate::db::api_response::ApiResponse;
use crate::db::models::resources::ResourceType;

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

pub fn delete_article_snippet(conn: &TigumPgConn, id: i32, user_id: i32) -> ApiResponse {
    let query_result = conn.query("DELETE FROM article_snippets WHERE id = $1 AND user_id = $2 RETURNING topic_id", &[&id, &user_id]);
    match query_result {
        Ok(result) => {
            let row_result = result.get(0);
            let topic_id = row_result.get(0);
            let remove_query = remove_from_topic_resource_list(&conn, topic_id, id, ResourceType::Snippet);
            match remove_query {
                Ok(_rows_removed) => ApiResponse {
                    json: json!({ "msg": format!("Snippet with id {} deleted successfully", id) }),
                    status: Status::raw(200)
                },
                Err(_error) => ApiResponse {
                    json: json!({ "error": format!("Could not delete snippet {} from topic {}", id, topic_id) }),
                    status: Status::raw(500)
                }
            }
        },
        Err(_error) => {
            ApiResponse {
                json: json!({"error": format!("Could not delete snippet {}", id) }),
                status: Status::raw(500) 
            }
        }
    }
}

pub fn update_article_snippet(
    conn: &TigumPgConn,
    id: i32,
    article_snippet: Json<NewArticleSnippet>,
    user_id: i32
) -> ApiResponse {
    let updated_rows = conn.query(
        "UPDATE article_snippets SET topic_id = $2, user_id = $3, content = $4, origin = $5 WHERE id = $1 AND user_id = $3 RETURNING *",
        &[&id, &article_snippet.topic_id, &user_id, &article_snippet.content, &article_snippet.origin],
    );
    match updated_rows {
        Ok(rows) => {
            ApiResponse {
                json: json!(row_to_article_snippet(rows.get(0))),
                status: Status::raw(200)
            }
        },
        Err(_error) => {
            ApiResponse {
                json: json!({ "error": format!("Could not update article snippet with id {}", id)}),
                status: Status::raw(500)
            }
        }
    }
   
}

pub fn get_article_snippets(conn: &TigumPgConn, ids: Json<Ids>, user_id: i32) -> ApiResponse {
    let query_result = conn.query(
            "SELECT * FROM article_snippets WHERE id = ANY($1) AND user_id = $2 ORDER BY date_created ASC",
            &[&ids.ids, &user_id],
        );
    match query_result {
        Ok(rows) => {
            let mut results: Vec<ArticleSnippet> = vec![];
            for row in rows.iter() {
                let article_snippet_response = row_to_article_snippet(row);
                results.push(article_snippet_response);
            }
            ApiResponse {
                json: json!(results),
                status: Status::raw(200)
            }
        },
        Err(_error) => {
            ApiResponse {
                json: json!({"error": format!("Could not retrieve snippets with ids {:?}", ids)}),
                status: Status::raw(500)
            }
        }
    }
}

pub fn get_article_snippet(conn: &TigumPgConn, id: i32, user_id: i32) -> ApiResponse {
    let query_result = conn.query("SELECT * FROM article_snippets WHERE id = $1 AND user_id = $2", &[&id, &user_id]);
    match query_result {
        Ok(rows) => {
            let result_row = rows.get(0);
            ApiResponse {
                json: json!(row_to_article_snippet(result_row)),
                status: Status::raw(200)
            }
        },
        Err(_error) => {
            ApiResponse {
                json: json!({"error": format!("Could not retrieve snippet {}", id)}),
                status: Status::raw(500)
            }
        }
    }
}

pub fn create_article_snippet(
    conn: &TigumPgConn,
    article_snippet: &Json<NewArticleSnippet>,
    user_id: i32
) -> ApiResponse {
    match conn.query(
        "INSERT INTO article_snippets (title, content, origin, topic_id, user_id) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        &[
            &article_snippet.title,
            &article_snippet.content,
            &article_snippet.origin,
            &article_snippet.topic_id,
            &user_id,
        ],
    ) {
        Ok(row) => {
            let new_row = row.get(0);
            let new_article_snippet = row_to_article_snippet(new_row);
            match add_to_topic_resource_list(
                &conn,
                new_article_snippet.topic_id,
                new_article_snippet.id,
                ResourceType::Snippet,
            ) {
                Ok(_rows_updated) => ApiResponse { json: json!(new_article_snippet), status: Status::raw(200) },
                Err(_error) => ApiResponse {
                    json: json!({ "error": "Could not create snippet" }),
                    status: Status::raw(500)
                }
            }
        },  
        Err(_error) => ApiResponse {
            json: json!({
                "error": format!("Could not create snippet {}", article_snippet.topic_id )
            }),
            status: Status::raw(500)
        }
    }
}
