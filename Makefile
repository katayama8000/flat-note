.PHONY: env-init dev dev-prod build-prod db-seed db-clear db-query fmt fmt-check lint check

DB_FILE := $(CURDIR)/local.db

env-init:
	cp -n .env.local.example .env.local || true
	cp -n .env.production.example .env.production || true

dev:
	@if [ -f .env.local ]; then set -a; . ./.env.local; set +a; fi; \
	: $${FLAT_NOTE_DB_URL:="file:$(DB_FILE)"}; \
	FLAT_NOTE_DB_URL="$$FLAT_NOTE_DB_URL" FLAT_NOTE_DB_AUTH_TOKEN="$$FLAT_NOTE_DB_AUTH_TOKEN" deno task tauri dev

dev-prod:
	@if [ ! -f .env.production ]; then echo "Missing .env.production"; exit 1; fi
	@set -a; . ./.env.production; set +a; deno task tauri dev

build-prod:
	@if [ ! -f .env.production ]; then echo "Missing .env.production"; exit 1; fi
	@set -a; . ./.env.production; set +a; deno task tauri build

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