pub mod queries;

use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sea_query::Iden;

#[derive(Debug, FromRow)]
pub struct LogType {
    pub id: Uuid,
    pub guild_id: Uuid,
    pub channel_id: String,
    pub log_type: LogTypeEnum,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "log_type")]
pub enum LogTypeEnum {
    MessageDelete,
}
