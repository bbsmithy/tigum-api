//Use Macros
use rocket_contrib::json::Json;
use rocket::http::Status;
use crate::db::models::resources::ResourceType;
use crate::db::querys::TigumPgConn;
// use crate::db::querys::topic_query::remove_from_topic_resource_list;
// use crate::db::querys::topic_query::{add_to_topic_resource_list, update_topic_mod_date};
use crate::db::api_response::ApiResponse;
use crate::db::models::resources::link::{Link, NewLink};
use crate::db::models::Ids;

// fn row_to_link(row: &rocket_contrib::databases::postgres::Row) -> Link {
//     Link {
//         id: row.get(0),
//         title: row.get(1),
//         user_id: row.get(2),
//         topic_id: row.get(3),
//         date_created: row.get(4),
//         source: row.get(5),
//     }
// }

pub fn delete_link(conn: &TigumPgConn, id: i32, user_id: i32) -> ApiResponse {
    ApiResponse {
        json: json!("All good"),
        status: Status::raw(200)
    }
    // let update = conn.run(move |c|
    //     c.query("DELETE FROM links WHERE id = $1 AND user_id = $2 RETURNING topic_id", &[&id, &user_id])
    // );
    // match update {
    //     Ok(row) => {
    //         let deleted_row = row.get(0);
    //         if let Some(row) = deleted_row {
    //             let deleted_row_topic_id: i32 = row.get(0);
    //             let remove_topic_result = remove_from_topic_resource_list(conn, deleted_row_topic_id, id, ResourceType::Link);
    //             match remove_topic_result {
    //                 Ok(_rows_removed) => {
    //                     ApiResponse {
    //                         json: json!({ "msg": format!("Successfully deleted link with id {}", id) }),
    //                         status: Status::raw(200)
    //                     }
    //                 },
    //                 Err(_err) => {
    //                     ApiResponse {
    //                         json: json!({ "error": format!("Failed to deleted row with id: {}", id) }),
    //                         status: Status::raw(500)
    //                     }
    //                 }
    //             }
    //         } else {
    //             ApiResponse {
    //                 json: json!({ "error": format!("Failed to deleted row with id: {}", id) }),
    //                 status: Status::raw(500)
    //             }
    //         }
    //     },  
    //     Err(_err) => {
    //         ApiResponse {
    //             json: json!({ "error": format!("Failed to deleted row with id: {}", id) }),
    //             status: Status::raw(500)
    //         }
    //     }
    // }
}

pub fn update_link(conn: &TigumPgConn, id: i32, link: Json<NewLink>, user_id: i32) -> ApiResponse {
    ApiResponse {
        json: json!("All good"),
        status: Status::raw(200)
    }
    // let update = conn.run(move |c|
    //     c.query(
    //         "UPDATE links SET topic_id = $2, user_id = $3, title = $4 WHERE id = $1 AND user_id = $5 RETURNING *",
    //         &[&id, &link.topic_id, &user_id, &link.title, &user_id],
    //     )
    // );
    // match update {
    //     Ok(rows) => {
    //         let updated_row = rows.get(0);
    //         if let Some(row) = updated_row {
    //             let updated_link = row_to_link(row);
    //             update_topic_from_link(&conn, updated_link)
    //         } else {
    //             ApiResponse {
    //                 json: json!({ "error": format!("Failed to update link") }),
    //                 status: Status::raw(200)
    //             }
    //         }
    //     },
    //     Err(_err) => {
    //         ApiResponse {
    //             json: json!({ "error": format!("Failed to update link with id: {}", id) }),
    //             status: Status::raw(500)
    //         }
    //     }
    // }
}

pub fn get_links(conn: &TigumPgConn, ids: Json<Ids>, user_id: i32) -> ApiResponse {
    ApiResponse {
        json: json!("All good"),
        status: Status::raw(200)
    }
    // let query_result = conn.run(move |c|
    //     c.query(
    //         "SELECT * FROM links WHERE id = ANY($1) AND user_id = $2 ORDER BY date_created DESC",
    //         &[&ids.ids, &user_id],
    //     )
    // );
    // match query_result {
    //     Ok(rows) => {
    //         let mut results: Vec<Link> = vec![];
    //         for row in rows.iter() {
    //             let link_response = row_to_link(row);
    //             results.push(link_response);
    //         }
    //         ApiResponse {
    //             json: json!(results),
    //             status: Status::raw(200)
    //         }
    //     },
    //     Err(_err) => {
    //         ApiResponse {
    //             json: json!({ "error": format!("Could not fetch links") }),
    //             status: Status::raw(500)
    //         }
    //     }
    // }
}

pub fn get_link(conn: &TigumPgConn, id: i32, user_id: i32) -> ApiResponse {
    ApiResponse {
        json: json!("All good"),
        status: Status::raw(200)
    }
    // let query_result = conn.run(move |c|
    //     c.query(
    //         "SELECT * FROM links WHERE id = $1 AND user_id = $2",
    //         &[&id, &user_id],
    //     )
    // );
    // match query_result {
    //     Ok(rows) => {
    //         if let Some(row) = rows.get(0) {
    //             let link_response = row_to_link(row);
    //             ApiResponse {
    //                 json: json!(link_response),
    //                 status: Status::raw(200)
    //             }
    //         } else {
    //             ApiResponse {
    //                 json: json!({ "error": format!("Could not fetch links") }),
    //                 status: Status::raw(200)
    //             }
    //         }
    //     },
    //     Err(_err) => {
    //         ApiResponse {
    //             json: json!({ "error": format!("Failed to get link with id: {}", id) }),
    //             status: Status::raw(500)
    //         }
    //     }
    // }
}

pub fn create_link(conn: &TigumPgConn, link: Json<NewLink>, user_id: i32) -> ApiResponse {
    ApiResponse {
        json: json!("All good"),
        status: Status::raw(200)
    }
    // let topic_id = link.topic_id;
    // let query_result = conn.run(move |c|
    //     c.query(
    //         "INSERT INTO links (title, topic_id, user_id, source) VALUES ($1, $2, $3, $4) RETURNING *",
    //         &[
    //             &link.title,
    //             &link.topic_id,
    //             &user_id,
    //             &link.source
    //         ],
    //     )
    // );
    // match query_result {
    //     Ok(new_link_rows) => {
    //         let new_row = new_link_rows.get(0);
    //         if let Some(row) = new_row {
    //             let new_link = row_to_link(row);
    //             let query_result = add_to_topic_resource_list(
    //                 &conn,
    //                 topic_id,
    //                 new_link.id,
    //                 ResourceType::Link,
    //             );
    //             match query_result {
    //                 Ok(_rows_updated) => update_topic_from_link(&conn, new_link),
    //                 Err(_error) => ApiResponse {
    //                     json: json!({ "error": format!("Could not create snippet {}", new_link.topic_id )}),
    //                     status: Status::raw(500)
    //                 }
    //             }
    //         } else {
    //             ApiResponse {
    //                 json: json!({ "error": format!("Could not create snippet")}),
    //                 status: Status::raw(500)
    //             }
    //         }
    //     }
    //     Err(_err) => ApiResponse {
    //         json: json!({
    //             "error": format!("Could not create link")
    //         }),
    //         status: Status::raw(500)
    //     }
    // }
}

// fn update_topic_from_link(conn: &TigumPgConn, link: Link) -> ApiResponse {
//     match update_topic_mod_date(conn, link.topic_id) {
//         Ok(_rows) => {
//             ApiResponse {
//                 json: json!(link),
//                 status: Status::raw(200)
//             }
//         },
//         Err(_err) => {
//             ApiResponse {
//                 json: json!({"error": format!("Could not update note")}),
//                 status: Status::raw(500)
//             }
//         }
//     }
// }
