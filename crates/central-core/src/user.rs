use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The tier of access a user has across the platform.
/// Determined at login based on Discord server membership and roles.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, sqlx::Type)]
#[sqlx(type_name = "user_tier", rename_all = "lowercase")]
pub enum UserTier {
    /// Anonymous / not logged in — public features only
    Public,
    /// Logged in via Discord — personal features enabled
    Registered,
    /// Member of the clan Discord server — clan features enabled
    Clan,
    /// Server administrator
    Admin,
}

impl UserTier {
    pub fn can_access_registered(&self) -> bool {
        matches!(self, UserTier::Registered | UserTier::Clan | UserTier::Admin)
    }

    pub fn can_access_clan(&self) -> bool {
        matches!(self, UserTier::Clan | UserTier::Admin)
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, UserTier::Admin)
    }
}

/// A platform user. Populated on first Discord OAuth login and updated on subsequent logins.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub discord_id: String,
    pub discord_username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub tier: UserTier,
    pub created_at: DateTime<Utc>,
    pub last_login: DateTime<Utc>,
}

/// Minimal user info safe to expose in session cookies and API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: Uuid,
    pub discord_id: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub tier: UserTier,
}

impl From<User> for SessionUser {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            discord_id: u.discord_id,
            display_name: u.display_name,
            avatar_url: u.avatar_url,
            tier: u.tier,
        }
    }
}
