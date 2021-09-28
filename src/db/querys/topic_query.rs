use crate::db::models;
use crate::db::querys::TigumPgConn;
use diesel::{QueryDsl, insert_into};
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
    conn: &diesel::PgConnection,
    topic_id: i32,
    resource_id: i32,
    resource_type: ResourceType,
) -> Result<Topic, Error> {
    use crate::schema::topics::dsl::*;

    let topic_filter = topics.filter(id.eq(topic_id));
    let current_topic = topic_filter.get_result::<Topic>(conn)?;

    match resource_type {
        ResourceType::Snippet => {
            let mut new_list = current_topic.article_snippets.clone();
            new_list.push(resource_id);
            diesel::update(topic_filter).set(article_snippets.eq(new_list)).get_result::<Topic>(conn)
        },
        ResourceType::Link => {
            let mut new_list = current_topic.links.clone();
            new_list.push(resource_id);
            diesel::update(topic_filter).set(links.eq(new_list)).get_result::<Topic>(conn)
        },
        ResourceType::Image => {
            let mut new_list = current_topic.images.clone();
            new_list.push(resource_id);
            diesel::update(topic_filter).set(images.eq(new_list)).get_result::<Topic>(conn)
        },
        ResourceType::Note => {
            let mut new_list = current_topic.notes.clone();
            new_list.push(resource_id);
            diesel::update(topic_filter).set(notes.eq(new_list)).get_result::<Topic>(conn)
        },
        ResourceType::Video => {
            let mut new_list = current_topic.videos.clone();
            new_list.push(resource_id);
            diesel::update(topic_filter).set(videos.eq(new_list)).get_result::<Topic>(conn)
        },
        ResourceType::Code => {
            let mut new_list = current_topic.code.clone();
            new_list.push(resource_id);
            diesel::update(topic_filter).set(code.eq(new_list)).get_result::<Topic>(conn)
        }
    }
}

pub fn remove_from_topic_resource_list(
    conn: &diesel::PgConnection,
    topic_id: i32,
    resource_id: i32,
    resource_type: ResourceType 
) -> Result<Topic, diesel::result::Error> {
    use crate::schema::topics::dsl::*;

    let topic_filter = topics.filter(id.eq(topic_id));
    let current_topic = topic_filter.get_result::<Topic>(conn)?;    

    match resource_type {
        ResourceType::Snippet => {
            let mut list_without_resource = current_topic.article_snippets.clone();
            list_without_resource.retain(|&x| x != resource_id);
            diesel::update(topic_filter).set(article_snippets.eq(list_without_resource)).get_result::<Topic>(conn)
        },
        ResourceType::Link => {
            let mut list_without_resource = current_topic.links.clone();
            list_without_resource.retain(|&x| x != resource_id);
            diesel::update(topic_filter).set(links.eq(list_without_resource)).get_result::<Topic>(conn)
        },
        ResourceType::Image => {
            let mut list_without_resource = current_topic.images.clone();
            list_without_resource.retain(|&x| x != resource_id);
            diesel::update(topic_filter).set(images.eq(list_without_resource)).get_result::<Topic>(conn)
        },
        ResourceType::Note => {
            let mut list_without_resource = current_topic.notes.clone();
            list_without_resource.retain(|&x| x != resource_id);
            diesel::update(topic_filter).set(notes.eq(list_without_resource)).get_result::<Topic>(conn)
        },
        ResourceType::Video => {
            let mut list_without_resource = current_topic.videos.clone();
            list_without_resource.retain(|&x| x != resource_id);
            diesel::update(topic_filter).set(videos.eq(list_without_resource)).get_result::<Topic>(conn)
        },
        ResourceType::Code => {
            let mut list_without_resource = current_topic.code.clone();
            list_without_resource.retain(|&x| x != resource_id);
            diesel::update(topic_filter).set(code.eq(list_without_resource)).get_result::<Topic>(conn)
        }
    }
}

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

pub fn update_topic_mod_date(conn: &diesel::PgConnection, topic_id: i32) -> Result<Topic, Error> {
    use crate::schema::topics::dsl::*;
    use crate::db::querys::topic_query::diesel::dsl::now;
    diesel::update(topics.filter(id.eq(topic_id))).set(
        date_updated.eq(now)
    ).get_result::<Topic>(conn)
}