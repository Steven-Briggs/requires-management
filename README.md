# requires.management

Platform hub for all requires.management services. Handles auth, user identity, and serves the landing page at `requires.management`.

## Structure

```
requires-management/
├── crates/
│   ├── central-core/    shared types, errors, API response shapes
│   ├── central-auth/    Discord OAuth, session handling, user identity
│   └── central-db/      SQLx queries, migrations
├── src/                 Actix binary — requires.management
├── frontend/
│   └── public/          Vanilla JS SPA frontend
└── deploy/
    ├── nginx/           Nginx site config
    └── systemd/         systemd service unit
```

## Setup

### 1. Prerequisites

```bash
sudo pacman -S postgresql
# Ensure postgres is running and you have a user/database
```

### 2. Environment

```bash
cp .env.example .env
# Fill in: DATABASE_URL, SESSION_SECRET_KEY, DISCORD_* vars
```

### 3. Discord Application

Create an application at https://discord.com/developers/applications:
- Add OAuth2 redirect URI: `https://requires.management/auth/callback`
- Required scopes: `identify guilds guilds.members.read`
- Copy Client ID and Client Secret to `.env`

### 4. Database

```bash
# Create the database
createdb requires_management

# Migrations run automatically on startup via SQLx
```

### 5. Build & Run

```bash
cargo build --release
./target/release/requires-management
```

### 6. Deploy on Pi

```bash
# Nginx
sudo cp deploy/nginx/requires.management.conf /etc/nginx/sites-available/
sudo ln -s /etc/nginx/sites-available/requires.management.conf /etc/nginx/sites-enabled/
sudo certbot --nginx -d requires.management
sudo nginx -t && sudo systemctl reload nginx

# systemd
sudo cp deploy/systemd/requires-management.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now requires-management
```

## Adding a New Service

Each service (e.g. `warframe-companion`) is a separate repo. To consume the central crates:

```toml
# In the service's Cargo.toml
[dependencies]
central-core = { git = "https://github.com/RocinRykor/requires-management", tag = "v0.1.0" }
central-auth = { git = "https://github.com/RocinRykor/requires-management", tag = "v0.1.0" }
central-db   = { git = "https://github.com/RocinRykor/requires-management", tag = "v0.1.0" }
```

Each service handles its own Nginx config and systemd unit on the same Pi.
