use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow)]
pub struct Guild {
    pub id: Uuid,
    pub guild_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct LogType {
    pub id: Uuid,
    pub guild_id: Uuid,
    pub channel_id: Option<String>,
    pub log_type: LogTypeEnum,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct GuildConfig {
    pub id: Uuid,
    pub guild_id: Uuid,
    pub default_channel_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "log_type")]
pub enum LogTypeEnum {
    MessageDelete,
}
