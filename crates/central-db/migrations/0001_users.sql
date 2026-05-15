-- requires.management platform schema
-- Migration 0001: core user infrastructure

DO $$ BEGIN
    CREATE TYPE user_tier AS ENUM ('transient', 'resident', 'operator', 'architect');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

CREATE TABLE IF NOT EXISTS users (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    discord_id       TEXT NOT NULL UNIQUE,
    discord_username TEXT NOT NULL,
    display_name     TEXT NOT NULL,
    avatar_url       TEXT,
    tier             user_tier NOT NULL DEFAULT 'resident',
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_users_discord_id ON users(discord_id);
