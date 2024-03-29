use crate::db::api_response::ApiResponse;
use rocket::http::Status;
use crate::db::models::topic::Topic;
use crate::db::models::resources::note::Note;
use crate::db::models::resources::video::Video;
use crate::db::models::resources::link::Link;
use crate::db::models::resources::article_snippets::ArticleSnippet;
use crate::db::models::search::resources::ResourceResult;
use crate::db::models::user::AuthUser;
use crate::db::models::dto::{PublicResources};
use rocket_contrib::databases::diesel;
use diesel::{QueryDsl, RunQueryDsl};
use diesel::sql_query;
use diesel::ExpressionMethods;


pub fn get_public_topics_for_user(conn: &diesel::PgConnection, user_name: String) -> ApiResponse {
    use crate::schema::users::dsl::*;
    use crate::schema::topics::dsl::*;

    let topic_date_updated = crate::schema::topics::dsl::date_updated;

    let check_for_user = users.filter(name.eq(user_name)).get_result::<AuthUser>(conn);
    match check_for_user {
        Ok(row) => {
            let published_topics_res = topics.filter(user_id.eq(row.id))
            .filter(published.eq(true))
            .order_by(topic_date_updated.desc())
            .get_results::<Topic>(conn);
            match published_topics_res {
                Ok(rows) => {
                    ApiResponse {
                        json: json!({ "topics": rows }),
                        status: Status::raw(200)
                    }
                },
                Err(_err) => {
                    ApiResponse {
                        status: Status::raw(500),
                        json: json!({ "error": "Something went wrong" })
                    }
                }
            }
        },
        Err(_err) => {
            ApiResponse {
                status: Status::raw(404),
                json: json!({ "msg": "Failed to find user" })
            }
        }
    }    
}

pub fn get_public_notes_in_topic(conn: &diesel::PgConnection, notes_topic_id: i32) -> ApiResponse {
    use crate::schema::notes::dsl::*;
    let published_notes_res = notes.filter(topic_id.eq(notes_topic_id))
    .filter(published.eq(true))
    .order_by(date_updated.desc())
    .get_results::<Note>(conn);
    if let Ok(result) = published_notes_res {
        ApiResponse {
            status: Status::raw(200),
            json: json!({ "notes": result })
        }
    } else {
        ApiResponse {
            status: Status::raw(404),
            json: json!({ "msg": "Failed to find user" })
        }
    }
}

pub fn get_public_videos_in_topic(conn: &diesel::PgConnection, videos_topic_id: i32) -> ApiResponse {
    use crate::schema::videos::dsl::*;
    let published_videos_res = videos.filter(topic_id.eq(videos_topic_id)).filter(published.eq(true)).order_by(date_updated.desc()).get_results::<Video>(conn);
    if let Ok(result) = published_videos_res {
        ApiResponse {
            status: Status::raw(200),
            json: json!({ "videos": result })
        }
    } else {
        ApiResponse {
            status: Status::raw(404),
            json: json!({ "msg": "Failed to find user" })
        }
    }
}

pub fn get_public_snippets_in_topic(conn: &diesel::PgConnection, snippet_topic_id: i32) -> ApiResponse {
    use crate::schema::article_snippets::dsl::*;
    let published_videos_res = article_snippets.filter(topic_id.eq(snippet_topic_id)).filter(published.eq(true)).order_by(date_updated.desc()).get_results::<ArticleSnippet>(conn);
    if let Ok(result) = published_videos_res {
        ApiResponse {
            status: Status::raw(200),
            json: json!({ "snippets": result })
        }
    } else {
        ApiResponse {
            status: Status::raw(404),
            json: json!({ "msg": "Failed to find user" })
        }
    }
}

pub fn get_public_links_in_topic(conn: &diesel::PgConnection, link_topic_id: i32) -> ApiResponse {
    use crate::schema::links::dsl::*;
    let published_videos_res = links.filter(topic_id.eq(link_topic_id)).filter(published.eq(true)).order_by(date_updated.desc()).get_results::<Link>(conn);
    if let Ok(result) = published_videos_res {
        ApiResponse {
            status: Status::raw(200),
            json: json!({ "links": result })
        }
    } else {
        ApiResponse {
            status: Status::raw(404),
            json: json!({ "msg": "Failed to find user" })
        }
    }
}

pub fn build_public_resources_query(topic_id: i32) -> String {
    format!("
        SELECT 'note' result_type, topic_id, title, id as resource_id, 'none' as misc, 'none' as misc2, date_updated FROM notes WHERE topic_id = {tid} AND published = TRUE
        UNION ALL
        SELECT 'video' result_type, topic_id, title, id as resource_id, iframe as misc, thumbnail_img as misc2, date_updated FROM videos
        WHERE topic_id = {tid} AND published = TRUE
        UNION ALL
        SELECT 'link' result_type, topic_id, title, id as resource_id, source as misc, favicon_source as misc2, date_updated FROM links
        WHERE topic_id = {tid} AND published = TRUE
        UNION ALL
        SELECT 'snippet' result_type, topic_id, content as title, id as resource_id, origin as misc, title as misc2, date_updated FROM article_snippets
        WHERE topic_id = {tid} AND published = true
        ORDER BY date_updated DESC
    ", tid=topic_id)
}

pub fn build_public_resources_count_query(topic_id: i32) -> String {
    format!("
        SELECT COUNT(*) as public_resources_count
        FROM
        (
            SELECT 'note' result_type, topic_id, title, id as resource_id, 'none' as misc, 'none' as misc2, date_updated FROM notes WHERE topic_id = {tid} AND published = TRUE
            UNION ALL
            SELECT 'video' result_type, topic_id, title, id as resource_id, iframe as misc, thumbnail_img as misc2, date_updated FROM videos
            WHERE topic_id = {tid} AND published = TRUE
            UNION ALL
            SELECT 'link' result_type, topic_id, title, id as resource_id, source as misc, favicon_source as misc2, date_updated FROM links
            WHERE topic_id = {tid} AND published = TRUE
            UNION ALL
            SELECT 'snippet' result_type, topic_id, content as title, id as resource_id, origin as misc, title as misc2, date_updated FROM article_snippets
            WHERE topic_id = {tid} AND published = true
            ORDER BY date_updated DESC
        ) AS x", tid=topic_id)
    }


pub fn get_public_resources_for_topic(conn: &diesel::PgConnection, topic_id: i32) -> ApiResponse {
    let find_public_resources_query = build_public_resources_query(topic_id);
    let result = sql_query(find_public_resources_query).get_results::<ResourceResult>(conn);
    match result {
        Ok(rows) => {
            let pub_resources = PublicResources::new(rows);
            ApiResponse {
                json: json!(pub_resources),
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
