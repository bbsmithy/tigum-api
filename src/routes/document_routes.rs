use crate::db;
use crate::User;
use rocket::Route;

//Use Macros
use rocket_contrib::json::Json;

use db::models::resources::documents::{Document, NewDocument};
use db::models::resources::ResourceType;
use db::models::{Id, Ids};
use db::querys::document_query::{
    create_document, delete_document, get_document, get_documents, update_document,
};
use db::querys::topic_query::update_topic_resource_list;
use db::querys::TigumPgConn;

//////////////////////////
//// DOCUMENT ROUTES /////
//////////////////////////

#[delete("/documents/<id>")]
pub fn delete_single_document(conn: TigumPgConn, id: i32, _auth_user: User) -> Json<String> {
    delete_document(&conn, id)
}

#[put("/documents/<id>", format = "application/json", data = "<document>")]
pub fn update_single_document(
    conn: TigumPgConn,
    id: i32,
    document: Json<NewDocument>,
) -> Json<Document> {
    update_document(&conn, id, document)
}

#[post("/documents/create", format = "application/json", data = "<document>")]
pub fn create_single_document(conn: TigumPgConn, document: Json<NewDocument>) -> Json<Id> {
    let new_document = create_document(&conn, &document);
    update_topic_resource_list(
        &conn,
        document.topic_id,
        new_document.id,
        ResourceType::Document,
    );
    new_document
}

#[get("/documents/<id>")]
pub fn single_document(conn: TigumPgConn, id: i32, _auth_user: User) -> Json<Document> {
    get_document(&conn, id)
}

#[post("/documents", format = "application/json", data = "<ids>")]
pub fn documents(conn: TigumPgConn, ids: Json<Ids>) -> Json<Vec<Document>> {
    println!("{:?}", ids);
    get_documents(&conn, ids)
}

pub fn document_routes() -> Vec<Route> {
    routes![
        create_single_document,
        delete_single_document,
        single_document,
        documents,
        update_single_document,
    ]
}
