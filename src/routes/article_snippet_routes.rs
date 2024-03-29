//Use Macros
use crate::db;
use rocket::Route;
use rocket_contrib::json::Json;

use db::api_response::ApiResponse;

use db::models::resources::article_snippets::{NewArticleSnippet};
use db::models::user::User;
use db::models::Ids;

use db::querys::article_snippets_query::{
    create_article_snippet, 
    delete_article_snippet, 
    get_article_snippet, 
    get_article_snippets,
    update_article_snippet,
    publish_snippet
};
use db::querys::TigumPgConn;

/////////////////////////////////
//// ARTICLE SNIPPETS ROUTES ////
/////////////////////////////////

#[delete("/article_snippets/<id>")]
fn delete_single_article_snippet(conn: TigumPgConn, id: i32, auth_user: User) -> ApiResponse {
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
) -> ApiResponse {
    update_article_snippet(&conn, id, article_snippet, auth_user.id)
}

#[put("/article_snippets/publish/<video_id>/<publish_flag>", format = "application/json")]
fn publish_single_article_snippet(conn: TigumPgConn, video_id: i32, publish_flag: bool, auth_user: User) -> ApiResponse {
    publish_snippet(&*conn, video_id, publish_flag, auth_user.id)
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
    create_article_snippet(&conn, article_snippet, auth_user.id)
}

#[get("/article_snippets/<id>")]
fn single_article_snippet(conn: TigumPgConn, id: i32, auth_user: User) -> ApiResponse {
    get_article_snippet(&conn, id, auth_user.id)
}

#[post("/article_snippets", format = "application/json", data = "<ids>")]
fn article_snippets(
    conn: TigumPgConn,
    ids: Json<Ids>,
    auth_user: User,
) -> ApiResponse {
    get_article_snippets(&conn, ids, auth_user.id)
}

pub fn get_article_snippet_routes() -> Vec<Route> {
    routes![
        create_single_article_snippet,
        article_snippets,
        single_article_snippet,
        update_single_article_snippet,
        delete_single_article_snippet,
        publish_single_article_snippet
    ]
}
