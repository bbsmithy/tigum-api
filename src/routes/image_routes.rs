use crate::db;
use crate::User;
use rocket::Route;

//Use Macros
use rocket_contrib::json::Json;

use db::models::resources::image::{NewImage, Image};
use db::models::{Id, Ids};
use db::querys::image_query::{create_image, delete_image, get_image, get_images, update_image};
use db::querys::topic_query::update_topic_resource_list;
use db::querys::TigumPgConn;
use db::models::resources::ResourceType;


/////////////////////////
//// IMAGE ROUTES ///////
/////////////////////////

#[delete("/images/<id>")]
pub fn delete_single_image(conn: TigumPgConn, id: i32, _auth_user: User) -> Json<String> {
    delete_image(&conn, id)
}

#[put("/images/<id>", format = "application/json", data = "<image>")]
pub fn update_single_image(conn: TigumPgConn, id: i32, image: Json<NewImage>) -> Json<Image> {
    update_image(&conn, id, image)
}

#[post("/images/create", format = "application/json", data = "<image>")]
pub fn create_single_image(conn: TigumPgConn, image: Json<NewImage>) -> Json<Id> {
    let new_image = create_image(&conn, &image);
    update_topic_resource_list(
        &conn,
        image.topic_id,
        new_image.id,
        ResourceType::Image,
    );
    new_image
}

#[get("/images/<id>")]
pub fn single_image(conn: TigumPgConn, id: i32, _auth_user: User) -> Json<Image> {
    get_image(&conn, id)
}

#[post("/images", format = "application/json", data = "<ids>")]
pub fn images(conn: TigumPgConn, ids: Json<Ids>) -> Json<Vec<Image>> {
    println!("{:?}", ids);
    get_images(&conn, ids)
}

pub fn image_routes() -> Vec<Route> {
    routes![
        create_single_image,
        delete_single_image,
        single_image,
        images,
        update_single_image,
    ]
}
