//Use Macros
use rocket_contrib::json::Json;
use rocket_contrib::databases::postgres::Error;

use crate::db::models;
use crate::db::querys::TigumPgConn;

use models::resources::question::{Code, NewCode};
use models::{Ids};

fn row_to_question(row: rocket_contrib::databases::postgres::rows::Row) -> Code {
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

pub fn delete_question(conn: &TigumPgConn, id: i32, user_id: i32) -> Json<String> {
    let update = conn
        .execute("DELETE FROM question WHERE id = $1 AND user_id = ", &[&id, &user_id])
        .unwrap();
    Json(format!("{} rows affected", update))
}

pub fn update_question(conn: &TigumPgConn, id: i32, question: Json<NewCode>, user_id: i32) -> Json<Code> {
    let updated_rows = conn.query(
        "UPDATE question SET topic_id = $2, user_id = $3, content = $4, origin = $5, language = $6 WHERE id = $1 RETURNING *",
        &[&id, &question.topic_id, &user_id, &question.content, &question.origin, &question.language],
    ).unwrap();

    let question_response = row_to_question(updated_rows.get(0));

    Json(question_response)
}

pub fn get_questions(conn: &TigumPgConn, ids: Json<Ids>, user_id: i32) -> Json<Vec<Code>> {
    println!("{:?}", ids);
    let query_result = conn
        .query("SELECT * FROM question WHERE id = ANY($1) AND user_id = $2", &[&ids.ids, &user_id])
        .unwrap();
    let mut results: Vec<Code> = vec![];
    for row in query_result.iter() {
        let question_response = row_to_question(row);
        results.push(question_response);
    }
    Json(results)
}

pub fn get_question(conn: &TigumPgConn, id: i32, user_id: i32) -> Json<Code> {
    let query_result = conn
        .query("SELECT * FROM question WHERE id = $1 AND user_id = $2", &[&id, &user_id])
        .unwrap();
    println!("{:#?}", query_result);
    let question_response = row_to_question(query_result.get(0));
    Json(question_response)
}

pub fn create_question(conn: &TigumPgConn, question: &Json<NewCode>, user_id: i32) -> Result<Code, Error> {
    let inserted_row = conn.query(
            "INSERT INTO question (content, language, origin, topic_id, user_id) VALUES ($1, $2, $3, $4, $5) RETURNING id",
            &[
                &question.content,
                &question.language,
                &question.origin,
                &question.topic_id,
                &user_id,
            ],
        );
    match inserted_row {
        Ok(question_row) => {
            Ok(row_to_question(question_row.get(0)))
        },
        Err(error) => Err(error)
    }
}
