#![feature(proc_macro_hygiene, decl_macro)]

// Macros
#[macro_use]
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate rocket_contrib;

//Use Macros
use rocket_contrib::json::Json;

// Main modules
mod cors;
mod db;
mod guards;
mod routes;

// Database Models
use db::models::topic::{NewTopic, Topic, TopicIds};

//Database Querys
use db::querys::topic_query::{create_topic, delete_topic, get_topic, get_topics, update_topic};
use db::querys::TigumPgConn;

//Request Routes
use routes::article_snippet_routes::get_article_snippet_routes;
use routes::video_routes::video_routes;
use routes::note_routes::get_note_routes;

// Request Gaurds
use guards::User;

//////////////////////
//// TOPIC ROUTES ////
//////////////////////

#[delete("/topics/<topic_id>")]
fn delete_single_topic(conn: TigumPgConn, topic_id: i32) -> String {
    delete_topic(&conn, topic_id)
}

#[put("/topics/<topic_id>", format = "application/json", data = "<topic>")]
fn update_single_topic(
    conn: TigumPgConn,
    topic_id: i32,
    topic: Json<Topic>,
    _auth_user: User,
) -> Json<Topic> {
    update_topic(&conn, topic_id, topic)
}

#[post("/topics/create-topic", format = "application/json", data = "<topic>")]
fn create_single_topic(conn: TigumPgConn, topic: Json<NewTopic>, _auth_user: User) -> Json<Topic> {
    create_topic(&conn, topic)
}

#[get("/topics/<topic_id>")]
fn single_topic(conn: TigumPgConn, topic_id: i32, _auth_user: User) -> Json<Topic> {
    get_topic(&conn, topic_id)
}

#[post("/topics", format = "application/json", data = "<topic_ids>")]
fn topics(conn: TigumPgConn, topic_ids: Json<TopicIds>, _auth_user: User) -> Json<Vec<Topic>> {
    println!("{:?}", topic_ids);
    get_topics(&conn, topic_ids)
}

// CORS Prelight Request Handler

#[route(OPTIONS, path = "/")]
fn preflight_handler() -> String {
    let res: String = String::from("Handling preflight");
    res
}

fn create_routes() -> Vec<rocket::Route> {
    let mut app_routes = routes![
        topics,
        single_topic,
        create_single_topic,
        update_single_topic,
        delete_single_topic,
        preflight_handler
    ];
    let mut video_routes_config = video_routes();
    let mut article_snippets_routes_config = get_article_snippet_routes();
    let mut note_routes_config = get_note_routes();
    app_routes.append(&mut video_routes_config);
    app_routes.append(&mut article_snippets_routes_config);
    app_routes.append(&mut note_routes_config);
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
