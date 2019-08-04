pub mod topic;
pub mod user;

use topic::note::{Note, Resource};
use topic::Topic;

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
