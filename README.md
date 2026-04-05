# flat-note

A desktop note-taking app built with Tauri, React, and TypeScript.

## Overview

**flat-note** is a lightweight, fast, and feature-rich note-taking application
designed for users who want a simple yet powerful way to organize their thoughts
and ideas. Built with modern web technologies and Tauri, it provides a native
desktop experience with cloud synchronization capabilities.

## Features

- **Rich Text Editing** - Create notes with markdown support, tables, YouTube
  embeds, code blocks with syntax highlighting, and task lists
- **Full-Text Search** - Quickly find notes across your entire library
- **Auto-Save** - Your work is automatically saved every 1.5 seconds, no need to
  manually save
- **Task Management** - Create nested task lists to stay organized
- **Code Highlights** - Support for syntax highlighting in multiple programming
  languages
- **Tables** - Create and edit tables with resizable columns
- **Cross-Platform** - Native desktop app for macOS (with support for
  Windows/Linux)
- **Cloud Sync** - Optional cloud storage via Turso database for syncing across
  devices
- **Keyboard Shortcuts** - Cmd/Ctrl+S to save, fast navigation between notes

## How It Works

flat-note uses a modern architecture:

- **Frontend**: React with TanStack Router for routing, Tiptap for rich text
  editing via Markdown
- **Backend**: Rust with Tauri for native desktop integration
- **Database**: SQLite for local development, Turso (SQLite-compatible) for
  cloud storage
- **File Structure**: Feature-first organization with clear separation between
  domain logic and UI

All your notes are stored locally or in the cloud, and the app automatically
saves changes as you type.

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
