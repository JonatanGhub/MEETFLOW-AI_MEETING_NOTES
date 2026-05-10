# MeetFlow — Project Status

_Updated: 2026-05-09_

## ⚡ SIGUIENTE ACCIÓN

**Primera ejecución real del dev server:**

```bash
cd frontend
pnpm tauri dev        # primera ejecución — abre la ventana Tauri
```

Estado del scaffold a 2026-05-09:
- `cargo check` → ✅ 0 errores (23 warnings, todos de stubs)
- `pnpm type-check` → ✅ 0 errores
- `pnpm test` → ✅ 9/9 passing

**Siguiente paso tras `pnpm tauri dev`:**
Integrar `whisper-rs` real en `src/whisper/engine.rs` (actualmente stub).

## Hitos completados

- [x] Repo clonado y plan maestro revisado
- [x] Stack decisions fijadas (todo Rust, sin Python; sherpa-onnx en lugar de pyannote)
- [x] MVP v0.1 scope cortado y aprobado
- [x] Workspace estructura base + `.claude/` settings + slash commands
- [x] LICENSE, PRIVACY, TERMS, CHANGELOG, CONTRIBUTING, README
- [x] `.gitignore` completo
- [x] pnpm instalado globalmente
- [x] Rust + MSVC Build Tools 2022 instalados (workload C++ desktop)
- [x] Init Tauri v2 + Next.js 14 en `frontend/`
- [x] Design system Tailwind ultra-dark (globals.css, tailwind.config.ts)
- [x] i18n next-intl client-side EN+ES
- [x] Zustand store (recordingState, onboardingComplete, sidebarCollapsed)
- [x] Rust backend completo:
  - [x] `error.rs` — MeetflowError enum + From impls
  - [x] `db/` — schema migrations + models
  - [x] `audio/` — capture (cpal WASAPI), pipeline (tokio), devices
  - [x] `whisper/` — model catalog, download manager (SHA256), engine stub
  - [x] `llm/` — providers, client (Ollama + Claude + OpenAI-compat), summary
  - [x] `commands/` — audio, meetings, settings, whisper, llm (25 commands)
  - [x] `lib.rs` — Tauri builder, DB init, state, invoke_handler
- [x] TypeScript wrappers (`src/lib/tauri.ts`) — tipos + wrappers para los 25 comandos
- [x] shadcn/ui components: button, badge, card, input, textarea, separator, progress, tooltip, dialog, tabs, scroll-area, select
- [x] Layout sidebar (collapsible, recording indicator, nav items)
- [x] Hooks: useRecording, useMeetings (TanStack Query)
- [x] Pages: onboarding (3 steps), record, meetings list, meeting detail, settings
- [x] GitHub Actions: ci.yml (Windows) + release.yml (tag-triggered NSIS)
- [x] Vitest config + test setup (Tauri mocks)
- [x] Placeholder Tauri icons generated (all 5: 32x32.png, 128x128.png, 128x128@2x.png, icon.png, icon.ico)
- [x] `cargo check` — 0 errors (fixed: generate_handler paths, From conflict, !Send pipeline, Manager import, setup ?)
- [x] `pnpm type-check` — 0 errors
- [x] `pnpm test` — 9/9 passing (fixed: truncate off-by-one)

## Hitos pendientes (v0.1)

1. **`pnpm tauri dev` — primera ejecución con ventana abierta** ← SIGUIENTE
2. **whisper-rs real** en `engine.rs` — actualmente stub
3. **Playwright e2e** config + tests básicos de onboarding + recording flow
4. **cargo test** — añadir unit tests para LLM summary parser + DB helpers
5. **v0.1.0 tag** → CI release build → NSIS installer en GitHub Releases

## Para v0.2

- Diarización sherpa-onnx (speaker labels en transcriptos)
- Google OAuth + Notion + Slack integrations
- AI Agent Executor (post-meeting actions via Claude)
- macOS DMG bundle

## Decisiones registradas

- `docs/decisions/ADR-001-stack.md` — stack decisions (no Python, sherpa-onnx, Windows-first)
