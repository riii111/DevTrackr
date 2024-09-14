use bson::oid::ObjectId;
use bson::DateTime as BsonDateTime;
use chrono::{DateTime, Local, TimeZone};
use chrono_tz::Asia::Tokyo; // JSTのタイムゾーン
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingTime {
    pub start_time: Option<BsonDateTime>,
    pub end_time: Option<BsonDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: ObjectId,
    pub title: String,
    pub description: Option<String>,
    pub skill_labels: Vec<String>,
    pub company_name: String,
    pub working_times: Option<Vec<WorkingTime>>,
    pub total_working_time: Option<i64>,
    pub created_at: BsonDateTime,
    pub updated_at: Option<BsonDateTime>,
}
