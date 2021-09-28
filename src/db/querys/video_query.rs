//Use Macros
use rocket_contrib::json::Json;
use rocket::http::Status;

use crate::db::models;
use crate::db::querys::TigumPgConn;
use crate::db::api_response::ApiResponse;
use crate::db::querys::topic_query::{
    add_to_topic_resource_list,
    remove_from_topic_resource_list, 
    update_topic_mod_date
};
use crate::routes::video_routes::videos;
use models::resources::video::{NewVideo, Video};
use models::resources::ResourceType;
use models::Ids;

// DB Schema
use diesel::{QueryDsl, RunQueryDsl};
use diesel::result::Error;
use diesel::Connection;
use diesel::PgConnection;
use diesel::ExpressionMethods;
use diesel::dsl::any;



pub fn delete_video(conn: &PgConnection, video_id: i32, uid: i32) -> ApiResponse {
    use crate::schema::videos::dsl::*;
    let video_to_delete = videos.filter(id.eq(video_id)).filter(user_id.eq(uid));
    // TODO use transactions when 2 queries should both happend for success
    let transaction_result = conn.transaction::<_, Error, _>(|| {
        let res = diesel::delete(video_to_delete).get_result::<Video>(conn)?;
        remove_from_topic_resource_list(conn, res.topic_id, res.id, ResourceType::Note)?;
        Ok(())
    });
    if transaction_result.is_ok() {
        ApiResponse {
            json: json!({ "msg": format!("Successfully deleted note with id {}", video_id) }),
            status: Status::raw(200)
        }
    } else {
        ApiResponse {
            json: json!({ "error": format!("Failed to delete note with id: {}", video_id) }),
            status: Status::raw(500)
        } 
    }
}

pub fn update_video(conn: &PgConnection, video_id: i32, video: Json<NewVideo>, uid: i32) -> ApiResponse {
    use crate::schema::videos::dsl::*;
    let video_to_update = videos.filter(id.eq(video_id))
    .filter(user_id.eq(uid));
    let values = (
        topic_id.eq(video.topic_id),
        title.eq(video.title.clone()),
        iframe.eq(video.iframe.clone()),
        origin.eq(video.origin.clone()),
        thumbnail_img.eq(video.thumbnail_img.clone())
    );
    let query_result = diesel::update(video_to_update).set(values).get_results::<Video>(conn);
    match query_result {
        Ok(updated_rows) => {
            ApiResponse {
                json: json!(updated_rows),
                status: Status::raw(200)
            }
       },
        Err(_err) => {
            ApiResponse {
                json: json!({"error": format!("Could not update video with id {}", video_id)}),
                status: Status::raw(500)
            }
        }
    }
}

pub fn get_videos(conn: &PgConnection, video_ids: Json<Ids>, uid: i32) -> ApiResponse {
    use crate::schema::videos::dsl::*;
    let ids = video_ids.ids.clone();
    let query_result = videos.filter(id.eq(any(ids)))
    .filter(user_id.eq(uid))
    .get_results::<Video>(conn);
    match query_result {
        Ok(rows) => {
            ApiResponse {
                json: json!(rows),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": format!("Could not get videos") }),
                status: Status::raw(500)
            }
        }
    }
}

pub fn get_video(conn: &PgConnection, video_id: i32, _user_id: i32) -> ApiResponse {
    use crate::schema::videos::dsl::*;
    let query_result = videos.filter(id.eq(video_id)).first::<Video>(conn);
    match query_result {
        Ok(row) => {
            ApiResponse {
                json: json!(row),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": "Could not create video" }),
                status: Status::raw(500)
            }
        }
    }
}

pub fn create_video(conn: &diesel::PgConnection, video: Json<NewVideo>, uid: i32) -> ApiResponse {

    use crate::schema::videos::dsl::*;

    let video_title = video.title.clone();
    let video_iframe = video.iframe.clone();
    let video_origin = video.origin.clone();
    let video_thumbnail = video.thumbnail_img.clone();

    // TODO use transactions when 2 queries should both happend for success
    let transaction_result = conn.transaction::<Video, Error, _>(|| {
        let new_video = diesel::insert_into(videos).values((
            title.eq(video_title), 
            topic_id.eq(video.topic_id), 
            user_id.eq(uid),
            iframe.eq(video_iframe),
            origin.eq(video_origin),
            thumbnail_img.eq(video_thumbnail)
        )).get_result::<Video>(conn)?;
        add_to_topic_resource_list(
            conn, 
            new_video.topic_id, 
            new_video.id, 
            ResourceType::Video
        )?;
        Ok(new_video)
    });

    match transaction_result {
        Ok(new_video) => {
            update_topic_mod_date(conn,  new_video.topic_id);
            ApiResponse {
                json: json!(new_video),
                status: Status::raw(200)
            }
        },
        Err(_err) => {
            ApiResponse {
                json: json!({ "error": format!("Could not create note") }),
                status: Status::raw(500)
            }
        }
    }
}


fn update_topic_from_video(conn: &PgConnection, video: Video) -> ApiResponse {
    ApiResponse {
        json: json!("All good"),
        status: Status::raw(200)
    }
    // match update_topic_mod_date(conn, video.topic_id) {
    //     Ok(_rows) => {
    //         ApiResponse {
    //             json: json!(video),
    //             status: Status::raw(200)
    //         }
    //     },
    //     Err(err) => {
    //         println!("{:?}", err);
    //         ApiResponse {
    //             json: json!({"error": format!("Could not update note")}),
    //             status: Status::raw(500)
    //         }
    //     }
    // }
}
