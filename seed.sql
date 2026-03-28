CREATE TABLE IF NOT EXISTS pages (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO pages (id, title, description) VALUES
    ('1', 'Getting Started', 'An introduction to flat-note and how to use it.'),
    ('2', 'Rust & Tauri', 'Notes on building desktop apps with Rust and Tauri.'),
    ('3', 'libSQL', 'Notes on using libSQL and Turso for local development.');
