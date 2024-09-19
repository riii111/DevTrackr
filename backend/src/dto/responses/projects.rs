use bson::oid::ObjectId;
use serde::Serialize;
#[derive(Serialize, Debug)]
pub struct ProjectCreatedResponse {
    #[serde(serialize_with = "serialize_object_id")]
    pub id: ObjectId,
}

impl From<ObjectId> for ProjectCreatedResponse {
    fn from(id: ObjectId) -> Self {
        Self { id }
    }
}
