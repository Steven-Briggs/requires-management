use actix_session::Session;
use central_core::{AppError, user::SessionUser};

use crate::session::get_session_user;

/// Extract the current user from the session, returning Unauthenticated if not logged in.
/// Use this in handlers that require any login.
pub fn require_auth(session: &Session) -> Result<SessionUser, AppError> {
    get_session_user(session)?
        .ok_or(AppError::Unauthenticated)
}

/// Require a Registered or higher tier user.
pub fn require_registered(session: &Session) -> Result<SessionUser, AppError> {
    let user = require_auth(session)?;
    if user.tier.can_access_registered() {
        Ok(user)
    } else {
        Err(AppError::Forbidden)
    }
}

/// Require a Clan or higher tier user.
pub fn require_clan(session: &Session) -> Result<SessionUser, AppError> {
    let user = require_auth(session)?;
    if user.tier.can_access_clan() {
        Ok(user)
    } else {
        Err(AppError::Forbidden)
    }
}

/// Require an Admin tier user.
pub fn require_admin(session: &Session) -> Result<SessionUser, AppError> {
    let user = require_auth(session)?;
    if user.tier.is_admin() {
        Ok(user)
    } else {
        Err(AppError::Forbidden)
    }
}

/// Extract optional user info — does not fail if not logged in.
/// Use this for routes that are public but show different content when logged in.
pub fn optional_user(session: &Session) -> Option<SessionUser> {
    get_session_user(session).ok().flatten()
}
