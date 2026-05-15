pub mod discord;
pub mod middleware;
pub mod session;

pub use discord::DiscordOAuth;
pub use middleware::{optional_user, require_architect, require_auth, require_operator, require_resident};
pub use session::{clear_session_user, get_session_user, set_session_user};

pub const SESSION_USER_KEY: &str = "user";
pub const OAUTH_STATE_KEY:  &str = "oauth_state";
