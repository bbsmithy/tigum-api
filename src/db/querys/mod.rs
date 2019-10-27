use crate::db::models;
use rocket_contrib::databases;
use rocket_contrib::json::Json;

pub mod note_q;
pub mod topic_q;
pub mod video_q;

use models::resources::note::{NewResource, Resource};
use models::topic::{NewTopic, Topic, TopicIds};
use models::{Id, Ids};

#[database("tigum_db")]
pub struct TigumPgConn(databases::postgres::Connection);

////////////////////////////
//// RESOURCE DB QUERYS ////
////////////////////////////

pub fn delete_resource(conn: &TigumPgConn, resource_id: i32) -> Json<String> {
    let update = conn
        .execute("DELETE FROM resources WHERE id = $1", &[&resource_id])
        .unwrap();
    Json(format!("{} rows affected", update))
}

pub fn update_resource(
    conn: &TigumPgConn,
    resource_id: i32,
    resource: Json<Resource>,
) -> Json<Resource> {
    conn.execute(
        "UPDATE resources SET content = $2 WHERE id = $1",
        &[&resource_id, &resource.content],
    )
    .unwrap();
    get_resource(conn, resource_id)
}

pub fn get_resources(conn: &TigumPgConn, resource_ids: Json<Ids>) -> Json<Vec<Resource>> {
    let query_result = conn
        .query(
            "SELECT * FROM resources WHERE id = ANY($1)",
            &[&resource_ids.ids],
        )
        .unwrap();
    let mut results: Vec<Resource> = vec![];
    for row in query_result.iter() {
        let resource = Resource {
            resource_id: row.get(0),
            date_created: row.get(4),
            content_type: row.get(1),
            content: row.get(2),
            generated_by: row.get(3),
            thumbnail_img: row.get(6),
            title: row.get(5),
        };
        results.push(resource);
    }
    Json(results)
}

pub fn get_resource(conn: &TigumPgConn, resource_id: i32) -> Json<Resource> {
    let query_result = conn
        .query("SELECT * FROM resources WHERE id = $1", &[&resource_id])
        .unwrap();
    let row = query_result.get(0);
    let resource_response = Resource {
        resource_id: row.get(0),
        date_created: row.get(4),
        content_type: row.get(1),
        content: row.get(2),
        generated_by: row.get(3),
        title: row.get(5),
        thumbnail_img: row.get(6),
    };
    Json(resource_response)
}

pub fn create_resource(conn: &TigumPgConn, resource: Json<NewResource>) -> Json<Id> {
    let inserted_row = conn
        .query(
            "INSERT INTO resources (content_type, content, generated_by, title, thumbnail_img) VALUES ($1, $2, $3, $4, $5) RETURNING id",
            &[
                &resource.content_type,
                &resource.content,
                &resource.generated_by,
                &resource.title,
                &resource.thumbnail_img
            ],
        )
        .unwrap();
    let row = inserted_row.get(0);
    let resource_id: i32 = row.get(0);

    let id_response = Id { id: resource_id };

    Json(id_response)
}
