use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use validator::Validate;


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[validate(email)]
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub admin: bool,
}