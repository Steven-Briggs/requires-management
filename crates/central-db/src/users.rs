use central_core::user::{User, UserTier};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

/// Upsert a user on login — creates on first login, updates display info on subsequent logins.
/// Returns the full user record including their current tier.
pub async fn upsert_user(
    pool: &PgPool,
    discord_id: &str,
    discord_username: &str,
    display_name: &str,
    avatar_url: Option<&str>,
) -> Result<User, sqlx::Error> {
    let now = Utc::now();

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, discord_id, discord_username, display_name, avatar_url, tier, created_at, last_login)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (discord_id) DO UPDATE SET
            discord_username = EXCLUDED.discord_username,
            display_name     = EXCLUDED.display_name,
            avatar_url       = EXCLUDED.avatar_url,
            last_login       = EXCLUDED.last_login
        RETURNING
            id,
            discord_id,
            discord_username,
            display_name,
            avatar_url,
            tier AS "tier: UserTier",
            created_at,
            last_login
        "#,
        Uuid::new_v4(),
        discord_id,
        discord_username,
        display_name,
        avatar_url,
        UserTier::Registered as UserTier,
        now,  // created_at - $7
        now,  // last_login - $8
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Update a user's tier. Called after clan membership check on login.
pub async fn set_user_tier(
    pool: &PgPool,
    user_id: Uuid,
    tier: UserTier,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE users SET tier = $1 WHERE id = $2",
        tier as UserTier,
        user_id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Fetch a user by their internal UUID.
pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            discord_id,
            discord_username,
            display_name,
            avatar_url,
            tier AS "tier: UserTier",
            created_at,
            last_login
        FROM users WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;
    Ok(user)
}
