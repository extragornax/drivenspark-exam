use super::schema::data;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
pub struct Card {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub date: NaiveDate,
    pub priority: String,
    pub duration: i32,
    pub status: String,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[table_name = "data"]
pub struct CardData {
    pub title: String,
    pub description: String,
    pub date: NaiveDate,
    pub priority: String,
    pub duration: i32,
    pub status: String,
}
