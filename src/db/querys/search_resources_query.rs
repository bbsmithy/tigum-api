use crate::db;
use rocket_contrib::json::Json;
use rocket_contrib::databases::postgres::Error;
use rocket_contrib::databases::postgres::rows::Row;
use rocket::http::Status;

use db::models::search::resources::ResourceResult;
use db::querys::TigumPgConn;
use db::api_response::ApiResponse;

const FIND_BY_TITLE_QUERY_STRING: &str = "
SELECT 'topic' result_type, id as topic_id, title, 0 as resource_id FROM topics WHERE title LIKE $1 AND user_id = $2
UNION

SELECT 'note' result_type, topic_id, title, id as resource_id FROM notes WHERE title LIKE $1 AND user_id = $2
UNION

SELECT 'video' result_type, topic_id, title, id as resource_id FROM videos
WHERE title LIKE $1 AND user_id = $2
UNION

SELECT 'links' result_type, topic_id, title, id as resource_id FROM links
WHERE title LIKE $1 AND user_id = $2
UNION

SELECT 'article_snippet' result_type, topic_id, content, id as resource_id FROM article_snippets
WHERE content LIKE $1 AND user_id = $2
";


fn row_to_resource_result(row: Row) -> ResourceResult {
    ResourceResult {
        result_type: row.get(0),
        topic_id: row.get(1),
        title: row.get(2),
        resource_id: row.get(3)
    }
}

pub fn find_by_title(conn: &TigumPgConn, title: String, user_id: i32) -> ApiResponse {
    let like_title = format!("{}%", title);
    let result_query = conn.query(FIND_BY_TITLE_QUERY_STRING, &[&like_title, &user_id]);
    match result_query {
        Ok(rows) => {
            let mut resource_results: Vec<ResourceResult> = vec![];
            for row in rows.iter() {
                let resource_result_row = row_to_resource_result(row);
                resource_results.push(resource_result_row)
            };
            ApiResponse {
                json: json!(resource_results),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": "Query failed" }),
                status: Status::raw(500)
            }
        }
    }
}