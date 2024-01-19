use mongodb::bson::oid::ObjectId;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Authentication {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub logged_in: bool,
    pub user_session: String,
    #[serde(default)]
    pub admin_access: bool
}