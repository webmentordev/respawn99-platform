-- Add migration script here
CREATE TABLE account_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    account_id UUID NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    label VARCHAR(255) NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_account_items_account_id ON account_items (account_id, sort_order);