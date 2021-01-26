use crate::db;
use rocket::Route;

//Use Macros
use rocket_contrib::json::Json;

use db::models::resources::link::{NewLink};
use db::models::Ids;
use db::models::user::User;

use db::api_response::ApiResponse;

use db::querys::link_query::{
    create_link, delete_link, get_link, get_links, update_link,
};
use db::querys::TigumPgConn;

//////////////////////
//// LINK ROUTES /////
//////////////////////

#[delete("/links/<id>")]
pub fn delete_single_link(conn: TigumPgConn, id: i32, auth_user: User) -> ApiResponse {
    delete_link(&conn, id, auth_user.id)
}

#[put("/links/<id>", format = "application/json", data = "<link>")]
pub fn update_single_link(
    conn: TigumPgConn,
    id: i32,
    link: Json<NewLink>,
    auth_user: User
) -> ApiResponse {
    update_link(&conn, id, link, auth_user.id)
}

#[post("/links/create", format = "application/json", data = "<link>")]
pub fn create_single_link(conn: TigumPgConn, link: Json<NewLink>, auth_user: User) -> ApiResponse {
    create_link(&conn, link, auth_user.id)
}

#[get("/links/<id>")]
pub fn single_link(conn: TigumPgConn, id: i32, auth_user: User) -> ApiResponse {
    get_link(&conn, id, auth_user.id)
}

#[post("/links", format = "application/json", data = "<ids>")]
pub fn links(conn: TigumPgConn, ids: Json<Ids>, auth_user: User) -> ApiResponse {
    get_links(&conn, ids, auth_user.id)
}

pub fn link_routes() -> Vec<Route> {
    routes![
        create_single_link,
        delete_single_link,
        single_link,
        links,
        update_single_link,
    ]
}
