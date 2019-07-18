#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::{ContentType, Status};
use rocket::response::Response;
use rocket_contrib::json::Json;
use std::collections::HashMap;
use std::string::String;

mod models;

use models::topic::Topic;

#[get("/topics")]
fn index() -> Json<Topic> {
    let topic: Topic = Topic {
        title: String::from("Carl Jung on dreams"),
        date_created: String::from("12th March 2019"),
    };

    return Json(topic);
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
