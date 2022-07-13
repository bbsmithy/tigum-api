use crate::db;
use rocket::http::Status;
use crate::db::api_response::ApiResponse;
use rocket_contrib::databases::diesel;
use diesel::{RunQueryDsl};
use diesel::sql_query;
use db::models::search::resources::ResourceResult;


pub fn find_by_title(conn: &diesel::PgConnection, search_title: String, user_id: i32) -> ApiResponse {
    let find_by_title_like_query_string = format!("
        SELECT 'topic' result_type, id as topic_id, title, 0 as resource_id, 'none' as misc, 'none' as misc2, date_updated FROM topics WHERE lower(title) LIKE '%{q_title}%' AND user_id = {uid}
        UNION ALL
        SELECT 'note' result_type, topic_id, title, id as resource_id, 'none' as misc, 'none' as misc2, date_updated FROM notes WHERE lower(title) LIKE '%{q_title}%' AND user_id = {uid}
        UNION ALL
        SELECT 'video' result_type, topic_id, title, id as resource_id, iframe as misc, thumbnail_img as misc2, date_updated FROM videos
        WHERE lower(title) LIKE '%{q_title}%' AND user_id = {uid}
        UNION ALL
        SELECT 'link' result_type, topic_id, title, id as resource_id, source as misc, favicon_source as misc2, date_updated FROM links
        WHERE lower(title) LIKE '%{q_title}%' AND user_id = {uid}
        UNION ALL
        SELECT 'snippet' result_type, topic_id, content as title, id as resource_id, origin as misc, title as misc2, date_updated FROM article_snippets
        WHERE lower(content) LIKE '%{q_title}%' AND user_id = {uid}
        ORDER BY date_updated DESC
    ", q_title = search_title, uid = user_id);
    let result = sql_query(find_by_title_like_query_string).get_results::<ResourceResult>(conn);
    match result {
        Ok(rows) => {
            ApiResponse {
                json: json!(rows),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!("nope"),
                status: Status::raw(500)
            }
        }
    }
}

pub fn find_by_topic_id(conn: &diesel::PgConnection, topic_id: i32, user_id: i32) -> ApiResponse {
    let find_by_topic_id_query = format!("
        SELECT 'video' result_type, topic_id, title, id as resource_id, iframe as misc, thumbnail_img as misc2, date_updated FROM videos
        WHERE topic_id = {tid} AND user_id = {uid}
        UNION ALL
        SELECT 'link' result_type, topic_id, title, id as resource_id, source as misc, favicon_source as misc2, date_updated FROM links
        WHERE topic_id = {tid} AND user_id = {uid}
        UNION ALL
        SELECT 'snippet' result_type, topic_id, content as title, id as resource_id, origin as misc, title as misc2, date_updated FROM article_snippets
        WHERE topic_id = {tid} AND user_id = {uid}
        UNION ALL
        SELECT 'note' result_type, topic_id, title, id as resource_id, 'none' as misc, 'none' as misc2, date_updated FROM notes WHERE topic_id = {tid} AND user_id = {uid}
        ORDER BY date_updated DESC
    ", tid = topic_id, uid = user_id);
    let result = sql_query(find_by_topic_id_query).get_results::<ResourceResult>(conn);
    match result {
        Ok(rows) => {
            ApiResponse {
                json: json!(rows),
                status: Status::raw(200)
            }
        },
        Err(err) => {
            ApiResponse {
                json: json!("nope"),
                status: Status::raw(500)
            }
        }
    }
}