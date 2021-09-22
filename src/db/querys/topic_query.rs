use crate::db::models;
use crate::db::querys::TigumPgConn;
use diesel::{QueryDsl, insert_into, sql_query};
use diesel::ExpressionMethods;
use rocket_contrib::json::Json;
use rocket::http::{Status};

// DB Models
use models::topic::{NewTopic, TopicIds};
use models::resources::ResourceType;
// use crate::db::parsing_util::{row_to_topic, parse_topic_result};
use rocket_contrib::databases::diesel;

// DB Schema
use crate::diesel::RunQueryDsl;
use crate::diesel::result::Error;
use crate::db::models::topic::Topic;

// Api Response Struct
use crate::db::api_response::ApiResponse;

pub fn delete_topic(conn: TigumPgConn, topic_id: i32) -> ApiResponse {
    use crate::schema::topics::dsl::*;
    let query_result = diesel::delete(topics.filter(id.eq(topic_id))).get_result::<Topic>(&*conn);
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
    conn: TigumPgConn,
    topic_id: i32,
    resource_id: i32,
    resource_type: ResourceType,
) -> Result<Topic, Error> {
    use crate::schema::topics::dsl::*;
    match resource_type {
        ResourceType::Snippet => {
            diesel::update(topics.filter(id.eq(topic_id))).set(article_snippets.eq(vec![resource_id])).get_result::<Topic>(&*conn)
        },
        ResourceType::Link => {
            diesel::update(topics.filter(id.eq(topic_id))).set(links.eq(vec![resource_id])).get_result::<Topic>(&*conn)
        },
        ResourceType::Image => {
            diesel::update(topics.filter(id.eq(topic_id))).set(images.eq(vec![resource_id])).get_result::<Topic>(&*conn)
        },
        ResourceType::Note => {
            diesel::update(topics.filter(id.eq(topic_id))).set(notes.eq(vec![resource_id])).get_result::<Topic>(&*conn)
        },
        ResourceType::Video => {
            diesel::update(topics.filter(id.eq(topic_id))).set(videos.eq(vec![resource_id])).get_result::<Topic>(&*conn)
        },
        ResourceType::Code => {
            diesel::update(topics.filter(id.eq(topic_id))).set(code.eq(vec![resource_id])).get_result::<Topic>(&*conn)
        }
    }
}

// pub fn remove_from_topic_resource_list(
//     conn: TigumPgConn,
//     topic_id: i32,
//     resource_id: i32,
//     resource_type: ResourceType 
// ) -> Result<u64, Error> {
//     let query_result = match resource_type {
//         ResourceType::Snippet => {
//             let query = format!("UPDATE topics SET article_snippets = array_remove(article_snippets, {}) WHERE id = {}", resource_id, topic_id);
//             sql_query(query).load(&*conn)
//         },
//         ResourceType::Link => {
//             let query = format!("UPDATE topics SET article_snippets = array_remove(article_snippets, {}) WHERE id = {}", resource_id, topic_id);
//             sql_query(query).load(&*conn);
//         },
//         ResourceType::Image => {
//             conn.run(move |c|
//                 c.execute("UPDATE topics SET images = array_remove(images, $1) WHERE id = ($2)", &[&resource_id, &topic_id])
//             )
//         },
//         ResourceType::Note => {
//             conn.run(move |c|
//                 c.execute("UPDATE topics SET notes = array_remove(notes, $1) WHERE id = ($2)", &[&resource_id, &topic_id])
//             )
//         },
//         ResourceType::Video => {
//             conn.run(move |c|
//                 c.execute("UPDATE topics SET videos = array_remove(videos, $1) WHERE id = ($2)", &[&resource_id, &topic_id])
//             )
//         },
//         ResourceType::Code => {
//             conn.run(move |c|
//                 c.execute("UPDATE topics SET code = array_remove(code, $1) WHERE id = ($2)", &[&resource_id, &topic_id])
//             )
//         }
//     };
//     query_result
// }

pub fn update_topic_title(conn: TigumPgConn, topic_id: i32, updated_topic_title: String) -> ApiResponse {
    use crate::schema::topics::dsl::*;
    let updated_topic = diesel::update(topics.filter(id.eq(topic_id))).set(title.eq(updated_topic_title)).get_result::<Topic>(&*conn);
    match updated_topic {
        Ok(row) => {
            ApiResponse {
                json: json!(row),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": format!("Could not update topic with id {}", topic_id) }),
                status: Status::raw(500)
            }
        }
    }
}

pub fn get_topics(conn: TigumPgConn, topic_ids: Json<TopicIds>, uid: i32) -> ApiResponse {
    use crate::schema::topics::dsl::*;
    let req = topics.filter(user_id.eq(uid)).get_results::<Topic>(&*conn);
    match req {
        Ok(result) => {
            ApiResponse {
                json: json!(result),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": format!("Could not get topics for user") }),
                status: Status::raw(500)
            }
        }
    }
}

pub fn get_topic(conn: TigumPgConn, topic_id: i32, uid: i32) -> ApiResponse {
    use crate::schema::topics::dsl::*;
    let req = topics.filter(id.eq(topic_id)).filter(user_id.eq(uid)).first::<Topic>(&*conn);
    match req {
        Ok(result) => {
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

pub fn create_topic(conn: TigumPgConn, new_topic: Json<NewTopic>, uid: i32) -> ApiResponse {
    let topic_title = &new_topic.title;
    use crate::schema::topics::dsl::*;
    let res = insert_into(topics).values((title.eq(topic_title), user_id.eq(uid))).get_results::<Topic>(&*conn);
    match res {
        Ok(result) => {
            ApiResponse {
                json: json!(result),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": format!("Could not create topic with name {}", new_topic.title) }),
                status: Status::raw(500)
            }
        }
    }
}

pub fn update_topic_mod_date(conn: TigumPgConn, topic_id: i32) -> Result<Topic, Error> {
    use crate::schema::topics::dsl::*;
    use crate::db::querys::topic_query::diesel::dsl::now;
    diesel::update(topics.filter(id.eq(topic_id))).set(
        date_updated.eq(now)
    ).get_result::<Topic>(&*conn)
}