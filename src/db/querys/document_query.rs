//Use Macros
use rocket_contrib::json::Json;

use crate::db::models;
use crate::db::querys::TigumPgConn;

use models::resources::documents::{Document, NewDocument};
use models::{Id, Ids};

fn row_to_document(row: rocket_contrib::databases::postgres::rows::Row) -> Document {
    Document {
        id: row.get(0),
        title: row.get(1),
        user_id: row.get(2),
        topic_id: row.get(3),
        extension: row.get(4),
        origin: row.get(5),
        date_created: row.get(6),
        source: row.get(7)
    }
}

pub fn delete_document(conn: &TigumPgConn, id: i32) -> Json<String> {
    let update = conn
        .execute("DELETE FROM documents WHERE id = $1", &[&id])
        .unwrap();
    Json(format!("{} rows affected", update))
}

pub fn update_document(
    conn: &TigumPgConn,
    id: i32,
    document: Json<NewDocument>,
) -> Json<Document> {
    let updated_rows = conn.query(
        "UPDATE documents SET topic_id = $2, user_id = $3, title = $4, origin = $5, extension = $6 WHERE id = $1 RETURNING *",
        &[&id, &document.topic_id, &document.user_id, &document.title, &document.origin, &document.extension],
    ).unwrap();

    let document_response = row_to_document(updated_rows.get(0));

    Json(document_response)
}

pub fn get_documents(conn: &TigumPgConn, ids: Json<Ids>) -> Json<Vec<Document>> {
    println!("{:?}", ids);
    let query_result = conn
        .query(
            "SELECT * FROM documents WHERE id = ANY($1)",
            &[&ids.ids],
        )
        .unwrap();
    let mut results: Vec<Document> = vec![];
    for row in query_result.iter() {
        let document_response = row_to_document(row);
        results.push(document_response);
    }
    Json(results)
}

pub fn get_document(conn: &TigumPgConn, id: i32) -> Json<Document> {
    let query_result = conn
        .query("SELECT * FROM documents WHERE id = $1", &[&id])
        .unwrap();
    println!("{:#?}", query_result);
    let document_response = row_to_document(query_result.get(0));
    Json(document_response)
}

pub fn create_document(
    conn: &TigumPgConn,
    document: &Json<NewDocument>,
) -> Json<Id> {
    let inserted_row = conn
        .query(
            "INSERT INTO documents (title, origin, topic_id, user_id, extension, source) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
            &[
                &document.title,
                &document.origin,
                &document.topic_id,
                &document.user_id,
                &document.extension,
                &document.source
            ],
        )
        .unwrap();

    let row = inserted_row.get(0);
    println!("{:#?}", row);
    let id: i32 = row.get(0);

    let id_response = Id { id: id };

    Json(id_response)
}
