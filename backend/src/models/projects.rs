use bson::oid::ObjectId;
use chrono::DateTime;
use chrono_tz::Jst;
use serde::{Deserialize, Serialize};

pub struct WorkingTime {
    pub start_time: Option<DateTime<Jst>>,
    pub end_time: Option<DateTime<Jst>>,
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
    pub created_at: DateTime<Jst>,
    pub updated_at: Option<DateTime<Jst>>,
}
