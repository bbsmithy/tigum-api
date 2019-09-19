use rocket_contrib::databases;
use rocket_contrib::json::Json;

pub mod models;
use models::topic::note::{NewNote, NewResource, Note, NoteIds, Resource};
use models::topic::{NewTopic, Topic, TopicIds};
use models::{Id, Ids};

#[database("tigum_db")]
pub struct TigumPgConn(databases::postgres::Connection);

////////////////////////////
//// RESOURCE DB QUERYS ////
////////////////////////////

pub fn delete_resource(conn: &TigumPgConn, resource_id: i32) -> String {
    let update = conn
        .execute("DELETE FROM resources WHERE id = $1", &[&resource_id])
        .unwrap();
    format!("{} rows affected", update)
}

pub fn update_resource(
    conn: &TigumPgConn,
    resource_id: i32,
    resource: Json<Resource>,
) -> Json<Resource> {
    conn.execute(
        "UPDATE resources SET content = $2 WHERE id = $1",
        &[&resource_id, &resource.content],
    )
    .unwrap();
    get_resource(conn, resource_id)
}

pub fn get_resources(conn: &TigumPgConn, resource_ids: Json<Ids>) -> Json<Vec<Resource>> {
    let query_result = conn
        .query(
            "SELECT * FROM resources WHERE id = ANY($1)",
            &[&resource_ids.ids],
        )
        .unwrap();
    let mut results: Vec<Resource> = vec![];
    for row in query_result.iter() {
        let resources = Resource::new(
            row.get(0),
            row.get(4),
            row.get(1),
            row.get(2),
            row.get(3),
            row.get(5),
        );
        results.push(resources);
    }
    Json(results)
}

pub fn get_resource(conn: &TigumPgConn, resource_id: i32) -> Json<Resource> {
    let query_result = conn
        .query("SELECT * FROM resources WHERE id = $1", &[&resource_id])
        .unwrap();
    let resource = query_result.get(0);
    let resource_response = Resource::new(
        resource.get(0),
        resource.get(4),
        resource.get(1),
        resource.get(2),
        resource.get(3),
        resource.get(5),
    );
    Json(resource_response)
}

pub fn create_resource(conn: &TigumPgConn, resource: Json<NewResource>) -> Json<Id> {
    let inserted_row = conn
        .query(
            "INSERT INTO resources (content_type, content, generated_by, title) VALUES ($1, $2, $3, $4) RETURNING id",
            &[
                &resource.content_type,
                &resource.content,
                &resource.generated_by,
                &resource.title
            ],
        )
        .unwrap();
    let row = inserted_row.get(0);
    let resource_id: i32 = row.get(0);

    let id_response = Id { id: resource_id };

    Json(id_response)
}

////////////////////////
//// NOTE DB QUERYS ////
////////////////////////

fn parse_note_result(query_result: rocket_contrib::databases::postgres::rows::Rows) -> Vec<Note> {
    let mut results: Vec<Note> = vec![];
    for row in query_result.iter() {
        let note = Note::new(row.get(1), row.get(0), row.get(2), row.get(3));
        results.push(note);
    }
    results
}

pub fn delete_note(conn: &TigumPgConn, note_id: i32) -> String {
    let result = conn
        .execute("DELETE FROM notes WHERE id = $1", &[&note_id])
        .unwrap();
    format!("{} rows deleted", result)
}

pub fn update_note(conn: &TigumPgConn, note_id: i32, note: Json<Note>) -> Json<Note> {
    conn.execute(
        "UPDATE notes SET title = ($2), note_content = ($3) WHERE id = ($1)",
        &[&note_id, &note.title, &note.note_content],
    )
    .unwrap();
    get_note(&conn, note_id)
}

pub fn get_notes(conn: &TigumPgConn, note_ids: Json<NoteIds>) -> Json<Vec<Note>> {
    let query_result = conn
        .query("SELECT * FROM notes WHERE id = ANY($1)", &[&note_ids.ids])
        .unwrap();
    let results = parse_note_result(query_result);
    Json(results)
}

pub fn get_note(conn: &TigumPgConn, note_id: i32) -> Json<Note> {
    let query_result = conn
        .query("SELECT * FROM notes WHERE id = $1", &[&note_id])
        .unwrap();
    let note = query_result.get(0);
    let note_response = Note::new(note.get(1), note.get(0), note.get(2), note.get(3));
    Json(note_response)
}

pub fn create_note(conn: &TigumPgConn, note: Json<NewNote>) -> Json<Id> {
    let inserted_rows = conn
        .query(
            "INSERT INTO notes (title, note_content) VALUES ($1, $2) RETURNING id",
            &[&note.title, &note.note_content],
        )
        .unwrap();

    let row = inserted_rows.get(0);
    let note_id: i32 = row.get(0);

    let id_response = Id { id: note_id };

    Json(id_response)
}

/////////////////////////
//// TOPIC DB QUERYS ////
/////////////////////////

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
