//Use Macros
use rocket_contrib::json::Json;

use crate::db::models;
use crate::db::querys::TigumPgConn;

use models::resources::code::{Code, NewCode};
use models::{Id, Ids};

fn row_to_code(row: rocket_contrib::databases::postgres::rows::Row) -> Code {
    Code {
        id: row.get(0),
        content: row.get(3),
        date_created: row.get(5),
        language: row.get(2),
        origin: row.get(6),
        topic_id: row.get(1),
        user_id: row.get(4),
    }
}

pub fn delete_code(conn: &TigumPgConn, id: i32, user_id: i32) -> Json<String> {
    let update = conn
        .execute("DELETE FROM code WHERE id = $1 AND user_id = ", &[&id, &user_id])
        .unwrap();
    Json(format!("{} rows affected", update))
}

pub fn update_code(conn: &TigumPgConn, id: i32, code: Json<NewCode>, user_id: i32) -> Json<Code> {
    let updated_rows = conn.query(
        "UPDATE code SET topic_id = $2, user_id = $3, content = $4, origin = $5, language = $6 WHERE id = $1 RETURNING *",
        &[&id, &code.topic_id, &user_id, &code.content, &code.origin, &code.language],
    ).unwrap();

    let code_response = row_to_code(updated_rows.get(0));

    Json(code_response)
}

pub fn get_codes(conn: &TigumPgConn, ids: Json<Ids>, user_id: i32) -> Json<Vec<Code>> {
    println!("{:?}", ids);
    let query_result = conn
        .query("SELECT * FROM code WHERE id = ANY($1) AND user_id = $2", &[&ids.ids, &user_id])
        .unwrap();
    let mut results: Vec<Code> = vec![];
    for row in query_result.iter() {
        let code_response = row_to_code(row);
        results.push(code_response);
    }
    Json(results)
}

pub fn get_code(conn: &TigumPgConn, id: i32, user_id: i32) -> Json<Code> {
    let query_result = conn
        .query("SELECT * FROM code WHERE id = $1 AND user_id = $2", &[&id, &user_id])
        .unwrap();
    println!("{:#?}", query_result);
    let code_response = row_to_code(query_result.get(0));
    Json(code_response)
}

pub fn create_code(conn: &TigumPgConn, code: &Json<NewCode>, user_id: i32) -> Json<Id> {
    let inserted_row = conn
        .query(
            "INSERT INTO code (content, language, origin, topic_id, user_id) VALUES ($1, $2, $3, $4, $5) RETURNING id",
            &[
                &code.content,
                &code.language,
                &code.origin,
                &code.topic_id,
                &user_id,
            ],
        )
        .unwrap();
    let row = inserted_row.get(0);
    println!("{:#?}", row);
    let id: i32 = row.get(0);

    let id_response = Id { id: id };

    Json(id_response)
}
