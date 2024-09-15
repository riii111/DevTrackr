use bson::DateTime as BsonDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingTime {
    pub start_time: Option<BsonDateTime>,
    pub end_time: Option<BsonDateTime>,
}
