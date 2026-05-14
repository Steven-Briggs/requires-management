use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// Configuration for Discord OAuth — loaded from environment variables.
#[derive(Debug, Clone)]
pub struct DiscordOAuth {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    /// The Discord guild (server) ID used for clan tier verification.
    pub clan_guild_id: String,
    /// The Discord role ID within the clan guild that grants Clan tier.
    /// If None, all guild members get Clan tier.
    pub clan_role_id: Option<String>,
}

impl DiscordOAuth {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            client_id: std::env::var("DISCORD_CLIENT_ID")
                .map_err(|_| anyhow!("DISCORD_CLIENT_ID not set"))?,
            client_secret: std::env::var("DISCORD_CLIENT_SECRET")
                .map_err(|_| anyhow!("DISCORD_CLIENT_SECRET not set"))?,
            redirect_uri: std::env::var("DISCORD_REDIRECT_URI")
                .map_err(|_| anyhow!("DISCORD_REDIRECT_URI not set"))?,
            clan_guild_id: std::env::var("DISCORD_CLAN_GUILD_ID")
                .map_err(|_| anyhow!("DISCORD_CLAN_GUILD_ID not set"))?,
            clan_role_id: std::env::var("DISCORD_CLAN_ROLE_ID").ok(),
        })
    }

    /// Build the Discord OAuth authorization URL.
    /// The state parameter is a random string for CSRF protection — store it in the session
    /// before redirecting and verify it matches when Discord calls back.
    pub fn authorization_url(&self, state: &str) -> String {
        format!(
            "https://discord.com/oauth2/authorize\
             ?client_id={}\
             &redirect_uri={}\
             &response_type=code\
             &scope=identify+guilds+guilds.members.read\
             &state={}",
            self.client_id,
            urlencoding::encode(&self.redirect_uri),
            state,
        )
    }

    /// Exchange the authorization code for an access token.
    pub async fn exchange_code(&self, code: &str) -> Result<TokenResponse> {
        let client = reqwest::Client::new();
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", self.redirect_uri.as_str()),
        ];

        let resp = client
            .post("https://discord.com/api/oauth2/token")
            .form(&params)
            .send()
            .await?
            .error_for_status()?
            .json::<TokenResponse>()
            .await?;

        Ok(resp)
    }

    /// Fetch the Discord user associated with the access token.
    pub async fn fetch_user(&self, access_token: &str) -> Result<DiscordUser> {
        let client = reqwest::Client::new();
        let user = client
            .get("https://discord.com/api/users/@me")
            .bearer_auth(access_token)
            .send()
            .await?
            .error_for_status()?
            .json::<DiscordUser>()
            .await?;
        Ok(user)
    }

    /// Check whether the user is a member of the clan guild and optionally holds the clan role.
    /// Returns true if the user qualifies for Clan tier.
    pub async fn check_clan_membership(
        &self,
        access_token: &str,
        user_id: &str,
    ) -> Result<bool> {
        let client = reqwest::Client::new();

        // Check guild membership
        let url = format!(
            "https://discord.com/api/users/@me/guilds/{}/member",
            self.clan_guild_id
        );
        let resp = client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND
            || resp.status() == reqwest::StatusCode::FORBIDDEN
        {
            return Ok(false);
        }

        let member: GuildMember = resp.error_for_status()?.json().await?;

        // If a specific role is required, check for it
        if let Some(role_id) = &self.clan_role_id {
            return Ok(member.roles.contains(role_id));
        }

        // No specific role required — guild membership is enough
        let _ = user_id; // suppress unused warning
        Ok(true)
    }

    /// Build the avatar URL for a Discord user.
    pub fn avatar_url(user: &DiscordUser) -> Option<String> {
        user.avatar.as_ref().map(|hash| {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png?size=128",
                user.id, hash
            )
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GuildMember {
    pub roles: Vec<String>,
}
