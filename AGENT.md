## Commands

### Install dependencies

```bash
deno install npm:dayjs
```

### Run for desktop

```bash
deno task tauri dev
```

## lint

```bash
deno lint
```

## format

```bash
deno fmt
```

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
