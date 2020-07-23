//Use Macros
use crate::db;
use rocket::Route;
use rocket_contrib::json::Json;

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
) -> Json<ArticleSnippet> {
    let new_article_snippet = create_article_snippet(&conn, &article_snippet, auth_user.id);
    add_to_topic_resource_list(
        &conn,
        article_snippet.topic_id,
        new_article_snippet.id,
        ResourceType::Snippet,
    );
    return new_article_snippet;
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
