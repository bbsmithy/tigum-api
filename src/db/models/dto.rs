use serde::{Deserialize, Serialize};
use crate::db::models::search::resources::ResourceResult;
use diesel::sql_types::{Integer, BigInt};

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicResources {
    pub notes: Vec<ResourceResult>,
    pub links: Vec<ResourceResult>,
    pub videos: Vec<ResourceResult>,
    pub snippets: Vec<ResourceResult>
}

#[derive(Queryable, QueryableByName, PartialEq, Debug, Serialize, Deserialize)]
pub struct PublicResourcesCount {
    #[sql_type="BigInt"]
    pub public_resources_count: i64
}


impl PublicResources {

    pub fn new(results: Vec<ResourceResult>) -> Self {
        let mut notes: Vec<ResourceResult> = vec!();
        let mut videos: Vec<ResourceResult> = vec!();
        let mut links: Vec<ResourceResult> = vec!();
        let mut snippets: Vec<ResourceResult> = vec!();
        for r in results {
            match r.result_type.as_str() {
                "note" => {
                    notes.push(r)
                },
                "link" => {
                    links.push(r)
                },
                "video" => {
                    videos.push(r)
                }
                "snippet" => {
                    snippets.push(r)
                }
                _ => {
                    println!("Couldn't find macth fro resource: {}", r.result_type.as_str())
                }
            }
        }
        Self {
            notes,
            videos,
            links,
            snippets
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MasonaryView {
    pub lhs: Vec<ResourceResult>,
    pub rhs: Vec<ResourceResult>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfilePictureUrl {
    pub url: String
}