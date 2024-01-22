use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1, max = 300))]
    pub content: String,
    pub date: String,
}