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
- Turso CLI

## Commands

```bash
make dev        # Run for desktop (loads .env.local if present)
make dev-prod   # Run for desktop against production Turso (.env.production)
make build-prod # Build desktop app against production Turso (.env.production)
make fmt    # Format (Deno + Rust)
make lint   # Lint (Deno + Rust)
```

## Environment files

Create local env files from examples:

```bash
make env-init

# or manually
cp .env.local.example .env.local
cp .env.production.example .env.production
```

Then edit `.env.production` with your real Turso token.

## Build for macOS desktop

Run the Tauri production build from the project root:

```bash
deno task tauri build
```

Build outputs:

- app bundle: `src-tauri/target/release/bundle/macos/flat-note.app`
- dmg package: `src-tauri/target/release/bundle/dmg/flat-note_0.1.0_aarch64.dmg`

### Install dependencies

```bash
deno install
cargo add
```

## Database

### local development

- local SQLite database (`local.db`)

### production

- remote [Turso](https://app.turso.tech/katayama8000/databases/flat-note)
  database, accessed via HTTP API with auth token
