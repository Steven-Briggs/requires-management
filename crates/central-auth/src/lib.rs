pub mod discord;
pub mod middleware;
pub mod session;

pub use discord::DiscordOAuth;
pub use middleware::{optional_user, require_admin, require_auth, require_clan, require_registered};
pub use session::{clear_session_user, get_session_user, set_session_user};

/// The key used to store the session user in the actix-session store.
pub const SESSION_USER_KEY: &str = "user";

/// The key used to store the OAuth state parameter (CSRF protection).
pub const OAUTH_STATE_KEY: &str = "oauth_state";
