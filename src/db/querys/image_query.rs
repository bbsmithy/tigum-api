//Use Macros
use rocket_contrib::json::Json;

use crate::db::models;
use crate::db::querys::TigumPgConn;

use models::resources::image::{Image, NewImage};
use models::{Id, Ids};

fn row_to_image(row: rocket_contrib::databases::postgres::rows::Row) -> Image {
    Image {
        id: row.get(0),
        topic_id: row.get(1),
        user_id: row.get(2),
        src: row.get(3),
        origin: row.get(4),
        date_created: row.get(5),
    }
}

pub fn delete_image(conn: &TigumPgConn, id: i32) -> Json<String> {
    let update = conn
        .execute("DELETE FROM images WHERE id = $1", &[&id])
        .unwrap();
    Json(format!("{} rows affected", update))
}

pub fn update_image(
    conn: &TigumPgConn,
    id: i32,
    image: Json<NewImage>,
) -> Json<Image> {
    let updated_rows = conn.query(
        "UPDATE images SET topic_id = $2, user_id = $3, src = $4, origin = $5 WHERE id = $1 RETURNING *",
        &[&id, &image.topic_id, &image.user_id, &image.src, &image.origin],
    ).unwrap();

    let image_response = row_to_image(updated_rows.get(0));

    Json(image_response)
}

pub fn get_images(conn: &TigumPgConn, ids: Json<Ids>) -> Json<Vec<Image>> {
    println!("{:?}", ids);
    let query_result = conn
        .query(
            "SELECT * FROM images WHERE id = ANY($1)",
            &[&ids.ids],
        )
        .unwrap();
    let mut results: Vec<Image> = vec![];
    for row in query_result.iter() {
        let image_response = row_to_image(row);
        results.push(image_response);
    }
    Json(results)
}

pub fn get_image(conn: &TigumPgConn, id: i32) -> Json<Image> {
    let query_result = conn
        .query("SELECT * FROM images WHERE id = $1", &[&id])
        .unwrap();
    println!("{:#?}", query_result);
    let image_response = row_to_image(query_result.get(0));
    Json(image_response)
}

pub fn create_image(
    conn: &TigumPgConn,
    image: Json<NewImage>,
) -> Json<Id> {
    let inserted_row = conn
        .query(
            "INSERT INTO images (src, origin, topic_id, user_id) VALUES ($1, $2, $3, $4) RETURNING id",
            &[
                &image.src,
                &image.origin,
                &image.topic_id,
                &image.user_id,
            ],
        )
        .unwrap();
    let row = inserted_row.get(0);
    println!("{:#?}", row);
    let id: i32 = row.get(0);

    let id_response = Id { id: id };

    Json(id_response)
}
