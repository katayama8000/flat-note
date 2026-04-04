DROP TABLE IF EXISTS pages;

CREATE TABLE pages (
    id TEXT PRIMARY KEY,
    owner_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_pages_owner_id_updated_at ON pages (owner_id, updated_at DESC);

INSERT INTO pages (id, owner_id, title, description, created_at, updated_at) VALUES
    ('1', 'me-local-001', 'Getting Started', 'An introduction to flat-note and how to use it.', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('2', 'me-local-001', 'Rust & Tauri', 'Notes on building desktop apps with Rust and Tauri.', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('3', 'me-local-001', 'libSQL', 'Notes on using libSQL and Turso for local development.', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
