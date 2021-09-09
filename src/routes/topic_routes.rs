use crate::db;
use rocket::Route;
use rocket::http::Status;

//Use Macros
use rocket_contrib::json::Json;

// Database Models
use db::models::topic::{NewTopic, Topic, TopicIds};
use db::models::user::User;
use db::querys::TigumPgConn;

// Api Response Struct
use db::api_response::ApiResponse;

//Database Querys
use db::querys::topic_query::{
    create_topic,
    delete_topic,
    get_topic,
    get_topics,
    update_topic,
    // update_topic_mod_date
};

//////////////////////
//// TOPIC ROUTES ////
//////////////////////

#[delete("/topics/<topic_id>")]
fn delete_single_topic(conn: TigumPgConn, topic_id: i32) -> ApiResponse {
    delete_topic(&conn, topic_id)
}

#[put("/topics/<topic_id>", format = "application/json", data = "<topic>")]
fn update_single_topic(
    conn: TigumPgConn,
    topic_id: i32,
    topic: Json<Topic>,
) -> ApiResponse {
    update_topic(&conn, topic_id, topic)
}

#[put("/topics/update-mod-date/<topic_id>", format = "application/json")]
fn update_mod_date(
    conn: TigumPgConn,
    topic_id: i32
) -> ApiResponse {
    ApiResponse {
        json: json!("All good"),
        status: Status::raw(200)
    }
    // match update_topic_mod_date(&conn, topic_id) {
    //     Ok(_rows) => {
    //         ApiResponse {
    //             json: json!({"msg": "Success"}),
    //             status: Status::raw(200)
    //         }
    //     },
    //     Err(err) => {
    //         ApiResponse {
    //             json: json!({"error": format!("Could not update topic mod date")}),
    //             status: Status::raw(500)
    //         }
    //     }
    // }
}

#[post("/topics/create-topic", format = "application/json", data = "<topic>")]
fn create_single_topic(conn: TigumPgConn, topic: Json<NewTopic>, auth_user: User) -> ApiResponse {
    create_topic(&conn, topic, auth_user.id)
}

#[get("/topics/<topic_id>")]
fn single_topic(conn: TigumPgConn, topic_id: i32, auth_user: User) -> ApiResponse {
    get_topic(&conn, topic_id, auth_user.id)
}

#[post("/topics", format = "application/json", data = "<topic_ids>")]
fn topics(conn: TigumPgConn, topic_ids: Json<TopicIds>, auth_user: User) -> ApiResponse {
    get_topics(&conn, topic_ids, auth_user.id)
}

pub fn get_topic_routes() -> Vec<Route> {
    routes![
        delete_single_topic,
        update_single_topic,
        update_mod_date,
        create_single_topic,
        single_topic,
        topics
    ]
}
