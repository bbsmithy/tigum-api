use crate::db;
use rocket_contrib::databases::postgres::Row;
use rocket::http::Status;

use db::models::search::resources::ResourceResult;
use db::querys::TigumPgConn;
use db::api_response::ApiResponse;

const FIND_BY_TITLE_QUERY_STRING: &str = "
SELECT 'topic' result_type, id as topic_id, title, 0 as resource_id, 'none' as misc FROM topics WHERE lower(title) LIKE $1 AND user_id = $2
UNION

SELECT 'note' result_type, topic_id, title, id as resource_id, 'none' as misc FROM notes WHERE lower(title) LIKE $1 AND user_id = $2
UNION

SELECT 'video' result_type, topic_id, title, id as resource_id, iframe as misc FROM videos
WHERE lower(title) LIKE $1 AND user_id = $2
UNION

SELECT 'link' result_type, topic_id, title, id as resource_id, source as misc FROM links
WHERE lower(title) LIKE $1 AND user_id = $2
UNION

SELECT 'snippet' result_type, topic_id, content as title, id as resource_id, origin as misc FROM article_snippets
WHERE lower(content) LIKE $1 AND user_id = $2
";

const FIND_BY_TOPIC_ID: &str = "
SELECT 'video' result_type, topic_id, title, id as resource_id, iframe as misc FROM videos
WHERE topic_id = $1 AND user_id = $2
UNION

SELECT 'link' result_type, topic_id, title, id as resource_id, source as misc FROM links
WHERE topic_id = $1 AND user_id = $2
UNION

SELECT 'snippet' result_type, topic_id, content as title, id as resource_id, origin as misc FROM article_snippets
WHERE topic_id = $1 AND user_id = $2
UNION

SELECT 'note' result_type, topic_id, title, id as resource_id, 'none' as misc FROM notes WHERE topic_id = $1 AND user_id = $2
";


fn row_to_resource_result(row: &Row) -> ResourceResult {
    ResourceResult {
        result_type: row.get(0),
        topic_id: row.get(1),
        title: row.get(2),
        resource_id: row.get(3),
        misc: row.get(4)
    }
}

pub async fn find_by_title(conn: &TigumPgConn, title: String, user_id: i32) -> ApiResponse {
    let like_title = format!("%{}%", title.to_lowercase());
    let result_query = conn.run(move |c|
        c.query(FIND_BY_TITLE_QUERY_STRING, &[&like_title, &user_id])
    ).await;
    return_search_results(result_query)
}

pub async fn find_by_topic_id(conn: &TigumPgConn, topic_id: i32, user_id: i32) -> ApiResponse {
    let result_query = conn.run(move |c|
        c.query(FIND_BY_TOPIC_ID, &[&topic_id, &user_id])
    ).await;
    return_search_results(result_query)
}

fn return_search_results(result_query: Result<Vec<rocket_contrib::databases::postgres::Row>, rocket_contrib::databases::postgres::Error>) -> ApiResponse {
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