use chrono::{Utc};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

#[derive(Iden)]
pub enum Guilds {
    Table,
    Id,
    GuildId,
    CreatedAt,
    UpdatedAt
}

pub struct GuildInsertValue {
    pub guild_id: String
}

#[derive(Debug, FromRow)]
pub struct GuildStruct {
    pub id: Uuid,
    pub guild_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn find_guild(pool: &PgPool, guild_id: String) -> Result<GuildStruct, sqlx::Error> {
    let (sql, values) = Query::select()
        .from(Guilds::Table)
        .columns([
            Guilds::Id,
            Guilds::GuildId,
            Guilds::CreatedAt,
            Guilds::UpdatedAt
        ])
        .and_where(Expr::col(Guilds::GuildId).eq(guild_id))
        .build_sqlx(PostgresQueryBuilder);

    let row = sqlx::query_with(&sql, values)
        .fetch_one(pool)
        .await?;

    Ok(GuildStruct {
        id: row.try_get("id")?,
        guild_id: row.try_get("guild_id")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?
    })
}

pub async fn insert_guild(pool: &PgPool, guild_id: String) -> Result<GuildStruct, sqlx::Error> {
    let now = Utc::now();
    let new_id = Uuid::new_v4();

    let (sql, values) = Query::insert()
        .into_table(Guilds::Table)
        .columns([
            Guilds::Id,
            Guilds::GuildId,
        ])
        .values_panic([
            new_id.into(),
            guild_id.into(),
        ])
        .returning(Query::returning()
            .columns([
                Guilds::Id,
                Guilds::GuildId,
                Guilds::CreatedAt,
                Guilds::UpdatedAt
            ]))
        .build_sqlx(PostgresQueryBuilder);

    let row = sqlx::query_with(&sql, values)
        .fetch_one(pool)
        .await?;

    Ok(GuildStruct {
        id: row.try_get("id")?,
        guild_id: row.try_get("guild_id")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?
    })
}

pub async fn upsert_guild(pool: &PgPool, guild_id: String) -> Result<GuildStruct, sqlx::Error> {
    let guild = find_guild(&pool, guild_id.clone()).await;

    match guild {
        Ok(guild) => {
            Ok(guild)
        },
        Err(sqlx::Error::RowNotFound) => {
            let guild = insert_guild(pool, guild_id.clone()).await?;
            Ok(guild)
        },
        Err(e) => {
            Err(e)
        }
    }
}
