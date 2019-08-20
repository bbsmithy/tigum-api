use rocket_contrib::databases;
use rocket_contrib::json::Json;


pub mod models;
use models::topic::note::{Note, Resource};
use models::topic::Topic;

#[database("tigum_db")]
pub struct TigumPgConn(databases::postgres::Connection);

pub fn get_topic(conn: &TigumPgConn, topic_id: i32) -> Json<Vec<Topic>> {
    let query_result = conn.query("SELECT * FROM topics WHERE id = ($1)", &[&topic_id]).unwrap();
    let mut result: Vec<Topic> = vec![];
    for row in query_result.iter() {
        let topic = Topic::new(row.get(2), row.get(1), row.get(0));
        result.push(topic);
    }
    return Json(result)
}

pub fn create_topic(conn: &TigumPgConn, topic: Json<Topic>) -> Json<Vec<Topic>> {
    let updates = conn.execute("INSERT INTO topics (id, title, date_created) VALUES ($1, $2, $3)",
                 &[&topic.topic_id, &topic.title, &topic.date_created]).unwrap();
    let results = get_topic(&conn, topic.topic_id);
    return results;
}

pub fn create_note(conn: &TigumPgConn, note: Json<Note>) -> String {
    let update = conn.execute("INSERT INTO notes (title, note_content) VALUES ($1, $2)", &[&note.title, &note.note_content]).unwrap();
    format!("Rows affected {}", update)
}


fn generate_test_resources(amount: i64) -> Vec<i64> {
    let mut resources: Vec<i64> = vec![];
    for n in 1..amount {
        resources.push(n);
    }
    return resources;
}

pub fn generate_test_notes(amount: u64) -> Vec<Note> {
    let mut notes: Vec<Note> = vec![];

    for n in 1..amount {
        let note_title = format!("Note Title Test {}", &n.to_string());
        let note_content = generate_test_resources(10);
        let new_note = Note::new(note_title, n, note_content);
        notes.push(new_note);
    }

    return notes;
}

pub fn generate_single_note() -> Note {
    let resources = generate_test_resources(10);
    let note = Note::new(String::from("Test Single note"), 1234, resources);

    note
}

pub fn generate_test_topics(amount: i32) -> Vec<Topic> {
    let mut topics: Vec<Topic> = vec![];

    for n in 1..amount {
        let note_title = format!("Topic Title Test {}", &amount.to_string());
        let date = "12th Test 2019".to_string();
        let new_topic = Topic::new(note_title, date, n);
        topics.push(new_topic);
    }

    return topics;
}

pub fn generate_single_topic(topic_id: i32) -> Topic {
    let title = "Test Single Topic".to_string();
    let date_created = "12th of Never".to_string();

    let topic = Topic::new(title, date_created, topic_id);

    topic
}