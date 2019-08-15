use rocket_contrib::databases;
use rocket_contrib::json::Json;


pub mod models;
use models::topic::note::{Note, Resource};
use models::topic::Topic;

#[database("tigum_db")]
pub struct TigumPgConn(databases::postgres::Connection);

pub fn create_topic(conn: &TigumPgConn, topic: Json<Topic>) -> String {
    let updates = conn.execute("INSERT INTO topics (title, date_created) VALUES ($1, $2)",
                 &[&topic.title, &topic.date_created]).unwrap();
    format!("Rows affected {}", updates)
}


fn generate_test_resources(amount: u64) -> Vec<Resource> {
    let mut resources: Vec<Resource> = vec![];
    for n in 1..amount {
        let content_type = "TEXT".to_string();
        let content = "<h1>Hello</h1>".to_string();
        let new_resource = Resource::new(n, content_type, content);
        resources.push(new_resource);
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

pub fn generate_test_topics(amount: u64) -> Vec<Topic> {
    let mut topics: Vec<Topic> = vec![];

    for n in 1..amount {
        let note_title = format!("Topic Title Test {}", &amount.to_string());
        let date = "12th Test 2019".to_string();
        let new_topic = Topic::new(note_title, date, n);
        topics.push(new_topic);
    }

    return topics;
}

pub fn generate_single_topic(topic_id: u64) -> Topic {
    let title = "Test Single Topic".to_string();
    let date_created = "12th of Never".to_string();

    let topic = Topic::new(title, date_created, topic_id);

    topic
}