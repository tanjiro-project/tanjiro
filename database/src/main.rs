use sqlx::{PgPool, migrate::Migrator};
use dotenv::dotenv;
use std::env;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await?;

    MIGRATOR.run(&pool).await?;

    println!("Migrations ran successfully!");
    Ok(())
}
