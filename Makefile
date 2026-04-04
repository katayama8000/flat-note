.PHONY: dev db-seed db-clear db-query fmt fmt-check lint check

DB_FILE := $(CURDIR)/local.db

dev:
	FLAT_NOTE_DB_URL="file:$(DB_FILE)" deno task tauri dev

db-seed:
	sqlite3 "$(DB_FILE)" < sql/seed.sql

db-clear:
	sqlite3 "$(DB_FILE)" < sql/clear.sql

db-query:
	sqlite3 "$(DB_FILE)" "SELECT id, owner_id, title, created_at, updated_at FROM pages ORDER BY updated_at DESC;"

fmt:
	deno fmt
	cd src-tauri && cargo fmt

fmt-check:
	deno fmt --check
	cd src-tauri && cargo fmt --check

lint:
	deno lint
	cd src-tauri && cargo clippy

check:
	deno check
	cd src-tauri && cargo check

all: fmt lint check