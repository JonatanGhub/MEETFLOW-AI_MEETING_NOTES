# Active hooks

Hooks live in `.claude/settings.json` under `hooks.PostToolUse`.

## PostToolUse — `Edit | Write` → prettier

Runs Prettier in `--write` mode on `.ts/.tsx/.js/.jsx/.json/.md/.css` files
right after Claude edits or writes them. Failures are silently swallowed
(`stdio: ignore`) — Prettier missing in `frontend/` shouldn't block edits.

**Why no `tsc --noEmit` hook?** It blocks every edit with 5–15s of compilation
in a project with 500+ files. Type-check is run on demand via `/check` slash
command and in CI on every PR.

## Future hooks (not yet active)

- `PreToolUse` matcher `Write|Edit` → reject writes to `frontend/src-tauri/target/`
  (build artifacts should never be edited manually).
- `Stop` → run `pnpm type-check` summary if there were Edit/Write in this turn.
