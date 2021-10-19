//Use Macros
use rocket_contrib::json::{Json};
use rocket::http::Status;
use crate::db::models;
use crate::db::querys::topic_query::{
    remove_from_topic_resource_list,
    add_to_topic_resource_list,
    update_topic_mod_date
};
use crate::db::api_response::ApiResponse;
use crate::db::models::resources::ResourceType;
use crate::db::models::resources::article_snippets::{ArticleSnippet, NewArticleSnippet};
use models::Ids;
use diesel::result::Error;
use rocket_contrib::databases::diesel;
use diesel::{QueryDsl, RunQueryDsl};
use diesel::ExpressionMethods;
use diesel::Connection;
use diesel::dsl::any;



pub fn delete_article_snippet(conn: &diesel::PgConnection, a_id: i32, uid: i32) -> ApiResponse {

    use crate::schema::article_snippets::dsl::*;
    let article_snippet_to_delete = article_snippets.filter(id.eq(a_id)).filter(user_id.eq(uid));
    // TODO use transactions when 2 queries should both happend for success
    let transaction_result = conn.transaction::<_, Error, _>(|| {
        let res = diesel::delete(article_snippet_to_delete).get_result::<ArticleSnippet>(conn)?;
        remove_from_topic_resource_list(conn, res.topic_id, a_id, ResourceType::Snippet)?;
        Ok(())
    });
    if transaction_result.is_ok() {
        ApiResponse {
            json: json!({ "msg": format!("Successfully deleted article snippet with id {}", a_id) }),
            status: Status::raw(200)
        }
    } else {
        ApiResponse {
            json: json!({ "error": format!("Failed to delete article snippet with id: {}", a_id) }),
            status: Status::raw(500)
        } 
    }
}

pub fn update_article_snippet(
    conn: &diesel::PgConnection,
    v_id: i32,
    article_snippet: Json<NewArticleSnippet>,
    uid: i32
) -> ApiResponse {
    use crate::schema::article_snippets::dsl::*;
    let article_snippet_to_update = article_snippets.filter(id.eq(v_id)).filter(user_id.eq(uid));
    let values = (
        topic_id.eq(article_snippet.topic_id),
        title.eq(article_snippet.title.clone()),
        user_id.eq(uid),
        origin.eq(article_snippet.origin.clone()),
        content.eq(article_snippet.content.clone())
    );
    let query_result = diesel::update(article_snippet_to_update).set(values).get_result::<ArticleSnippet>(conn);
    match query_result {
        Ok(updated_row) => {
            update_topic_mod_date(conn, updated_row.topic_id);
            ApiResponse {
                json: json!(updated_row),
                status: Status::raw(200)
            }
       },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": format!("Could not update video with id {}", v_id) }),
                status: Status::raw(500)
            }
        }
    }
}

pub fn get_article_snippets(conn: &diesel::PgConnection, article_snippet_ids: Json<Ids>, uid: i32) -> ApiResponse {
    use crate::schema::article_snippets::dsl::*;
    let ids = article_snippet_ids.ids.clone();
    let res = article_snippets.filter(id.eq(any(ids))).filter(user_id.eq(uid)).order_by(date_updated.desc()).get_results::<ArticleSnippet>(conn);
    match res {
        Ok(rows) => {
            ApiResponse {
                json: json!(rows),
                status: Status::raw(200)
            }
        }, 
        Err(_err) => {
            ApiResponse {
                json: json!({"error": format!("Could not retrieve snippets")}),
                status: Status::raw(500)
            }
        }
    }
}

pub fn get_article_snippet(conn: &diesel::PgConnection, article_snippet_id: i32, uid: i32) -> ApiResponse {
    use crate::schema::article_snippets::dsl::*;
    let res = article_snippets.filter(id.eq(article_snippet_id)).filter(user_id.eq(uid)).get_result::<ArticleSnippet>(conn);
    match res {
        Ok(rows) => {
            ApiResponse {
                json: json!(rows),
                status: Status::raw(200)
            }
        }, 
        Err(_err) => {
            ApiResponse {
                json: json!({"error": format!("Could not retrieve snippet {}", article_snippet_id)}),
                status: Status::raw(500)
            }
        }
    }
}

pub fn create_article_snippet(
    conn: &diesel::PgConnection,
    article_snippet: Json<NewArticleSnippet>,
    uid: i32
) -> ApiResponse {

    use crate::schema::article_snippets::dsl::*;

    let a_content = article_snippet.content.clone();
    let a_origin = article_snippet.origin.clone();
    let a_topic_id = article_snippet.topic_id.clone();
    let a_title = article_snippet.title.clone();


    let tranaction_result = conn.transaction::<ArticleSnippet, Error, _>(|| {
        let new_snippet = diesel::insert_into(article_snippets).values((
            content.eq(a_content),
            origin.eq(a_origin),
            topic_id.eq(a_topic_id),
            title.eq(a_title),
            user_id.eq(uid)
        )).get_result::<ArticleSnippet>(conn)?;
        add_to_topic_resource_list(conn, a_topic_id, new_snippet.id, ResourceType::Snippet)?;
        Ok(new_snippet)
    });

    match tranaction_result {
        Ok(new_article_snippet) => {
            update_topic_mod_date(conn, new_article_snippet.topic_id);
            ApiResponse {
                json: json!(new_article_snippet),
                status: Status::raw(200)
            }
        },
        Err(err) => {
            println!("{:?}", err);
            ApiResponse {
                json: json!({ "error": "Could not create snippet" }),
                status: Status::raw(500)
            }
        }
    }
}
