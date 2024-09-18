use crate::models::working_times::WorkingTimeInDB;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct WorkingTimeResponse {
    pub id: ObjectId,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<WorkingTimeInDB> for WorkingTimeResponse {
    fn from(db_working_time: WorkingTimeInDB) -> Self {
        Self {
            id: db_working_time.id.expect("IDが存在しません"),
            start_time: db_working_time.start_time,
            end_time: db_working_time.end_time,
            created_at: db_working_time.created_at,
            updated_at: db_working_time.updated_at,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct WorkingTimeCreatedResponse {
    pub id: ObjectId,
}

impl From<ObjectId> for WorkingTimeCreatedResponse {
    fn from(id: ObjectId) -> Self {
        Self { id: id }
    }
}
