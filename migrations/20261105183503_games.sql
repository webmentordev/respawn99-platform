-- Add migration script here
CREATE TABLE games (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    slug VARCHAR(50) NOT NULL,
    name VARCHAR(150) NOT NULL,
    tag TEXT NOT NULL,
    tint VARCHAR(200) NOT NULL,
    ring VARCHAR(100) NOT NULL,
    dot VARCHAR(100) NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_games_slug UNIQUE (slug),
    CONSTRAINT uq_games_name UNIQUE (name)
);

CREATE INDEX idx_games_sort_order ON games (sort_order);

CREATE INDEX idx_games_is_active ON games (is_active);