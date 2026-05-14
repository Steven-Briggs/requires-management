-- requires.management platform schema
-- Migration 0001: core user infrastructure

CREATE TYPE user_tier AS ENUM ('public', 'registered', 'clan', 'admin');

CREATE TABLE users (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    discord_id       TEXT NOT NULL UNIQUE,
    discord_username TEXT NOT NULL,
    display_name     TEXT NOT NULL,
    avatar_url       TEXT,
    tier             user_tier NOT NULL DEFAULT 'registered',
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_discord_id ON users(discord_id);
