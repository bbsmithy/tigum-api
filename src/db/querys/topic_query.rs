use crate::db::models;
use crate::db::querys::TigumPgConn;
use rocket_contrib::json::Json;
use rocket::http::{Status};

// DB Models
use models::resources::ResourceType;
use models::topic::{NewTopic, Topic, TopicIds};
// use crate::db::parsing_util::{row_to_topic, parse_topic_result};

// Api Response Struct
use crate::db::api_response::ApiResponse;

pub fn delete_topic(conn: &TigumPgConn, topic_id: i32) -> ApiResponse {
    ApiResponse {
        json: json!("All good"),
        status: Status::raw(200)
    }
    // let query_result = conn.run(move |c|
    //     c.execute("DELETE FROM topics WHERE id = $1", &[&topic_id])
    // );
    // match query_result {
    //     Ok(_result) => ApiResponse { 
    //         json: json!({ "msg": format!("Topic with id {} deleted", topic_id)}),
    //         status: Status::raw(200)
    //     },
    //     Err(_error) => ApiResponse { 
    //         json: json!({ "error": format!("Could not delete topic with id {}", topic_id) }),
    //         status: Status::raw(500)
    //     }
    // }
}

// RESEARCH FOREIGN KEY CONSTRAINTS FOR UPDATING OTHER TABLES WHEN TOPIC ROW IS DELETED
// fn delete_topic_resources(conn: &TigumPgConn, topic_id: i32) -> ApiResponse {
//     DELETE messages, usersmessages  FROM messages  INNER JOIN usersmessages  
//     WHERE messages.messageid= usersmessages.messageid and messages.messageid = '1'
// }

// pub fn add_to_topic_resource_list(
//     conn: &TigumPgConn,
//     topic_id: i32,
//     resource_id: i32,
//     resource_type: ResourceType,
// ) -> Result<u64, Error> {
//     let query_result = match resource_type {
//         ResourceType::Snippet => {
//             conn.run(move |c|
//                 c.execute("UPDATE topics SET article_snippets = array_append(article_snippets, $1) WHERE id = ($2)", &[&resource_id, &topic_id])
//             )
//         },
//         ResourceType::Link => {
//             conn.run(move |c|
//                 c.execute("UPDATE topics SET links = array_append(links, $1) WHERE id = ($2)", &[&resource_id, &topic_id])
//             )
//         },
//         ResourceType::Image => {
//             conn.run(move |c|
//                 c.execute("UPDATE topics SET images = array_append(images, $1) WHERE id = ($2)", &[&resource_id, &topic_id])
//             )
//         },
//         ResourceType::Note => {
//             conn.run(move |c|
//                 c.execute("UPDATE topics SET notes = array_append(notes, $1) WHERE id = ($2)", &[&resource_id, &topic_id])
//             )
//         },
//         ResourceType::Video => {
//             conn.run(move |c|
//                 c.execute("UPDATE topics SET videos = array_append(videos, $1) WHERE id = ($2)", &[&resource_id, &topic_id])
//             )
//         },
//         ResourceType::Code => {
//             conn.run(move |c|
//                 c.execute("UPDATE topics SET code = array_append(code, $1) WHERE id = ($2)", &[&resource_id, &topic_id])
//             )
//         }
//     };
//     query_result
// }

// pub fn remove_from_topic_resource_list(
//     conn: &TigumPgConn,
//     topic_id: i32,
//     resource_id: i32,
//     resource_type: ResourceType 
// ) -> Result<u64, Error> {
//     let query_result = match resource_type {
//         ResourceType::Snippet => {
//             conn.run(move |c|
//                 c.execute(
//                     "UPDATE topics SET article_snippets = array_remove(article_snippets, $1) WHERE id = ($2)",
//                     &[&resource_id, &topic_id]
//                 )
//             )
//         },
//         ResourceType::Link => {
//             conn.run(move |c|
//                 c.execute(
//                     "UPDATE topics SET links = array_remove(links, $1) WHERE id = ($2)",
//                     &[&resource_id, &topic_id]
//                 )
//             )
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

pub fn update_topic(conn: &TigumPgConn, topic_id: i32, topic: Json<Topic>) -> ApiResponse {
    ApiResponse {
        json: json!("All good"),
        status: Status::raw(200)
    }
    // let query_result = conn.run(move |c|
    //     c.query(
    //         "UPDATE topics SET title = ($2), notes = ($3), videos = ($4) WHERE id = ($1) RETURNING *",
    //         &[&topic_id, &topic.title, &topic.notes, &topic.videos],
    //     )
    // );
    // match query_result {
    //     Ok(rows) => {
    //         if let Some(row) = rows.get(0) {
    //             let updated_topic = row_to_topic(row);
    //             ApiResponse {
    //                 json: json!(updated_topic),
    //                 status: Status::raw(200)
    //             }     
    //         } else {
    //             ApiResponse {
    //                 json: json!({"error": format!("Could not update topic")}),
    //                 status: Status::raw(500)
    //             } 
    //         }
    //     },
    //     Err(_err) => {
    //         ApiResponse {
    //             json: json!({"error": format!("Could not update topic with id {}", topic_id)}),
    //             status: Status::raw(500)
    //         }
    //     }
    // }
}

pub fn get_topics(conn: &TigumPgConn, topic_ids: Json<TopicIds>, user_id: i32) -> ApiResponse {
    ApiResponse {
        json: json!("All good"),
        status: Status::raw(200)
    }
    // if topic_ids.ids.len() == 0 {
    //     let query_result = conn.run(move |c|
    //         c.query("SELECT * FROM topics WHERE user_id = $1 ORDER BY date_updated DESC", &[&user_id])
    //     );
    //     match query_result {
    //         Ok(rows) => {
    //             let topics = parse_topic_result(rows);
    //             ApiResponse {
    //                 json: json!(topics),
    //                 status: Status::raw(200)
    //             }
    //         },
    //         Err(_err) => {
    //             ApiResponse {
    //                 json: json!({ "error":  format!("Could not get topics with ids {:?}", topic_ids)}),
    //                 status: Status::raw(200)
    //             }
    //         }
    //     }
    // } else {
    //     let query_result = conn.run(move |c|
    //         c.query("SELECT * FROM topics WHERE id = ANY($1) AND user_id = $2 ORDER BY date_created DESC", &[&topic_ids.ids, &user_id])
    //     );
    //     match query_result {
    //         Ok(rows) => {
    //             let topics = parse_topic_result(rows);
    //             ApiResponse {
    //                 json: json!(topics),
    //                 status: Status::raw(200)
    //             }
    //         },
    //         Err(_err) => {
    //             ApiResponse {
    //                 json: json!({ "error":  format!("Could not get topics")}),
    //                 status: Status::raw(200)
    //             }
    //         }
    //     }
    // }
}

pub fn get_topic(conn: &TigumPgConn, topic_id: i32, user_id: i32) -> ApiResponse {
    ApiResponse {
        json: json!("All good"),
        status: Status::raw(200)
    }
    // let query_result = conn.run(move |c|
    //     c.query("SELECT * FROM topics WHERE id = $1 AND user_id = $2", &[&topic_id, &user_id])
    // );
    // match query_result {
    //     Ok(rows) => {
    //         if let Some(row) = rows.get(0) {
    //             let result = row_to_topic(row);
    //             ApiResponse {
    //                 json: json!(result),
    //                 status: Status::raw(200)
    //             }
    //         } else {
    //             ApiResponse {
    //                 json: json!({ "error": format!("Could not get topic with id {}", topic_id) }),
    //                 status: Status::raw(500)
    //             }
    //         }
    //     },
    //     Err(_err) => {
    //         ApiResponse {
    //             json: json!({ "error": format!("Could not get topic with id {}", topic_id) }),
    //             status: Status::raw(500)
    //         }
    //     }
    // }
}

pub fn create_topic(conn: &TigumPgConn, topic: Json<NewTopic>, user_id: i32) -> ApiResponse {
    ApiResponse {
        json: json!("All good"),
        status: Status::raw(200)
    }
    // let query_result = conn.run(move |c|
    //     c.query(
    //         "INSERT INTO topics (title, user_id) VALUES ($1, $2) RETURNING *",
    //         &[&topic.title, &user_id],
    //     )
    // );
    // match query_result {
    //     Ok(rows) => {
    //         if let Some(row) = rows.get(0) {
    //             let new_topic = row_to_topic(row);
    //             ApiResponse {
    //                 json: json!(new_topic),
    //                 status: Status::raw(200)
    //             }   
    //         } else {
    //             ApiResponse {
    //                 json: json!({ "error": format!("Could not create topic") }),
    //                 status: Status::raw(500)
    //             }
    //         }
    //     },
    //     Err(_err) => {
    //         ApiResponse {
    //             json: json!({ "error": format!("Could not create topic") }),
    //             status: Status::raw(500)
    //         }
    //     }
    // }
}

// pub fn update_topic_mod_date(conn: &TigumPgConn, topic_id: i32) -> Result<Vec<Row>, Error> {
//     conn.run(move |c|
//         c.query(
//             "UPDATE topics SET date_updated = CURRENT_TIMESTAMP WHERE id = $1",
//             &[&topic_id],
//         )
//     )
// }