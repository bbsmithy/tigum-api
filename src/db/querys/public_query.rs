use crate::db::api_response::ApiResponse;
use rocket::http::Status;
use crate::db::models::topic::Topic;
use crate::db::models::resources::note::Note;
use crate::db::models::resources::video::Video;
use crate::db::models::resources::link::Link;
use crate::db::models::resources::article_snippets::ArticleSnippet;
use crate::db::models::user::AuthUser;
use rocket_contrib::databases::diesel;
use diesel::{QueryDsl, RunQueryDsl};
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
