use actix_session::Session;
use central_core::{AppError, user::SessionUser};

/// Retrieve the current session user. Returns None if not logged in.
pub fn get_session_user(session: &Session) -> Result<Option<SessionUser>, AppError> {
    session
        .get::<SessionUser>(crate::SESSION_USER_KEY)
        .map_err(|e| AppError::Internal(format!("Session read error: {e}")))
}

/// Store a user in the session after successful OAuth login.
pub fn set_session_user(session: &Session, user: &SessionUser) -> Result<(), AppError> {
    session
        .insert(crate::SESSION_USER_KEY, user)
        .map_err(|e| AppError::Internal(format!("Session write error: {e}")))
}

/// Clear the session (logout).
pub fn clear_session_user(session: &Session) {
    session.purge();
}
