use crate::db::models;
use crate::db::querys::TigumPgConn;
use rocket_contrib::json::Json;

use models::resources::ResourceType;
use models::topic::{NewTopic, Topic, TopicIds};

fn parse_topic_result(query_result: rocket_contrib::databases::postgres::rows::Rows) -> Vec<Topic> {
    let mut results: Vec<Topic> = vec![];
    for row in query_result.iter() {
        let topic = row_to_topic(row);
        results.push(topic);
    }
    results
}

fn row_to_topic(row: rocket_contrib::databases::postgres::rows::Row) -> Topic {
    println!("{:?}", row);
    let topic = Topic::new(
        row.get(0),
        row.get(1),
        row.get(2),
        row.get(3),
        row.get(4),
        row.get(5),
        row.get(6),
        row.get(7),
        row.get(8),
        row.get(10),
        123,
    );
    return topic;
}

pub fn delete_topic(conn: &TigumPgConn, topic_id: i32) -> String {
    let result = conn
        .execute("DELETE FROM topics WHERE id = $1", &[&topic_id])
        .unwrap();
    format!("{} rows deleted", result)
}

pub fn update_topic_resource_list(
    conn: &TigumPgConn,
    topic_id: i32,
    resource_id: i32,
    resource_type: ResourceType,
) {
    let result = match resource_type {
        ResourceType::Snippet => conn.execute("UPDATE topics SET article_snippets = array_append(article_snippets, $1) WHERE id = ($2)", &[&resource_id, &topic_id]),
        ResourceType::Link => conn.execute("UPDATE topics SET links = array_append(links, $1) WHERE id = ($2)", &[&resource_id, &topic_id]),
        ResourceType::Image => conn.execute("UPDATE topics SET images = array_append(images, $1) WHERE id = ($2)", &[&resource_id, &topic_id]),
        ResourceType::Note => conn.execute("UPDATE topics SET notes = array_append(notes, $1) WHERE id = ($2)", &[&resource_id, &topic_id]),
        ResourceType::Video => conn.execute("UPDATE topics SET videos = array_append(videos, $1) WHERE id = ($2)", &[&resource_id, &topic_id]),
        ResourceType::Code => conn.execute("UPDATE topics SET code = array_append(code, $1) WHERE id = ($2)", &[&resource_id, &topic_id]),
        _ => Ok(1)
    };
    print!("{:?}", result);
}

pub fn update_topic(conn: &TigumPgConn, topic_id: i32, topic: Json<Topic>) -> Json<Topic> {
    let updated_topic_rows = conn
        .query(
            "UPDATE topics SET title = ($2), notes = ($3), videos = ($4) WHERE id = ($1) RETURNING *",
            &[&topic_id, &topic.title, &topic.notes, &topic.videos],
        )
        .unwrap();
    println!("{:?}", updated_topic_rows);
    let result = row_to_topic(updated_topic_rows.get(0));
    Json(result)
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
    let row = query_result.get(0);
    let result = row_to_topic(row);
    Json(result)
}

pub fn create_topic(conn: &TigumPgConn, topic: Json<NewTopic>) -> Json<Topic> {
    let result = conn
        .query(
            "INSERT INTO topics (title, user_id) VALUES ($1, $2) RETURNING *",
            &[&topic.title, &topic.user_id],
        )
        .unwrap();
    let new_topic = row_to_topic(result.get(0));
    Json(new_topic)
}
