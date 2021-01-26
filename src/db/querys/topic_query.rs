use crate::db::models;
use crate::db::querys::TigumPgConn;
use rocket_contrib::json::Json;
use rocket_contrib::databases::postgres::row::Row;
use rocket_contrib::databases::postgres::Error;
use rocket::http::{Status};


// DB Models
use models::resources::ResourceType;
use models::topic::{NewTopic, Topic, TopicIds};

// Api Response Struct
use crate::db::api_response::ApiResponse;

type QueryResult = std::result::Result<rocket_contrib::databases::postgres::row::Row, rocket_contrib::databases::postgres::Error>;

fn parse_topic_result(query_result: rocket_contrib::databases::postgres::row::Row) -> Vec<Topic> {
    let mut results: Vec<Topic> = vec![];
    for row in query_result {
        let topic = row_to_topic(row);
        results.push(topic);
    }
    results
}

fn row_to_topic(row: Row) -> Topic {
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
    );
    return topic;
}

pub async fn delete_topic(conn: &TigumPgConn, topic_id: i32) -> ApiResponse {
    let query_result = conn.run(move |c|
        c.execute("DELETE FROM topics WHERE id = $1", &[&topic_id])
    ).await;
    match query_result {
        Ok(_result) => ApiResponse { 
            json: json!({ "msg": format!("Topic with id {} deleted", topic_id)}),
            status: Status::raw(200)
        },
        Err(_error) => ApiResponse { 
            json: json!({ "error": format!("Could not delete topic with id {}", topic_id) }),
            status: Status::raw(500)
        }
    }
}

// RESEARCH FOREIGN KEY CONSTRAINTS FOR UPDATING OTHER TABLES WHEN TOPIC ROW IS DELETED
// fn delete_topic_resources(conn: &TigumPgConn, topic_id: i32) -> ApiResponse {
//     DELETE messages, usersmessages  FROM messages  INNER JOIN usersmessages  
//     WHERE messages.messageid= usersmessages.messageid and messages.messageid = '1'
// }

pub fn add_to_topic_resource_list(
    conn: &TigumPgConn,
    topic_id: i32,
    resource_id: i32,
    resource_type: ResourceType,
) -> Result<u64, Error> {
    let query_result = match resource_type {
        ResourceType::Snippet => conn.execute("UPDATE topics SET article_snippets = array_append(article_snippets, $1) WHERE id = ($2)", &[&resource_id, &topic_id]),
        ResourceType::Link => conn.execute("UPDATE topics SET links = array_append(links, $1) WHERE id = ($2)", &[&resource_id, &topic_id]),
        ResourceType::Image => conn.execute("UPDATE topics SET images = array_append(images, $1) WHERE id = ($2)", &[&resource_id, &topic_id]),
        ResourceType::Note => conn.execute("UPDATE topics SET notes = array_append(notes, $1) WHERE id = ($2)", &[&resource_id, &topic_id]),
        ResourceType::Video => conn.execute("UPDATE topics SET videos = array_append(videos, $1) WHERE id = ($2)", &[&resource_id, &topic_id]),
        ResourceType::Code => conn.execute("UPDATE topics SET code = array_append(code, $1) WHERE id = ($2)", &[&resource_id, &topic_id])
    };
    query_result
}

pub async fn remove_from_topic_resource_list(
    conn: &TigumPgConn,
    topic_id: i32,
    resource_id: i32,
    resource_type: ResourceType 
) -> Result<u64, Error> {
    let query_result = match resource_type {
        ResourceType::Snippet => {
            conn.run(|c|
                c.execute(
                    "UPDATE topics SET article_snippets = array_remove(article_snippets, $1) WHERE id = ($2)",
                    &[&resource_id, &topic_id]
                )
            ).await
        },
        ResourceType::Link => conn.execute(),
        ResourceType::Image => conn.execute("UPDATE topics SET images = array_remove(images, $1) WHERE id = ($2)", &[&resource_id, &topic_id]),
        ResourceType::Note => conn.execute("UPDATE topics SET notes = array_remove(notes, $1) WHERE id = ($2)", &[&resource_id, &topic_id]),
        ResourceType::Video => conn.execute("UPDATE topics SET videos = array_remove(videos, $1) WHERE id = ($2)", &[&resource_id, &topic_id]),
        ResourceType::Code => conn.execute("UPDATE topics SET code = array_remove(code, $1) WHERE id = ($2)", &[&resource_id, &topic_id])
    };
    query_result
}

pub fn update_topic(conn: &TigumPgConn, topic_id: i32, topic: Json<Topic>) -> ApiResponse {
    let query_result = conn.run(move |c|
        c.query(
            "UPDATE topics SET title = ($2), notes = ($3), videos = ($4) WHERE id = ($1) RETURNING *",
            &[&topic_id, &topic.title, &topic.notes, &topic.videos],
        )
    );
    match query_result {
        Ok(rows) => {
            let updated_topic = row_to_topic(rows.get(0));
            ApiResponse {
                json: json!(updated_topic),
                status: Status::raw(200)
            }   
        },
        Err(_err) => {
            ApiResponse {
                json: json!({"error": format!("Could not update topic with id {}", topic_id)}),
                status: Status::raw(500)
            }
        }
    }
}

pub fn get_topics(conn: &TigumPgConn, topic_ids: Json<TopicIds>, user_id: i32) -> ApiResponse {
    if topic_ids.ids.len() == 0 {
        let query_result = conn.query("SELECT * FROM topics WHERE user_id = $1 ORDER BY date_created DESC", &[&user_id]);
        get_topics_response(query_result, &topic_ids.ids)
    } else {
        let query_result = conn.query("SELECT * FROM topics WHERE id = ANY($1) AND user_id = $2 ORDER BY date_created DESC", &[&topic_ids.ids, &user_id]);
        get_topics_response(query_result, &topic_ids.ids)
    }
}

fn get_topics_response(query_result: QueryResult, topic_ids: &Vec<i32>) -> ApiResponse {
    match query_result {
        Ok(rows) => {
            let topics = parse_topic_result(rows);
            ApiResponse {
                json: json!(topics),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error":  format!("Could not get topics with ids {:?}", topic_ids)}),
                status: Status::raw(200)
            }
        }
    }
}

pub fn get_topic(conn: &TigumPgConn, topic_id: i32, user_id: i32) -> ApiResponse {
    let query_result = conn.query("SELECT * FROM topics WHERE id = $1 AND user_id = $2", &[&topic_id, &user_id]);
    match query_result {
        Ok(rows) => {
            let result = row_to_topic(rows.get(0));
            ApiResponse {
                json: json!(result),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": format!("Could not get topic with id {}", topic_id) }),
                status: Status::raw(500)
            }
        }
    }
}

pub fn create_topic(conn: &TigumPgConn, topic: Json<NewTopic>, user_id: i32) -> ApiResponse {
    let query_result = conn.query(
            "INSERT INTO topics (title, user_id) VALUES ($1, $2) RETURNING *",
            &[&topic.title, &user_id],
        );
    match query_result {
        Ok(rows) => {
            let new_topic = row_to_topic(rows.get(0));
            ApiResponse {
                json: json!(new_topic),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": format!("Could not create topic with title {:#?}", topic) }),
                status: Status::raw(500)
            }
        }
    }
}
