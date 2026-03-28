## Commands

All commands are available via `make`. See [Makefile](Makefile) for details.

```bash
make dev    # Run for desktop
make fmt    # Format (Deno + Rust)
make lint   # Lint (Deno + Rust)
make check  # Type check (Deno + Rust)
make all    # Run all format, lint, and type check
```

## Architecture

### Frontend Structure (React + TanStack Router)

The frontend uses a feature-first structure under `src/`.

```text
src/
  assets/                         # Images, SVG, global static assets
  components/                     # App-wide reusable components
    ui/                           # UI primitives (shadcn/ui style location)
  features/                       # Domain/feature modules (most important)
    pages/
      api/                        # Tauri invoke wrappers for page operations
      components/                 # Feature-local components
      types/                      # Feature-local types
  hooks/                          # Shared hooks used by multiple features
  layouts/                        # App-level layout components
  pages/                          # Screen entry components
  routes/                         # File-based route definitions (thin wrappers)
  services/                       # Shared API/client setup
  store/                          # Global state management
  utils/                          # Shared utility functions
  App.tsx                         # Router setup and app root
```

#### Routing Rule

This project uses TanStack Router file-based routing. Keep route files in
`src/routes/`, but make them thin: each route should only define route metadata
and delegate UI/logic to `src/pages/` and `src/features/`.

#### Feature Rule

When adding new functionality, prefer placing domain-specific code under
`src/features/<feature-name>/` first, and only promote code to `components/`,
`hooks/`, `services/`, `store/`, or `utils/` when it is truly shared.

#### View and Logic Rule

Keep rendering and state/effects separate when a screen grows complex.

- Put UI-only markup in feature components such as
  `src/features/<feature-name>/components/*View.tsx`.
- Put state management, side effects, and event handling in hooks such as
  `src/features/<feature-name>/hooks/use*.ts`.
- Keep files under `src/pages/` as thin container entry points that connect
  hooks to view components.

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
- Prefer `const` + arrow functions for React components and hooks as well.
- Use `const` for variables that won't be reassigned and `let` for variables
  that will be reassigned.
- Use template literals for string concatenation.
- When importing components, use `import App from "./App.tsx"` instead of
  `import App from "./App"` to make it clear that it's a TypeScript file.
