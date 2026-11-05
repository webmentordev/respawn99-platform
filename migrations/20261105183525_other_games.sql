-- Add migration script here
CREATE TABLE other_games (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    name VARCHAR(150) NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_other_games_name UNIQUE (name)
);

CREATE INDEX idx_other_games_sort_order ON other_games (sort_order);