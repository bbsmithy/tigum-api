use crate::db::models::user::{User};
use crate::db::models::topic::{NewTopic, Topic, TopicIds};
use crate::db::models::resources::note::{Note};
use crate::db::models::resources::article_snippets::ArticleSnippet;
use crate::db::models::resources::link::Link;
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

pub fn row_to_article_snippet(row: &rocket_contrib::databases::postgres::Row) -> ArticleSnippet {
    ArticleSnippet {
        id: row.get(0),
        topic_id: row.get(4),
        user_id: row.get(5),
        content: row.get(1),
        origin: row.get(2),
        title: row.get(6),
        date_created: row.get(3),
    }
}

pub fn parse_article_snippet_result(query_result: Vec<rocket_contrib::databases::postgres::Row>) -> Vec<ArticleSnippet> {
    let mut results: Vec<ArticleSnippet> = vec![];
    for row in query_result.iter() {
        let article = row_to_article_snippet(&row);
        results.push(article);
    }
    results
}

fn row_to_link(row: &rocket_contrib::databases::postgres::Row) -> Link {
    Link {
        id: row.get(0),
        title: row.get(1),
        user_id: row.get(2),
        topic_id: row.get(3),
        date_created: row.get(4),
        source: row.get(5),
    }
}

pub fn parse_link_result(query_result: Vec<rocket_contrib::databases::postgres::Row>) -> Vec<Link> {
    let mut results: Vec<Link> = vec![];
    for row in query_result.iter() {
        let link = row_to_link(&row);
        results.push(link);
    }
    results
}
