use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::Serializer;

// ObjectIdを16進数文字列としてシリアライズするためのヘルパー関数
pub fn serialize_object_id<S>(object_id: &ObjectId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&object_id.to_hex())
}

// BsonDateTimeをISO 8601形式の文字列としてシリアライズするためのヘルパー関数
// ※RFC 3339...ISO 8601のサブセット. 広く認知された標準形式
pub fn serialize_bson_datetime<S>(date: &BsonDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(
        &date
            .try_to_rfc3339_string()
            .map_err(serde::ser::Error::custom)?,
    )
}

// Option<BsonDateTime>をシリアライズするためのヘルパー関数
pub fn serialize_option_bson_datetime<S>(
    date: &Option<BsonDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(d) => serialize_bson_datetime(d, serializer),
        None => serializer.serialize_none(),
    }
}
