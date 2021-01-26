use crate::db;
use rocket::Route;

//Use Macros
use rocket_contrib::json::Json;

// Database Models
use db::models::topic::{NewTopic, Topic, TopicIds};
use db::models::user::User;
use db::querys::TigumPgConn;

// Api Response Struct
use db::api_response::ApiResponse;

//Database Querys
use db::querys::topic_query::{create_topic, delete_topic, get_topic, get_topics, update_topic};

//////////////////////
//// TOPIC ROUTES ////
//////////////////////

#[delete("/topics/<topic_id>")]
async fn delete_single_topic(conn: TigumPgConn, topic_id: i32) -> ApiResponse {
    delete_topic(&conn, topic_id).await
}

#[put("/topics/<topic_id>", format = "application/json", data = "<topic>")]
async fn update_single_topic(
    conn: TigumPgConn,
    topic_id: i32,
    topic: Json<Topic>,
) -> ApiResponse {
    update_topic(&conn, topic_id, topic).await
}

#[post("/topics/create-topic", format = "application/json", data = "<topic>")]
async fn create_single_topic(conn: TigumPgConn, topic: Json<NewTopic>, auth_user: User) -> ApiResponse {
    create_topic(&conn, topic, auth_user.id).await
}

#[get("/topics/<topic_id>")]
async fn single_topic(conn: TigumPgConn, topic_id: i32, auth_user: User) -> ApiResponse {
    get_topic(&conn, topic_id, auth_user.id).await
}

#[post("/topics", format = "application/json", data = "<topic_ids>")]
async fn topics(conn: TigumPgConn, topic_ids: Json<TopicIds>, auth_user: User) -> ApiResponse {
    get_topics(&conn, topic_ids, auth_user.id).await
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
