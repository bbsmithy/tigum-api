//Use Macros
use crate::db;
use rocket::Route;
use rocket_contrib::json::Json;

use db::models::resources::code::{Code, NewCode};
use db::models::resources::ResourceType;
use db::models::{Id, Ids};
use db::models::user::User;

use db::querys::code_query::{create_code, delete_code, get_code, get_codes, update_code};
use db::querys::topic_query::update_topic_resource_list;
use db::querys::TigumPgConn;

/////////////////////
//// CODE ROUTES ////
/////////////////////

#[delete("/code/<id>")]
fn delete_single_code(conn: TigumPgConn, id: i32, auth_user: User) -> Json<String> {
    delete_code(&conn, id)
}

#[put("/code/<id>", format = "application/json", data = "<code>")]
fn update_single_code(conn: TigumPgConn, id: i32, code: Json<NewCode>) -> Json<Code> {
    update_code(&conn, id, code)
}

#[post("/code/create", format = "application/json", data = "<code>")]
fn create_single_code(conn: TigumPgConn, code: Json<NewCode>) -> Json<Id> {
    let new_code = create_code(&conn, &code);
    update_topic_resource_list(&conn, code.topic_id, new_code.id, ResourceType::Code);
    return new_code;
}

#[get("/code/<id>")]
fn single_code(conn: TigumPgConn, id: i32, auth_user: User) -> Json<Code> {
    get_code(&conn, id)
}

#[post("/code", format = "application/json", data = "<ids>")]
fn code(conn: TigumPgConn, ids: Json<Ids>) -> Json<Vec<Code>> {
    println!("{:?}", ids);
    get_codes(&conn, ids)
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
