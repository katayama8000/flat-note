# Turso Local Development

## Install

```bash
# Turso CLI
curl -sSfL https://get.tur.so/install.sh | bash

# sqld (required for --db-file)
brew install libsql/sqld/sqld
```

## Start the server

`turso dev` uses an in-memory DB by default — data is lost on stop. Use
`--db-file` to persist:

```bash
turso dev --db-file local.db
```

## Seed data

```bash
turso db shell http://127.0.0.1:8080 < seed.sql
```

## Verify

```bash
turso db shell http://127.0.0.1:8080 "SELECT * FROM pages;"
```

## GUI clients

- [TablePlus](https://tableplus.com/)
- [Outerbase Studio](https://libsqlstudio.com/) — browser-based

Connect to `http://127.0.0.1:8080` (no auth token required).
