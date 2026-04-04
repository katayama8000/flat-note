# Turso Local Development

## Install

```bash
# Turso CLI
curl -sSfL https://get.tur.so/install.sh | bash

# sqld (required for --db-file)
brew install libsql/sqld/sqld
```

## Commands

### start local Turso instance with local.db file

```bash
turso dev --db-file local.db
```

### seed local.db with initial data

```bash
turso db shell http://127.0.0.1:8080 < sql/seed.sql
```

### Query pages

```bash
turso db shell http://127.0.0.1:8080 "SELECT * FROM pages;"
```

### Query pages for current hardcoded user

```bash
turso db shell http://127.0.0.1:8080 "SELECT * FROM pages WHERE owner_id = 'me-local-001';"
```

### Clear all data from pages table

```bash
turso db shell http://127.0.0.1:8080 < sql/clear.sql
```
