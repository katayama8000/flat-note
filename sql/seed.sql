CREATE TABLE IF NOT EXISTS pages (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO pages (id, title, description, created_at, updated_at) VALUES
    ('1', 'Getting Started', 'An introduction to flat-note and how to use it.', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('2', 'Rust & Tauri', 'Notes on building desktop apps with Rust and Tauri.', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('3', 'libSQL', 'Notes on using libSQL and Turso for local development.', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
