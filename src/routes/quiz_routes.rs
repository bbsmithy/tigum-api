use crate::db;
use rocket::Route;
use rocket::http::{RawStr, Status};

//Use Macros
use rocket_contrib::json::Json;

// Database Models
use db::models::topic::{NewTopic, TopicIds};
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
    update_topic_title,
    update_topic_mod_date
};

//////////////////////
//// QUIZ ROUTES /////
//////////////////////




#[delete("/quiz/<quiz_id>")]
fn delete_single_topic(conn: TigumPgConn, quiz_id: i32) -> ApiResponse {
    ApiResponse {
        json: json!({ "msg": "Test" }),
        status: Status::raw(200)
    }
}