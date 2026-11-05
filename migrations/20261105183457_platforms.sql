-- Add migration script here
CREATE TABLE platforms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    slug VARCHAR(50) NOT NULL,
    name VARCHAR(100) NOT NULL,
    icon VARCHAR(100) NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_platforms_slug UNIQUE (slug),
    CONSTRAINT uq_platforms_name UNIQUE (name)
);

CREATE INDEX idx_platforms_sort_order ON platforms (sort_order);