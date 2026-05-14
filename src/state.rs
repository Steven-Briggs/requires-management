use central_auth::DiscordOAuth;
use sqlx::PgPool;

/// Shared application state injected into every handler via actix-web Data.
pub struct AppState {
    pub pool: PgPool,
    pub discord: DiscordOAuth,
}
