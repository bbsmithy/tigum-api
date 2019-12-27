use crate::db;

use db::querys::TigumPgConn;
use rocket::Route;

//Use Macros
use rocket_contrib::json::Json;

// Database Models
use db::models::topic::{NewTopic, Topic, TopicIds};
use db::models::user::User;

//Database Querys
use db::querys::topic_query::{create_topic, delete_topic, get_topic, get_topics, update_topic};

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

pub fn get_topic_routes() -> Vec<Route> {
    routes![
        delete_single_topic,
        update_single_topic,
        create_single_topic,
        single_topic,
        topics
    ]
}
