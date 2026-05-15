use actix_session::Session;
use central_core::{AppError, user::SessionUser};

use crate::session::get_session_user;

/// Extract the current user from the session, returning Unauthenticated if not logged in.
pub fn require_auth(session: &Session) -> Result<SessionUser, AppError> {
    get_session_user(session)?
        .ok_or(AppError::Unauthenticated)
}

/// Require Resident tier or above — any Discord login.
pub fn require_resident(session: &Session) -> Result<SessionUser, AppError> {
    let user = require_auth(session)?;
    if user.tier.is_resident() {
        Ok(user)
    } else {
        Err(AppError::Forbidden)
    }
}

/// Require Operator tier or above — manually granted.
pub fn require_operator(session: &Session) -> Result<SessionUser, AppError> {
    let user = require_auth(session)?;
    if user.tier.is_operator() {
        Ok(user)
    } else {
        Err(AppError::Forbidden)
    }
}

/// Require Architect tier — full platform admins only.
pub fn require_architect(session: &Session) -> Result<SessionUser, AppError> {
    let user = require_auth(session)?;
    if user.tier.is_architect() {
        Ok(user)
    } else {
        Err(AppError::Forbidden)
    }
}

/// Extract optional user — does not fail if not logged in.
/// Use for routes that are public but show different content when authenticated.
pub fn optional_user(session: &Session) -> Option<SessionUser> {
    get_session_user(session).ok().flatten()
}
