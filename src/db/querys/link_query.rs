// Use Macros
use rocket_contrib::json::Json;
use rocket::http::Status;
use diesel::result::Error;
use crate::diesel::ExpressionMethods;
use crate::diesel::Connection;
use crate::diesel::RunQueryDsl;
use crate::diesel::QueryDsl;
use diesel::dsl::any;

// Schema
use crate::db::models::resources::ResourceType;
use crate::db::querys::topic_query::remove_from_topic_resource_list;
use crate::db::querys::topic_query::{add_to_topic_resource_list, update_topic_mod_date};
use crate::db::api_response::ApiResponse;
use crate::db::models::resources::link::{Link, NewLink};
use crate::db::models::Ids;


pub fn delete_link(conn: &diesel::PgConnection, link_id: i32, uid: i32) -> ApiResponse {
    use crate::schema::links::dsl::*;
    let link_to_delete_query = links.filter(id.eq(link_id)).filter(user_id.eq(uid));
    // TODO use transactions when 2 queries should both happend for success
    let transaction_result = conn.transaction::<_, Error, _>(|| {
        let res = diesel::delete(link_to_delete_query).get_result::<Link>(conn)?;
        remove_from_topic_resource_list(conn, res.topic_id, res.id, ResourceType::Link)?;
        Ok(())
    });
    if transaction_result.is_ok() {
        ApiResponse {
            json: json!({ "msg": format!("Successfully deleted note with id {}", link_id) }),
            status: Status::raw(200)
        }
    } else {
        ApiResponse {
            json: json!({ "error": format!("Failed to delete note with id: {}", link_id) }),
            status: Status::raw(500)
        } 
    }
}

pub fn update_link(conn: &diesel::PgConnection, link_id: i32, link: Json<NewLink>, uid: i32) -> ApiResponse {
    use crate::schema::links::dsl::*;
    let link_to_update = links.filter(id.eq(link_id)).filter(user_id.eq(uid));
    let values = (topic_id.eq(link.topic_id), user_id.eq(uid), title.eq(link.title.clone()));
    let res = diesel::update(link_to_update).set(values).get_result::<Link>(conn);
    match res {
        Ok(row) => {
            update_topic_mod_date(conn, row.topic_id);
            ApiResponse {
                json: json!(row),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": format!("Failed to update link with id: {}", link_id) }),
                status: Status::raw(500)
            }
        }
    }

}

pub fn get_links(conn: &diesel::PgConnection, link_ids: Json<Ids>, uid: i32) -> ApiResponse {
    use crate::schema::links::dsl::*;
    let ids = link_ids.ids.clone();
    let query_result = links.filter(id.eq(any(ids))).filter(user_id.eq(uid)).order(date_created.desc()).get_results::<Link>(conn);
    match query_result {
        Ok(rows) => {
            ApiResponse {
                json: json!(rows),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": format!("Could not fetch links") }),
                status: Status::raw(500)
            }
        }
    }
}

pub fn get_link(conn: &diesel::PgConnection, link_id: i32, uid: i32) -> ApiResponse {
    use crate::schema::links::dsl::*;
    let query_result = links.filter(id.eq(link_id)).filter(user_id.eq(uid)).first::<Link>(conn);
    match query_result {
        Ok(row) => {
            ApiResponse {
                json: json!(row),
                status: Status::raw(200)
            }
        },
        Err(err) => {
            ApiResponse {
                json: json!({ "error": "Could not create video" }),
                status: Status::raw(500)
            }
        }
    }
}

pub fn create_link(conn: &diesel::PgConnection, link: Json<NewLink>, uid: i32) -> ApiResponse {
    use crate::schema::links::dsl::*;
    let link_title = link.title.clone();
    let transaction_result = conn.transaction::<Link, Error, _>(|| {
        let values = (
            title.eq(link_title), 
            topic_id.eq(link.topic_id), 
            user_id.eq(uid),
            source.eq(link.source.clone()),
            favicon_source.eq(link.favicon_source.clone())
        );
        let new_link = diesel::insert_into(links).values(values).get_result::<Link>(conn)?;
        add_to_topic_resource_list(
            conn, 
            link.topic_id, 
            new_link.id, 
            ResourceType::Link
        )?;
        Ok(new_link)
    });
    match transaction_result {
        Ok(updated_row) => {
            update_topic_mod_date(conn, updated_row.topic_id);
            ApiResponse {
                json: json!(updated_row),
                status: Status::raw(200)
            }
       },
        Err(err) => {
            ApiResponse {
                json: json!({"error": format!("Could not create link" )}),
                status: Status::raw(500)
            }
        }
    }
}
