use crate::db::models::user::{User};
use crate::db::models::topic::{NewTopic, Topic, TopicIds};
use crate::db::models::resources::note::{Note};
use rocket_contrib::databases::postgres::row::Row;


pub fn row_to_user(row: &rocket_contrib::databases::postgres::Row) -> User {
    User {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
        email_hash: row.get(4)
    }
}

pub fn parse_topic_result(query_result: Vec<rocket_contrib::databases::postgres::row::Row>) -> Vec<Topic> {
    let mut results: Vec<Topic> = vec![];
    for row in query_result {
        let topic = row_to_topic(&row);
        results.push(topic);
    }
    results
}

pub fn row_to_topic(row: &Row) -> Topic {
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
        row.get(11)
    );
    return topic;
}

pub fn row_to_note(row: &rocket_contrib::databases::postgres::Row) -> Note {
    Note::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4), row.get(5))
}

pub fn parse_note_result(query_result: Vec<rocket_contrib::databases::postgres::Row>) -> Vec<Note> {
    let mut results: Vec<Note> = vec![];
    for row in query_result.iter() {
        let note = Note::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4), row.get(5));
        results.push(note);
    }
    results
}
