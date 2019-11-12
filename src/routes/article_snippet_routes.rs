//Use Macros
use crate::db;
use crate::guards::User;
use rocket::Route;
use rocket_contrib::json::Json;

use db::models::resources::article_snippets::{ArticleSnippet, NewArticleSnippet};
use db::models::{Id, Ids};

use db::querys::article_snippets_query::{
    create_article_snippet, get_article_snippet, get_article_snippets, update_article_snippet,
};
use db::querys::TigumPgConn;

/////////////////////////////////
//// ARTICLE SNIPPETS ROUTES ////
/////////////////////////////////

// #[delete("/article_snippets/<id>")]
// fn delete_single_article_snippet(conn: TigumPgConn, id: i32, _auth_user: User) -> Json<String> {
//     delete_article_snippet(&conn, id)
// }

#[put(
    "/article_snippets/<id>",
    format = "application/json",
    data = "<article_snippet>"
)]
fn update_single_article_snippet(
    conn: TigumPgConn,
    id: i32,
    article_snippet: Json<NewArticleSnippet>,
) -> Json<ArticleSnippet> {
    update_article_snippet(&conn, id, article_snippet)
}

#[post(
    "/article_snippets/create",
    format = "application/json",
    data = "<article_snippet>"
)]
fn create_single_article_snippet(
    conn: TigumPgConn,
    article_snippet: Json<NewArticleSnippet>,
) -> Json<Id> {
    println!("{:?}", article_snippet);
    create_article_snippet(&conn, article_snippet)
}

#[get("/article_snippets/<id>")]
fn single_article_snippet(conn: TigumPgConn, id: i32, _auth_user: User) -> Json<ArticleSnippet> {
    get_article_snippet(&conn, id)
}

#[post("/article_snippets", format = "application/json", data = "<ids>")]
fn article_snippets(conn: TigumPgConn, ids: Json<Ids>) -> Json<Vec<ArticleSnippet>> {
    println!("{:?}", ids);
    get_article_snippets(&conn, ids)
}

pub fn get_article_snippet_routes() -> Vec<Route> {
    routes![
        create_single_article_snippet,
        article_snippets,
        single_article_snippet,
        update_single_article_snippet
    ]
}
