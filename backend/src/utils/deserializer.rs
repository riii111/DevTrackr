use bson::DateTime as BsonDateTime;
use serde::de::{self, Deserializer};
use serde::Deserialize;

// カスタムデシリアライザ関数
pub fn deserialize_bson_date_time<'de, D>(deserializer: D) -> Result<BsonDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    BsonDateTime::parse_rfc3339_str(&s).map_err(de::Error::custom)
}

// Option<BsonDateTime>用のデシリアライザ
pub fn deserialize_option_bson_date_time<'de, D>(
    deserializer: D,
) -> Result<Option<BsonDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)?
        .map(|s| BsonDateTime::parse_rfc3339_str(&s))
        .transpose()
        .map_err(de::Error::custom)
}
