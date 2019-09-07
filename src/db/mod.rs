use rocket_contrib::databases;
use rocket_contrib::json::Json;


pub mod models;
use models::topic::note::{Note, NewNote, NoteIds, Resource, NewResource};
use models::topic::{Topic, NewTopic, TopicIds};

#[database("tigum_db")]
pub struct TigumPgConn(databases::postgres::Connection);

////////////////////////////
//// RESOURCE DB QUERYS ////
////////////////////////////

pub fn get_resource(conn: &TigumPgConn, resource_id: i32) -> Json<Resource> {
    let query_result = conn.query("SELECT * FROM resources WHERE id = $1", &[&resource_id]).unwrap();
    let resource = query_result.get(0);
    let resource_response = Resource::new(resource.get(0), resource.get(4), resource.get(1), resource.get(2), resource.get(3));
    Json(resource_response)
}

pub fn create_resource(conn: &TigumPgConn, resource: Json<NewResource>) -> String {
    let update = conn.execute("INSERT INTO resources (content_type, content, generated_by) VALUES ($1, $2, $3)", &[&resource.content_type, &resource.content, &resource.generated_by]).unwrap();
    format!("{} rows affected", update)
}

////////////////////////
//// NOTE DB QUERYS ////
////////////////////////


pub fn delete_note(conn: &TigumPgConn, note_id: i32) -> String {
    let result = conn.execute("DELETE FROM notes WHERE id = $1", &[&note_id]).unwrap();
    format!("{} rows deleted", result)
}

pub fn update_note(conn: &TigumPgConn, note_id: i32, note: Json<Note>) -> Json<Note> {
    conn.execute("UPDATE notes SET title = ($2), note_content = ($3) WHERE id = ($1)", &[&note_id, &note.title, &note.note_content]).unwrap();
    get_note(&conn, note_id)
}

pub fn get_notes(conn: &TigumPgConn, note_ids: Json<NoteIds>) -> Json<Vec<Note>> {
    let query_result = conn.query("SELECT * FROM notes WHERE id = ANY($1)", &[&note_ids.ids]).unwrap();
    let mut results: Vec<Note> = vec![];
    for row in query_result.iter() {
        let note = Note::new(row.get(1), row.get(0), row.get(2));
        results.push(note);
    }
    Json(results)
}

pub fn get_note(conn: &TigumPgConn, note_id: i32) -> Json<Note> {
    let query_result = conn.query("SELECT * FROM notes WHERE id = $1", &[&note_id]).unwrap();
    let note = query_result.get(0);
    let note_response = Note::new(note.get(1), note.get(0), note.get(2));
    Json(note_response)
}

pub fn create_note(conn: &TigumPgConn, note: Json<NewNote>) -> String {
    let update = conn.execute("INSERT INTO notes (title, note_content) VALUES ($1, $2)", &[&note.title, &note.note_content]).unwrap();
    format!("Rows affected {}", update)
}


/////////////////////////
//// TOPIC DB QUERYS ////
/////////////////////////

pub fn delete_topic(conn: &TigumPgConn, topic_id: i32) -> String {
    let result = conn.execute("DELETE FROM topics WHERE id = $1", &[&topic_id]).unwrap();
    format!("{} rows deleted", result)
}


pub fn update_topic(conn: &TigumPgConn, topic_id: i32, topic: Json<Topic>) -> Json<Topic> {
    conn.execute("UPDATE topics SET title = ($2), date_created = ($3) WHERE id = ($1)", &[&topic_id, &topic.title, &topic.topic_content]).unwrap();
    get_topic(&conn, topic_id)
}

pub fn get_topics(conn: &TigumPgConn, topic_ids: Json<TopicIds>) -> Json<Vec<Topic>> {
    let query_result = conn.query("SELECT * FROM topics WHERE id = ANY($1)", &[&topic_ids.ids]).unwrap();
    let mut results: Vec<Topic> = vec![];
    for row in query_result.iter() {
        let topic = Topic::new(row.get(1), row.get(0), row.get(2), row.get(3));
        results.push(topic);
    }
    Json(results)
}

pub fn get_topic(conn: &TigumPgConn, topic_id: i32) -> Json<Topic> {
    let query_result = conn.query("SELECT * FROM topics WHERE id = $1", &[&topic_id]).unwrap();
    let topic = query_result.get(0);
    let result = Topic::new(topic.get(1), topic.get(0), topic.get(2), topic.get(3));
    Json(result)
}

pub fn create_topic(conn: &TigumPgConn, topic: Json<NewTopic>) -> String {
    let update = conn.execute("INSERT INTO topics (title, topic_content) VALUES ($1, $2)", &[&topic.title, &topic.topic_content]).unwrap();
    format!("{} rows affected", update)
}