use crate::db::models;
use rocket_contrib::databases;
use rocket_contrib::json::Json;

pub mod note_q;
pub mod topic_q;
pub mod video_q;

use models::{Id, Ids};

#[database("tigum_db")]
pub struct TigumPgConn(databases::postgres::Connection);
