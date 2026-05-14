mod routes;
mod state;

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web::cookie::{Key, SameSite};
use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env first
    dotenvy::dotenv().ok();

    // Initialise tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "requires_management=info,actix_web=info".into()),
        )
        .init();

    info!("requires.management starting");

    // Database
    let pool = central_db::connect().await?;
    central_db::migrate(&pool).await?;
    info!("Database connected and migrations applied");

    // Auth config
    let discord = central_auth::DiscordOAuth::from_env()?;

    // Session key — must be at least 64 bytes
    let session_key_str = std::env::var("SESSION_SECRET_KEY")
        .expect("SESSION_SECRET_KEY must be set (at least 64 chars)");
    let session_key = Key::from(session_key_str.as_bytes());

    // App state
    let app_state = web::Data::new(state::AppState { pool, discord });

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    info!("Listening on {bind_addr}");

    let cookie_secure = std::env::var("COOKIE_SECURE")
        .map(|v| v == "true")
        .unwrap_or(false);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    session_key.clone(),
                )
                .cookie_secure(cookie_secure)
                .cookie_http_only(true)
                .cookie_same_site(SameSite::Lax)
                .build(),
            )
            .configure(routes::configure)
    })
    .bind(&bind_addr)?
    .run()
    .await?;

    Ok(())
}
