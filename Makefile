.PHONY: dev fmt fmt-check lint check

dev:
	turso dev --db-file local.db & deno task tauri dev

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