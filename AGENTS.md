## Commands

### Install dependencies

```bash
deno install npm:dayjs
```

### Run for desktop

```bash
deno task tauri dev
```

### Build Rust (src-tauri)

```bash
cd src-tauri && cargo build
```

### Format Rust

```bash
cd src-tauri && cargo fmt
```

## lint

```bash
deno lint
```

## format

```bash
deno fmt
```

## Architecture

### Calling Rust from TypeScript (Tauri)

Use `invoke` from `@tauri-apps/api/core` to call Rust functions registered as
Tauri commands.

```ts
import { invoke } from "@tauri-apps/api/core";
const pages = await invoke<Page[]>("get_pages");
```

On the Rust side, register the command with `#[tauri::command]` and add it to
`generate_handler![]`.

### Rust Crate Structure

The Rust backend follows a layered DDD-inspired structure under `src-tauri/`:

```
src-tauri/
  Cargo.toml                        # workspace root
  src/
    lib.rs                          # Tauri commands (wiring only)
    crates/
      domain/src/lib.rs             # Models (e.g. Page) + repository traits
      usecase/src/lib.rs            # Business logic, depends on domain traits
      infrastructure/src/lib.rs     # Concrete implementations of repository traits
```

**Dependency direction:** `infrastructure` → `domain` ← `usecase` ← `lib.rs`

- **domain**: defines models and repository traits (no dependencies on other
  crates)
- **usecase**: implements business logic using repository traits (does not know
  concrete implementations)
- **infrastructure**: implements repository traits with actual data sources
- **lib.rs**: wires everything together and exposes Tauri commands

### Adding a new Tauri command

1. Add the model to `domain/src/lib.rs`
2. Add the repository trait to `domain/src/lib.rs`
3. Implement the use case in `usecase/src/lib.rs`
4. Implement the repository in `infrastructure/src/lib.rs`
5. Wire up in `src/lib.rs` and register in `generate_handler![]`

## Basic Rules

- Do not use Japanese in code comments or variable names. Use English instead.
- Use `camelCase` for variable and function names.
- Use `PascalCase` for class names and React components.
- Use arrow functions for anonymous functions and callbacks.
- Use `const` for variables that won't be reassigned and `let` for variables
  that will be reassigned.
- Use template literals for string concatenation.
- When importing components, use `import App from "./App.tsx"` instead of
  `import App from "./App"` to make it clear that it's a TypeScript file.
