#![feature(proc_macro_hygiene, decl_macro)]

// Macros
#[macro_use]
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate rocket_contrib;

// Main modules
mod cors;
mod db;
mod guards;
mod routes;
mod util;

use db::querys::TigumPgConn;

//Request Routes
use routes::article_snippet_routes::get_article_snippet_routes;
use routes::note_routes::get_note_routes;
use routes::topic_routes::get_topic_routes;
use routes::video_routes::video_routes;
use routes::image_routes::image_routes;

// Request Gaurds
use guards::User;

// CORS Prelight Request Handler

#[route(OPTIONS, path = "/")]
fn preflight_handler() -> String {
    String::from("Handling preflight")
}

fn create_routes() -> Vec<rocket::Route> {
    let mut app_routes = routes![preflight_handler];
    let mut video_routes_config = video_routes();
    let mut article_snippets_routes_config = get_article_snippet_routes();
    let mut note_routes_config = get_note_routes();
    let mut topic_routes_config = get_topic_routes();
    let mut image_routes_config = image_routes();
    app_routes.append(&mut video_routes_config);
    app_routes.append(&mut article_snippets_routes_config);
    app_routes.append(&mut note_routes_config);
    app_routes.append(&mut topic_routes_config);
    app_routes.append(&mut image_routes_config);
    app_routes
}

fn main() {
    let routes = create_routes();
    rocket::ignite()
        .mount("/", routes)
        .attach(cors::CorsFairing)
        .attach(TigumPgConn::fairing())
        .launch();
}
