use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use central_auth::{
    session::{clear_session_user, get_session_user, set_session_user},
    OAUTH_STATE_KEY,
};
use central_core::{
    response::ApiResponse,
    user::SessionUser,
    AppError,
};
use tracing::{error, info};
use uuid::Uuid;

use crate::state::AppState;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/login",    web::get().to(login))
        .route("/callback", web::get().to(callback))
        .route("/logout",   web::post().to(logout));
}

/// Redirect to Discord OAuth authorization page.
async fn login(
    session:    Session,
    state:      web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let oauth_state = Uuid::new_v4().to_string();
    session
        .insert(OAUTH_STATE_KEY, &oauth_state)
        .map_err(|e| AppError::Internal(format!("Session error: {e}")))?;

    let url = state.discord.authorization_url(&oauth_state);
    Ok(HttpResponse::Found()
        .append_header(("Location", url))
        .finish())
}

#[derive(serde::Deserialize)]
pub struct CallbackQuery {
    pub code:  Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
}

/// Handle the Discord OAuth callback.
async fn callback(
    session:   Session,
    query:     web::Query<CallbackQuery>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    // User denied auth
    if let Some(err) = &query.error {
        info!("OAuth denied: {err}");
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/?auth=denied"))
            .finish());
    }

    // CSRF check
    let expected = session
        .get::<String>(OAUTH_STATE_KEY)
        .map_err(|e| AppError::Internal(format!("Session error: {e}")))?
        .ok_or_else(|| AppError::BadRequest("Missing OAuth state".into()))?;

    let received = query.state.as_deref()
        .ok_or_else(|| AppError::BadRequest("Missing state parameter".into()))?;

    if expected != received {
        return Err(AppError::BadRequest("OAuth state mismatch".into()));
    }

    let code = query.code.as_deref()
        .ok_or_else(|| AppError::BadRequest("Missing authorization code".into()))?;

    // Exchange code for token
    let token = app_state.discord
        .exchange_code(code)
        .await
        .map_err(|e| {
            error!("Token exchange failed: {e}");
            AppError::ExternalApi("Discord token exchange failed".into())
        })?;

    // Fetch Discord user
    let discord_user = app_state.discord
        .fetch_user(&token.access_token)
        .await
        .map_err(|e| {
            error!("Discord user fetch failed: {e}");
            AppError::ExternalApi("Failed to fetch Discord user".into())
        })?;

    let display_name = discord_user.global_name
        .clone()
        .unwrap_or_else(|| discord_user.username.clone());

    let avatar_url = central_auth::DiscordOAuth::avatar_url(&discord_user);

    // Upsert user — all logins start as Resident
    let user = central_db::users::upsert_user(
        &app_state.pool,
        &discord_user.id,
        &discord_user.username,
        &display_name,
        avatar_url.as_deref(),
    )
    .await
    .map_err(AppError::Database)?;

    let session_user = SessionUser {
        id:           user.id,
        discord_id:   user.discord_id,
        display_name: user.display_name.clone(),
        avatar_url:   user.avatar_url,
        tier:         user.tier,
    };

    set_session_user(&session, &session_user)?;
    info!("Login: {} ({})", user.display_name, discord_user.id);

    Ok(HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish())
}

/// Logout — clear session and redirect home.
async fn logout(session: Session) -> impl Responder {
    clear_session_user(&session);
    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

/// GET /api/v1/me — current session user as JSON.
pub async fn me(session: Session) -> Result<HttpResponse, AppError> {
    match get_session_user(&session)? {
        Some(u) => Ok(HttpResponse::Ok().json(ApiResponse::ok(u))),
        None    => Ok(HttpResponse::Unauthorized()
                        .json(ApiResponse::<()>::error("Not authenticated"))),
    }
}
