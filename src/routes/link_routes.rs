use crate::db;
use rocket::Route;

//Use Macros
use rocket_contrib::json::Json;

use db::models::resources::link::{Link, NewLink};
use db::models::resources::ResourceType;
use db::models::Ids;
use db::models::user::User;

use db::querys::link_query::{
    create_link, delete_link, get_link, get_links, update_link,
};
use db::querys::topic_query::update_topic_resource_list;
use db::querys::TigumPgConn;

//////////////////////////
//// DOCUMENT ROUTES /////
//////////////////////////

#[delete("/links/<id>")]
pub fn delete_single_link(conn: TigumPgConn, id: i32, auth_user: User) -> Json<String> {
    delete_link(&conn, id, auth_user.id)
}

#[put("/links/<id>", format = "application/json", data = "<link>")]
pub fn update_single_link(
    conn: TigumPgConn,
    id: i32,
    link: Json<NewLink>,
    auth_user: User
) -> Json<Link> {
    update_link(&conn, id, link, auth_user.id)
}

#[post("/links/create", format = "application/json", data = "<link>")]
pub fn create_single_link(conn: TigumPgConn, link: Json<NewLink>, auth_user: User) -> Json<Link> {
    let new_link = create_link(&conn, &link, auth_user.id);
    update_topic_resource_list(
        &conn,
        link.topic_id,
        new_link.id,
        ResourceType::Link,
    );
    new_link
}

#[get("/links/<id>")]
pub fn single_link(conn: TigumPgConn, id: i32, auth_user: User) -> Json<Link> {
    get_link(&conn, id, auth_user.id)
}

#[post("/links", format = "application/json", data = "<ids>")]
pub fn links(conn: TigumPgConn, ids: Json<Ids>, auth_user: User) -> Json<Vec<Link>> {
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
