use bson::DateTime as BsonDateTime;
use serde::de::{self, Deserializer};
use serde::Deserialize;

// カスタムデシリアライザ関数
// ※RFC 3339...ISO 8601のサブセット. 広く認知された標準形式
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

// ProjectQueryのスキルラベルのデシリアライザ
pub fn deserialize_skill_labels<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum SkillLabels {
        Single(String),
        Multiple(Vec<String>),
    }

    match Option::<SkillLabels>::deserialize(deserializer)? {
        Some(SkillLabels::Single(s)) => Ok(Some(vec![s])),
        Some(SkillLabels::Multiple(vec)) => Ok(Some(vec)),
        None => Ok(None),
    }
}
