use chrono::{Utc};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;
use crate::queries::guild::{find_guild, upsert_guild};

#[derive(Iden)]
pub enum GuildConfigs {
    Table,
    Id,
    GuildId,
    DefaultChannelId,
    CreatedAt,
    UpdatedAt
}

pub struct GuildConfigInsertValue {
    pub guild_id: String,
    pub default_channel_id: String,
}

#[derive(Debug, FromRow)]
pub struct GuildConfigStruct {
    pub id: Uuid,
    pub guild_id: Uuid,
    pub default_channel_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn find_guild_config(pool: &PgPool, guild_id: Uuid) -> Result<GuildConfigStruct, sqlx::Error> {
    let (sql, values) = Query::select()
        .from(GuildConfigs::Table)
        .columns([
            GuildConfigs::Id,
            GuildConfigs::GuildId,
            GuildConfigs::DefaultChannelId,
            GuildConfigs::CreatedAt,
            GuildConfigs::UpdatedAt
        ])
        .and_where(Expr::col(GuildConfigs::GuildId).eq(guild_id))
        .build_sqlx(PostgresQueryBuilder);

    let row = sqlx::query_with(&sql, values)
        .fetch_one(pool)
        .await?;

    Ok(GuildConfigStruct {
        id: row.try_get("id")?,
        guild_id: row.try_get("guild_id")?,
        default_channel_id: row.try_get("default_channel_id")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?
    })
}

pub async fn insert_guild_config(pool: &PgPool, guild_id: Uuid, guild_config: GuildConfigInsertValue) -> Result<GuildConfigStruct, sqlx::Error> {
    let now = Utc::now();
    let new_id = Uuid::new_v4();

    let (sql, values) = Query::insert()
        .into_table(GuildConfigs::Table)
        .columns([
            GuildConfigs::Id,
            GuildConfigs::GuildId,
            GuildConfigs::DefaultChannelId,
            GuildConfigs::CreatedAt,
            GuildConfigs::UpdatedAt
        ])
        .values_panic([
            new_id.into(),
            guild_id.into(),
            guild_config.default_channel_id.into(),
            now.into(),
            now.into()
        ])
        .returning(Query::returning()
            .columns([
                GuildConfigs::Id,
                GuildConfigs::GuildId,
                GuildConfigs::DefaultChannelId,
                GuildConfigs::CreatedAt,
                GuildConfigs::UpdatedAt
            ]))
        .build_sqlx(PostgresQueryBuilder);

    let row = sqlx::query_with(&sql, values)
        .fetch_one(pool)
        .await?;

    Ok(GuildConfigStruct {
        id: row.try_get("id")?,
        guild_id: row.try_get("guild_id")?,
        default_channel_id: row.try_get("default_channel_id")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?
    })
}

pub async fn update_guild_config(pool: &PgPool, guild_config: GuildConfigInsertValue) -> Result<GuildConfigStruct, sqlx::Error> {
    let fguild = find_guild(pool, guild_config.guild_id.clone()).await?;
    let fguild_config = find_guild_config(pool, fguild.id).await?;

    let (sql, values) = Query::update()
        .table(GuildConfigs::Table)
        .values([
            (GuildConfigs::DefaultChannelId, guild_config.default_channel_id.into())
        ])
        .and_where(Expr::col(GuildConfigs::Id).eq(fguild_config.id))
        .returning(Query::returning()
            .columns([
                GuildConfigs::Id,
                GuildConfigs::GuildId,
                GuildConfigs::DefaultChannelId,
                GuildConfigs::CreatedAt,
                GuildConfigs::UpdatedAt
            ]))
        .build_sqlx(PostgresQueryBuilder);

    let row = sqlx::query_with(&sql, values)
        .fetch_one(pool)
        .await?;

    Ok(GuildConfigStruct {
        id: row.try_get("id")?,
        guild_id: row.try_get("guild_id")?,
        default_channel_id: row.try_get("default_channel_id")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?
    })
}

pub async fn upsert_guild_config(pool: &PgPool, guild_config: GuildConfigInsertValue) -> Result<GuildConfigStruct, sqlx::Error> {
    let fguild = upsert_guild(&pool, guild_config.guild_id.clone()).await?;
    let fguild_config = find_guild_config(&pool, fguild.id).await;

    match fguild_config {
        Ok(_) => {
            let update_guild_config = update_guild_config(pool, guild_config).await?;
            Ok(update_guild_config)
        },
        Err(sqlx::Error::RowNotFound) => {
            let guild_configs = insert_guild_config(pool, fguild.id, guild_config).await?;
            Ok(guild_configs)
        },
        Err(e) => {
            Err(e)
        }
    }
}
