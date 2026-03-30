CREATE TABLE IF NOT EXISTS pages (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    last_used_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS page_tokens (
    page_id TEXT NOT NULL,
    token_id INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (page_id, token_id),
    FOREIGN KEY (page_id) REFERENCES pages(id) ON DELETE CASCADE,
    FOREIGN KEY (token_id) REFERENCES tokens(id) ON DELETE CASCADE
);

INSERT OR IGNORE INTO pages (id, title, description, created_at, updated_at) VALUES
    ('1', 'Getting Started', 'An introduction to flat-note and how to use it. #getting-started', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('2', 'Rust & Tauri', 'Notes on building desktop apps with Rust and Tauri. #rust #tauri', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('3', 'libSQL', 'Notes on using libSQL and Turso for local development. #libsql #turso', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

INSERT OR IGNORE INTO tokens (name) VALUES ('rust'), ('tauri'), ('libsql'), ('getting-started'), ('turso');

INSERT OR IGNORE INTO page_tokens (page_id, token_id) VALUES
    ('1', (SELECT id FROM tokens WHERE name = 'getting-started')),
    ('2', (SELECT id FROM tokens WHERE name = 'rust')),
    ('2', (SELECT id FROM tokens WHERE name = 'tauri')),
    ('3', (SELECT id FROM tokens WHERE name = 'libsql')),
    ('3', (SELECT id FROM tokens WHERE name = 'turso'));
