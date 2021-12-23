use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;

use crate::common::*;
pub use controller::*;
pub use service::*;

mod controller;
mod service;

impl Employee {
    pub const COLLECTION_NAME: &'static str = "employee";
}

// enum gender{
//     Male,
//     Female,
// }
// create the mongoDB schema
#[derive(Deserialize, Serialize, Debug)]
pub struct Employee {
    #[serde(serialize_with = "serialize_object_id", rename = "_id")]
    id: Option<ObjectId>,
    name:String,
    uid:i32,
    salary: f32,
    // match gender {
    //     gender::Male=gender:
    // }
    gender: String,
 }

#[derive(Deserialize, Serialize, Debug)]
pub struct ResourceQuery {
    #[serde(default)]
    keyword: String,
}