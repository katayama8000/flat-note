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

## Commands

```bash
make dev    # Run for desktop (with Turso local DB)
make fmt    # Format (Deno + Rust)
make lint   # Lint (Deno + Rust)
```

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
