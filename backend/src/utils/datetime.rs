use chrono::{DateTime, Local, TimeZone};
use chrono_tz::Asia::Tokyo;

fn local_to_bson(local_datetime: Datetime<Local>) -> BsonDateTime {
    BsonDateTime::from_chrono(local_datetime.with_timezone(&Utc))
}

fn bson_to_local(bson_datetime: BsonDateTime) -> DateTime<Local> {
    Utc.from_utc_datetime(&bson_datetime.to_chrono())
        .with_timezone(&Tokyo)
}
