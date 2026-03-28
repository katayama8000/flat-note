# flat-note

A desktop note-taking app built with Tauri, React, and TypeScript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) +
  [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) +
  [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Requirements

- Rust
  - clippy (for linting)
- Deno
- Turso CLI (if using Turso for local development)
  - sqld (if using `--db-file` with Turso)

## Commands

```bash
make dev    # Run for desktop (with Turso local DB)
make fmt    # Format (Deno + Rust)
make lint   # Lint (Deno + Rust)
```

### Install dependencies

```bash
deno install
cargo add
```
