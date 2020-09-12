//Use Macros
use crate::db;
use rocket::Route;
use rocket_contrib::json::Json;
use rocket::http::{Status};

use db::models::resources::question::{Code, NewCode};
use db::models::resources::ResourceType;
use db::models::{Ids};
use db::models::user::User;

use db::api_response::ApiResponse;

use db::querys::question_query::{create_question, delete_question, get_question, get_questions, update_question};
use db::querys::topic_query::add_to_topic_resource_list;
use db::querys::TigumPgConn;

/////////////////////
//// CODE ROUTES ////
/////////////////////

#[delete("/question/<id>")]
fn delete_single_question(conn: TigumPgConn, id: i32, auth_user: User) -> Json<String> {
    delete_question(&conn, id, auth_user.id)
}

#[put("/question/<id>", format = "application/json", data = "<question>")]
fn update_single_question(conn: TigumPgConn, id: i32, question: Json<NewCode>, auth_user: User) -> Json<Code> {
    update_question(&conn, id, question, auth_user.id)
}

#[post("/question/create", format = "application/json", data = "<question>")]
fn create_single_question(conn: TigumPgConn, question: Json<NewCode>, auth_user: User) -> ApiResponse {
    let create_question_query_result = create_question(&conn, &question, auth_user.id);
    match create_question_query_result {
        Ok(new_question) => {
            let query_result = add_to_topic_resource_list(
                &conn,
                new_question.topic_id,
                new_question.id,
                ResourceType::Snippet,
            );
            match query_result {
                Ok(_rows_updated) => ApiResponse { json: json!(new_question), status: Status::raw(200) },
                Err(_error) => ApiResponse {
                    json: json!({ "error": format!("Could not create snippet {}", new_question.topic_id )}),
                    status: Status::raw(500)
                }
            }
        },  
        Err(_error) => ApiResponse {
            json: json!({
                "error": format!("Could not create snippet {}", question.topic_id )
            }),
            status: Status::raw(500)
        }
    }
}

#[get("/question/<id>")]
fn single_question(conn: TigumPgConn, id: i32, auth_user: User) -> Json<Code> {
    get_question(&conn, id, auth_user.id)
}

#[post("/question", format = "application/json", data = "<ids>")]
fn question(conn: TigumPgConn, ids: Json<Ids>, auth_user: User) -> Json<Vec<Code>> {
    get_questions(&conn, ids, auth_user.id)
}

pub fn get_question_routes() -> Vec<Route> {
    routes![
        create_single_question,
        question,
        single_question,
        update_single_question,
        delete_single_question
    ]
}
