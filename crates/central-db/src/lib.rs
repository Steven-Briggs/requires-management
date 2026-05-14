pub mod users;

use anyhow::Result;
use sqlx::PgPool;

/// Create a connection pool from the DATABASE_URL environment variable.
pub async fn connect() -> Result<PgPool> {
    let url = std::env::var("DATABASE_URL")
        .map_err(|_| anyhow::anyhow!("DATABASE_URL not set"))?;

    let pool = PgPool::connect(&url).await?;
    Ok(pool)
}

/// Run all pending SQLx migrations.
pub async fn migrate(pool: &PgPool) -> Result<()> {
    sqlx::migrate!().run(pool).await?;
    Ok(())
}
