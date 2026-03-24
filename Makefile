.PHONY: dev fmt lint check

dev:
	deno task tauri dev

fmt:
	deno fmt
	cd src-tauri && cargo fmt

lint:
	deno lint
	cd src-tauri && cargo clippy

check:
	deno check
	cd src-tauri && cargo check