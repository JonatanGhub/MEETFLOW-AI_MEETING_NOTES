# MeetFlow — Project Status

_Updated: 2026-05-11_

## ⚡ SIGUIENTE ACCIÓN

**Probar flujo completo en `pnpm tauri dev`:**

```bash
cd frontend
pnpm tauri dev
```

Flujo a probar:
1. Onboarding → descargar modelo Whisper tiny/base
2. Grabar reunión de prueba (~20s)
3. Parar → ir a Meeting Detail
4. Verificar que transcripción arranca automáticamente y se muestra
5. Generar summary con Ollama (llama3.2) o Claude

Estado a 2026-05-11:
- `cargo check` → ✅ 0 errores (whisper-rs v0.16 + LLVM 22 + CMake 4.3)
- `pnpm type-check` → ✅ 0 errores
- `pnpm test` → ✅ 9/9 passing
- Commit: `feat(whisper): full local transcription with whisper-rs v0.16`

**Tras validar el flujo completo:**
Playwright e2e básico + v0.1.0 tag → GitHub Release

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
  - [x] `whisper/` — model catalog, download manager (SHA256), engine real (whisper-rs v0.16)
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

1. **`pnpm tauri dev` — validar flujo completo record → transcript → summary** ← SIGUIENTE
2. **Playwright e2e** config + tests básicos de onboarding + recording flow
3. **cargo test** — añadir unit tests para LLM summary parser + DB helpers
4. **v0.1.0 tag** → CI release build → NSIS installer en GitHub Releases

## Para v0.2

- Diarización sherpa-onnx (speaker labels en transcriptos)
- Google OAuth + Notion + Slack integrations
- AI Agent Executor (post-meeting actions via Claude)
- macOS DMG bundle

## Decisiones registradas

- `docs/decisions/ADR-001-stack.md` — stack decisions (no Python, sherpa-onnx, Windows-first)
