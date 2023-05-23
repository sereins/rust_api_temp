/// 时间序列化
pub mod serde_datetime {
    use sqlx::types::chrono::NaiveDateTime;
    use serde::{Deserialize, Deserializer, Serializer};

    const FMT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        return s.serialize_str(&date.format(FMT).to_string());
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
        where D: Deserializer<'de> {
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = s {
            return Ok(NaiveDateTime::parse_from_str(&s, FMT).map_err(serde::de::Error::custom)?);
        }
        Ok(NaiveDateTime::default())
    }
}