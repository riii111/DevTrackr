use bson::DateTime as BsonDateTime;
use serde::de;
use serde::Deserialize;

/// カスタムデシリアライザ関数
/// ※RFC 3339...ISO 8601のサブセット. 広く認知された標準形式
pub fn deserialize_bson_date_time<'de, D>(deserializer: D) -> Result<BsonDateTime, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    BsonDateTime::parse_rfc3339_str(&s).map_err(de::Error::custom)
}

// Option<BsonDateTime>用のデシリアライザ
pub fn deserialize_option_bson_date_time<'de, D>(
    deserializer: D,
) -> Result<Option<BsonDateTime>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)?
        .map(|s| BsonDateTime::parse_rfc3339_str(&s))
        .transpose()
        .map_err(de::Error::custom)
}

/// ProjectQueryのスキルラベルのデシリアライザ
pub fn deserialize_skill_labels<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: de::Deserializer<'de>,
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

/// ソートパラメータのカスタムデシリアライザ
pub fn deserialize_sort_params<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let sort_param: Option<String> = Option::deserialize(deserializer)?;

    if let Some(param) = sort_param {
        let sort_values: Vec<String> = param.split(',').map(|s| s.trim().to_string()).collect();

        // 空の場合はNoneを返す
        if sort_values.is_empty() {
            return Ok(None);
        }

        // 各パラメータの形式を検証
        for param in &sort_values {
            let parts: Vec<&str> = param.split(':').collect();
            if parts.len() != 2 {
                return Err(de::Error::custom(
                    "Invalid sort format. Expected 'field:order'",
                ));
            }

            let order = parts[1].to_lowercase();
            if order != "asc" && order != "desc" {
                return Err(de::Error::custom(
                    "Sort order must be either 'asc' or 'desc'",
                ));
            }
        }

        Ok(Some(sort_values))
    } else {
        Ok(None)
    }
}

pub fn deserialize_string_array<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct StringArrayVisitor;

    impl<'de> de::Visitor<'de> for StringArrayVisitor {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or array of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value.to_string()])
        }

        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            de::Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(StringArrayVisitor)
}
