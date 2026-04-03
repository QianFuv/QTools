# Adding a New Tool

## File Structure

Each tool consists of a backend command module and a frontend tool module. The backend module can be either a single file or a directory, depending on complexity:

```
# Single-file module (simple tools)
src-tauri/src/commands/<name>.rs

# Directory module (complex tools)
src-tauri/src/commands/<name>/
  mod.rs                                Public command handlers (re-exports)
  ...                                   Internal submodules as needed

# Frontend (always a directory)
src/tools/<name>/index.tsx              ToolDefinition export
src/tools/<name>/<Name>Tool.tsx         Tool UI component
```

## Backend

### 1. Create the command module

For simple tools, create `src-tauri/src/commands/<name>.rs`.

For complex tools with multiple files, create a directory `src-tauri/src/commands/<name>/` with a `mod.rs` that re-exports the public command functions. Internal logic can be split into private submodules within the directory.

All command functions must:

- Be marked with `#[tauri::command]` and declared `pub`
- Return `Result<T, AppError>` — never use `.unwrap()` or `.expect()`
- Include doc comments with `# Arguments`, `# Returns`, and `# Errors` sections

### 2. Register the module

Add `pub mod <name>;` to `src-tauri/src/commands/mod.rs`. This works identically for both single-file and directory modules.

### 3. Register the handler

Add command paths to `generate_handler![]` in `src-tauri/src/lib.rs`.

### 4. Verify

```bash
cd src-tauri
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check --all-targets
cargo test
```

## Frontend

### 1. Create the tool component

Create `src/tools/<name>/<Name>Tool.tsx`. The component:

- Calls backend commands via `invoke<T>("command_name", { args })` from `@tauri-apps/api/core`
- Uses `qtools-*` color classes with `dark:` variants
- Pushes status updates via `useStatusBar()` when appropriate

### 2. Create the tool definition

Create `src/tools/<name>/index.tsx`, exporting a default `ToolDefinition` object.

The `ToolDefinition` interface (`src/types/tool.ts`):

| Field | Type | Description |
|-------|------|-------------|
| `id` | `string` | Unique identifier for tool switching |
| `name` | `string` | Display name in the sidebar |
| `icon` | `() => JSX.Element` | Sidebar icon (SVG element) |
| `component` | `Component` | Main UI component |

### 3. Register in the registry

Import and add the definition to the `tools` array in `src/tools/registry.ts`. Array order determines sidebar order.

### 4. Verify

```bash
pnpm exec tsc --noEmit
pnpm lint
```

## Available Context Hooks

| Hook | Source | Purpose |
|------|--------|---------|
| `useStatusBar()` | `contexts/StatusBarContext` | Push messages to the status bar (info / success / warning / error) |
| `useTheme()` | `contexts/ThemeContext` | Read current theme / toggle dark mode |
| `useActiveTool()` | `contexts/ToolContext` | Read active tool / tool list |

## Error Handling

All backend commands return `Result<T, AppError>`. The `AppError` enum is defined in `src-tauri/src/error.rs` and implements `serde::Serialize` for Tauri IPC. Add new variants as needed.

On the frontend, `invoke` rejects with the serialized error string. Use `try/catch` to handle it.
