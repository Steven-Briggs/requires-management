use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Platform access tier.
/// Transient and Resident are granted automatically on login.
/// Operator and Architect are granted manually.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, sqlx::Type)]
#[sqlx(type_name = "user_tier", rename_all = "lowercase")]
pub enum UserTier {
    /// Not logged in — public tools only
    Transient,
    /// Any Discord login — personal features enabled
    Resident,
    /// Manually granted — trusted platform contributors
    Operator,
    /// Manually granted — full platform admins
    Architect,
}

impl UserTier {
    pub fn is_resident(&self) -> bool {
        matches!(self, UserTier::Resident | UserTier::Operator | UserTier::Architect)
    }

    pub fn is_operator(&self) -> bool {
        matches!(self, UserTier::Operator | UserTier::Architect)
    }

    pub fn is_architect(&self) -> bool {
        matches!(self, UserTier::Architect)
    }
}

/// A platform user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id:               Uuid,
    pub discord_id:       String,
    pub discord_username: String,
    pub display_name:     String,
    pub avatar_url:       Option<String>,
    pub tier:             UserTier,
    pub created_at:       DateTime<Utc>,
    pub last_login:       DateTime<Utc>,
}

/// Minimal user info safe to store in session cookies and return from API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionUser {
    pub id:           Uuid,
    pub discord_id:   String,
    pub display_name: String,
    pub avatar_url:   Option<String>,
    pub tier:         UserTier,
}

impl From<User> for SessionUser {
    fn from(u: User) -> Self {
        Self {
            id:           u.id,
            discord_id:   u.discord_id,
            display_name: u.display_name,
            avatar_url:   u.avatar_url,
            tier:         u.tier,
        }
    }
}
