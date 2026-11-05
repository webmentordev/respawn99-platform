-- Add migration script here
CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    game_id UUID NOT NULL REFERENCES games (id) ON DELETE RESTRICT,
    platform_id UUID NOT NULL REFERENCES platforms (id) ON DELETE RESTRICT,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    hours INTEGER NOT NULL DEFAULT 0,
    level INTEGER,
    price_cents INTEGER NOT NULL,
    stock INTEGER NOT NULL DEFAULT 1,
    is_published BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_accounts_hours CHECK (hours >= 0),
    CONSTRAINT chk_accounts_price CHECK (price_cents >= 0),
    CONSTRAINT chk_accounts_stock CHECK (stock >= 0)
);

CREATE INDEX idx_accounts_game_id ON accounts (game_id);

CREATE INDEX idx_accounts_platform_id ON accounts (platform_id);

CREATE INDEX idx_accounts_is_published ON accounts (is_published);

CREATE INDEX idx_accounts_price_cents ON accounts (price_cents);