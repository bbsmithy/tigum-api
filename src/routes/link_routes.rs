use crate::db;
use rocket::Route;
use rocket::http::Status;

//Use Macros
use rocket_contrib::json::Json;

use db::models::resources::link::{Link, NewLink};
use db::models::resources::ResourceType;
use db::models::Ids;
use db::models::user::User;

use db::api_response::ApiResponse;

use db::querys::link_query::{
    create_link, delete_link, get_link, get_links, update_link,
};
use db::querys::topic_query::add_to_topic_resource_list;
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
) -> Json<Link> {
    update_link(&conn, id, link, auth_user.id)
}

#[post("/links/create", format = "application/json", data = "<link>")]
pub fn create_single_link(conn: TigumPgConn, link: Json<NewLink>, auth_user: User) -> ApiResponse {
    let create_link_query_result = create_link(&conn, &link, auth_user.id);
    match create_link_query_result {
        Ok(new_link) => {
            let query_result = add_to_topic_resource_list(
                &conn,
                link.topic_id,
                new_link.id,
                ResourceType::Link,
            );
            match query_result {
                Ok(_rows_updated) => ApiResponse { json: json!(new_link), status: Status::raw(200) },
                Err(_error) => ApiResponse {
                    json: json!({ "error": format!("Could not create snippet {}", new_link.topic_id )}),
                    status: Status::raw(500)
                }
            }
        },  
        Err(_error) => ApiResponse {
            json: json!({
                "error": format!("Could not create snippet {}", link.topic_id )
            }),
            status: Status::raw(500)
        }
    }
    
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
