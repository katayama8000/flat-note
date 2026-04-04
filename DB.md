# Local DB Development (No Server)

## Install

```bash
# sqlite3 (usually already installed on macOS)
brew install sqlite
```

## Commands

### seed local.db with initial data

```bash
make db-seed
```

### query pages

```bash
make db-query
```

### query pages for current hardcoded user

```bash
sqlite3 local.db "SELECT * FROM pages WHERE owner_id = 'me-local-001';"
```

### clear all data from pages table

```bash
make db-clear
```

### run app (uses local.db by default)

```bash
make dev
```

## Optional: switch DB URL

`src-tauri` reads `FLAT_NOTE_DB_URL`. Default is `file:local.db`.

```bash
FLAT_NOTE_DB_URL=file:local.db make dev
```
