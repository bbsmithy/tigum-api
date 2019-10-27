use crate::db::models;
use crate::db::querys::TigumPgConn;
use rocket_contrib::json::Json;

use models::topic::{NewTopic, Topic, TopicIds};

fn parse_topic_result(query_result: rocket_contrib::databases::postgres::rows::Rows) -> Vec<Topic> {
    let mut results: Vec<Topic> = vec![];
    for row in query_result.iter() {
        let topic = Topic::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4));
        results.push(topic);
    }
    results
}

pub fn delete_topic(conn: &TigumPgConn, topic_id: i32) -> String {
    let result = conn
        .execute("DELETE FROM topics WHERE id = $1", &[&topic_id])
        .unwrap();
    format!("{} rows deleted", result)
}

pub fn update_topic(conn: &TigumPgConn, topic_id: i32, topic: Json<Topic>) -> Json<Topic> {
    conn.execute(
        "UPDATE topics SET title = ($2), notes = ($3), resources = ($4) WHERE id = ($1)",
        &[&topic_id, &topic.title, &topic.notes, &topic.resources],
    )
    .unwrap();
    get_topic(&conn, topic_id)
}

pub fn get_topics(conn: &TigumPgConn, topic_ids: Json<TopicIds>) -> Json<Vec<Topic>> {
    if topic_ids.ids.len() == 0 {
        let query_result = conn.query("SELECT * FROM topics", &[]).unwrap();
        let result = parse_topic_result(query_result);
        Json(result)
    } else {
        let query_result = conn
            .query("SELECT * FROM topics WHERE id = ANY($1)", &[&topic_ids.ids])
            .unwrap();
        let results = parse_topic_result(query_result);
        Json(results)
    }
}

pub fn get_topic(conn: &TigumPgConn, topic_id: i32) -> Json<Topic> {
    let query_result = conn
        .query("SELECT * FROM topics WHERE id = $1", &[&topic_id])
        .unwrap();
    let topic = query_result.get(0);
    let result = Topic::new(
        topic.get(0),
        topic.get(1),
        topic.get(2),
        topic.get(3),
        topic.get(4),
    );
    Json(result)
}

pub fn create_topic(conn: &TigumPgConn, topic: Json<NewTopic>) -> String {
    let update = conn
        .execute(
            "INSERT INTO topics (title, notes, resources) VALUES ($1, $2, $3)",
            &[&topic.title, &topic.notes, &topic.resources],
        )
        .unwrap();
    format!("{} rows affected", update)
}
