# MeetFlow — Project Status

_Updated: 2026-05-02_

## ⚡ SIGUIENTE ACCIÓN

**Bloqueado por toolchain.** Necesario antes de continuar:

1. **Instalar Rust toolchain** — `winget install Rustlang.Rustup` y luego `rustup default stable-msvc`
2. **Instalar Visual Studio Build Tools 2022** con workload "Desktop development with C++" (necesario para compilar whisper-rs y plugins Tauri en Windows)
3. **Instalar GitHub CLI** (opcional, recomendado): `winget install GitHub.cli` y `gh auth login`

Una vez instalados, la siguiente acción es:
- `cd frontend && pnpm create tauri-app .` (con flags para no interactivo) para inicializar Tauri v2 + Next.js.
- Aplicar el design system (paleta + tipografía) de `.claude/CLAUDE.md` §6.
- Setup i18n (next-intl) con archivos `messages/en.json` y `es.json`.

## Hitos completados

- [x] Repo clonado y plan maestro revisado
- [x] Decisiones de stack ajustadas (todo Rust, sin Python; sherpa-onnx en lugar de pyannote; Windows-only v0.1)
- [x] MVP cortado a v0.1 (recording + transcription + summary + notes + export)
- [x] Workspace estructura base creada
- [x] `.claude/` configurado con CLAUDE.md, settings.json y 6 slash commands
- [x] LICENSE (MIT), PRIVACY, TERMS, CHANGELOG, CONTRIBUTING, README escritos
- [x] `.gitignore` con todas las exclusiones (modelos, audio, secrets, build outputs)
- [x] pnpm instalado globalmente

## Hitos pendientes (orden)

1. Instalar Rust + MSVC Build Tools (BLOQUEADO — requiere usuario)
2. Init Tauri v2 + Next.js 14 en `frontend/`
3. Tailwind con design system ultra-dark
4. i18n next-intl (EN+ES)
5. Audio recording engine (cpal + WASAPI)
6. Whisper integration + model download manager
7. LLM clients (Ollama + Claude + OpenAI)
8. UI: sidebar, recording, meetings list, detail view
9. BlockNote editor + export Markdown
10. Onboarding mínimo (3 pasos)
11. GitHub Actions CI + release workflow
12. Tests (Vitest + cargo test + Playwright)
13. v0.1.0 release con NSIS installer

## Decisiones registradas

Ver [`docs/decisions/`](./decisions/) (vacío — primera ADR pendiente).
