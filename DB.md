# DB Guide

This project supports both local SQLite and remote Turso (libSQL).

## Environment file setup

Create env files from examples:

```bash
make env-init

# or manually
cp .env.local.example .env.local
cp .env.production.example .env.production
```

Edit `.env.production` and set your real token:

```bash
FLAT_NOTE_DB_AUTH_TOKEN=<your-token>
```

## Local development DB

### Install sqlite3

```bash
# usually already installed on macOS
brew install sqlite
```

### Seed local.db

```bash
make db-seed
```

### Query local pages

```bash
make db-query
```

### Query current hardcoded user

```bash
sqlite3 local.db "SELECT * FROM pages WHERE owner_id = 'me-local-001';"
```

### Clear local pages

```bash
make db-clear
```

### Run app with local DB

```bash
make dev
```

`make dev` automatically loads `.env.local` if present.

## Production DB (Turso)

### 1) Create DB

```bash
turso db create flat-note
```

### 2) Get DB URL

```bash
turso db show flat-note
```

Example URL:

```text
libsql://flat-note-katayama8000.aws-ap-northeast-1.turso.io
```

### 3) Create auth token

```bash
turso db tokens create flat-note
```

### 4) Seed production DB

```bash
turso db shell flat-note < sql/seed.sql
```

### 5) Verify seeded data

```bash
turso db shell flat-note "SELECT id, owner_id, title FROM pages ORDER BY id;"
```

## Connect app to Turso production

`src-tauri` reads the following environment variables:

- `FLAT_NOTE_DB_URL` (default: `file:local.db`)
- `FLAT_NOTE_DB_AUTH_TOKEN` (required for remote Turso)

Run desktop app against Turso:

```bash
FLAT_NOTE_DB_URL="libsql://flat-note-katayama8000.aws-ap-northeast-1.turso.io" \
FLAT_NOTE_DB_AUTH_TOKEN="<your-token>" \
deno task tauri dev
```

Or using env file:

```bash
make dev-prod
```

Build desktop app against Turso:

```bash
FLAT_NOTE_DB_URL="libsql://flat-note-katayama8000.aws-ap-northeast-1.turso.io" \
FLAT_NOTE_DB_AUTH_TOKEN="<your-token>" \
deno task tauri build
```

Or using env file:

```bash
make build-prod
```

## Note

Current application user id is hardcoded to `me-local-001` in backend commands.
