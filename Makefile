.PHONY: dev fmt fmt-check lint check

dev:
	deno task tauri dev

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