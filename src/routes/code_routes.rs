//Use Macros
use crate::db;
use rocket::Route;
use rocket_contrib::json::Json;
use rocket::http::{Status};

use db::models::resources::code::{Code, NewCode};
use db::models::resources::ResourceType;
use db::models::{Ids};
use db::models::user::User;

use db::api_response::ApiResponse;

use db::querys::code_query::{create_code, delete_code, get_code, get_codes, update_code};
use db::querys::topic_query::add_to_topic_resource_list;
use db::querys::TigumPgConn;

/////////////////////
//// CODE ROUTES ////
/////////////////////

#[delete("/code/<id>")]
fn delete_single_code(conn: TigumPgConn, id: i32, auth_user: User) -> Json<String> {
    delete_code(&conn, id, auth_user.id)
}

#[put("/code/<id>", format = "application/json", data = "<code>")]
fn update_single_code(conn: TigumPgConn, id: i32, code: Json<NewCode>, auth_user: User) -> Json<Code> {
    update_code(&conn, id, code, auth_user.id)
}

#[post("/code/create", format = "application/json", data = "<code>")]
fn create_single_code(conn: TigumPgConn, code: Json<NewCode>, auth_user: User) -> ApiResponse {
    let create_code_query_result = create_code(&conn, &code, auth_user.id);
    match create_code_query_result {
        Ok(new_code) => {
            let query_result = add_to_topic_resource_list(
                &conn,
                new_code.topic_id,
                new_code.id,
                ResourceType::Snippet,
            );
            match query_result {
                Ok(_rows_updated) => ApiResponse { json: json!(new_code), status: Status::raw(200) },
                Err(_error) => ApiResponse {
                    json: json!({ "error": format!("Could not create snippet {}", new_code.topic_id )}),
                    status: Status::raw(500)
                }
            }
        },  
        Err(_error) => ApiResponse {
            json: json!({
                "error": format!("Could not create snippet {}", code.topic_id )
            }),
            status: Status::raw(500)
        }
    }
}

#[get("/code/<id>")]
fn single_code(conn: TigumPgConn, id: i32, auth_user: User) -> Json<Code> {
    get_code(&conn, id, auth_user.id)
}

#[post("/code", format = "application/json", data = "<ids>")]
fn code(conn: TigumPgConn, ids: Json<Ids>, auth_user: User) -> Json<Vec<Code>> {
    get_codes(&conn, ids, auth_user.id)
}

pub fn get_code_routes() -> Vec<Route> {
    routes![
        create_single_code,
        code,
        single_code,
        update_single_code,
        delete_single_code
    ]
}
