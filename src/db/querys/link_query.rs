//Use Macros
use rocket_contrib::json::Json;

use crate::db::models;
use crate::db::querys::TigumPgConn;

use models::resources::link::{Link, NewLink};
use models::Ids;

fn row_to_link(row: rocket_contrib::databases::postgres::rows::Row) -> Link {
    Link {
        id: row.get(0),
        title: row.get(1),
        user_id: row.get(2),
        topic_id: row.get(3),
        date_created: row.get(4),
        source: row.get(5),
    }
}

pub fn delete_link(conn: &TigumPgConn, id: i32, user_id: i32) -> Json<String> {
    let update = conn
        .execute(
            "DELETE FROM links WHERE id = $1 AND user_id = $2",
            &[&id, &user_id],
        )
        .unwrap();
    Json(format!("{} rows affected", update))
}

pub fn update_link(conn: &TigumPgConn, id: i32, link: Json<NewLink>, user_id: i32) -> Json<Link> {
    let updated_rows = conn
        .query(
            "UPDATE links SET topic_id = $2, user_id = $3, title = $4 WHERE id = $1 AND user_id = $5 RETURNING *",
            &[&id, &link.topic_id, &user_id, &link.title, &user_id],
        )
        .unwrap();

    let link_response = row_to_link(updated_rows.get(0));

    Json(link_response)
}

pub fn get_links(conn: &TigumPgConn, ids: Json<Ids>, user_id: i32) -> Json<Vec<Link>> {
    println!("{:?}", ids);
    let query_result = conn
        .query(
            "SELECT * FROM links WHERE id = ANY($1) AND user_id = $2",
            &[&ids.ids, &user_id],
        )
        .unwrap();
    let mut results: Vec<Link> = vec![];
    for row in query_result.iter() {
        let link_response = row_to_link(row);
        results.push(link_response);
    }
    Json(results)
}

pub fn get_link(conn: &TigumPgConn, id: i32, user_id: i32) -> Json<Link> {
    let query_result = conn
        .query(
            "SELECT * FROM links WHERE id = $1 AND user_id = $2",
            &[&id, &user_id],
        )
        .unwrap();
    let link_response = row_to_link(query_result.get(0));
    Json(link_response)
}

pub fn create_link(conn: &TigumPgConn, link: &Json<NewLink>, user_id: i32) -> Json<Link> {
    let inserted_row = conn
        .query(
            "INSERT INTO links (title, topic_id, user_id, source) VALUES ($1, $2, $3, $4) RETURNING *",
            &[
                &link.title,
                &link.topic_id,
                &user_id,
                &link.source
            ],
        )
        .unwrap();

    let row = inserted_row.get(0);
    let link_response = row_to_link(row);

    Json(link_response)
}
