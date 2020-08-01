//Use Macros
use crate::db;
use rocket::Route;
use rocket_contrib::json::Json;
use rocket::http::{Status};

use db::api_response::ApiResponse;

use db::models::resources::article_snippets::{ArticleSnippet, NewArticleSnippet};
use db::models::resources::ResourceType;
use db::models::user::User;
use db::models::Ids;

use db::querys::article_snippets_query::{
    create_article_snippet, delete_article_snippet, get_article_snippet, get_article_snippets,
    update_article_snippet,
};
use db::querys::topic_query::add_to_topic_resource_list;
use db::querys::TigumPgConn;

/////////////////////////////////
//// ARTICLE SNIPPETS ROUTES ////
/////////////////////////////////

#[delete("/article_snippets/<id>")]
fn delete_single_article_snippet(conn: TigumPgConn, id: i32, auth_user: User) -> Json<String> {
    delete_article_snippet(&conn, id, auth_user.id)
}

#[put(
    "/article_snippets/<id>",
    format = "application/json",
    data = "<article_snippet>"
)]
fn update_single_article_snippet(
    conn: TigumPgConn,
    id: i32,
    article_snippet: Json<NewArticleSnippet>,
    auth_user: User,
) -> Json<ArticleSnippet> {
    update_article_snippet(&conn, id, article_snippet, auth_user.id)
}

#[post(
    "/article_snippets/create",
    format = "application/json",
    data = "<article_snippet>"
)]
fn create_single_article_snippet(
    conn: TigumPgConn,
    article_snippet: Json<NewArticleSnippet>,
    auth_user: User,
) -> ApiResponse {
    let create_article_snippet_query_result = create_article_snippet(&conn, &article_snippet, auth_user.id);
    match create_article_snippet_query_result {
        Ok(new_article_snippet) => {
            let query_result = add_to_topic_resource_list(
                &conn,
                article_snippet.topic_id,
                new_article_snippet.id,
                ResourceType::Snippet,
            );
            match query_result {
                Ok(_rows_updated) => ApiResponse { json: json!(new_article_snippet), status: Status::raw(200) },
                Err(_error) => ApiResponse {
                    json: json!({ "error": format!("Could not create snippet {}", article_snippet.topic_id )}),
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

#[get("/article_snippets/<id>")]
fn single_article_snippet(conn: TigumPgConn, id: i32, auth_user: User) -> Json<ArticleSnippet> {
    get_article_snippet(&conn, id, auth_user.id)
}

#[post("/article_snippets", format = "application/json", data = "<ids>")]
fn article_snippets(
    conn: TigumPgConn,
    ids: Json<Ids>,
    auth_user: User,
) -> Json<Vec<ArticleSnippet>> {
    get_article_snippets(&conn, ids, auth_user.id)
}

pub fn get_article_snippet_routes() -> Vec<Route> {
    routes![
        create_single_article_snippet,
        article_snippets,
        single_article_snippet,
        update_single_article_snippet,
        delete_single_article_snippet
    ]
}
