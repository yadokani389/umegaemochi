use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DisasterInfo {
    title: String,
    description: String,
    warning: String,
    occurred: NaiveDateTime,
}
