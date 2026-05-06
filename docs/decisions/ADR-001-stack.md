# ADR-001 — Stack base de MeetFlow v0.1

- **Estado**: Aceptado
- **Fecha**: 2026-05-02
- **Decisores**: Jonatan García (CEO) + CTO agent

## Contexto

El plan maestro original (MEETFLOW_MASTER_PLAN.md) proponía un stack con tres
runtimes: Rust (Tauri), Node.js (Next.js) y Python (FastAPI + pyannote.audio).
Esto introduce complejidad de packaging significativa en Windows (PyInstaller
como sidecar de Tauri, gestión de versiones Python, ~150 MB extra de instalador).

Adicionalmente, el plan incluía como MVP obligatorio 5 integraciones OAuth
+ AI Agent Executor + Analytics + Onboarding de 5 pasos, lo que estimamos en
25-30 días de desarrollo antes de tener algo demoable.

## Decisión

1. **Eliminar el backend Python/FastAPI.** Toda la lógica de servidor (DB, LLM
   clients, export, OAuth flows) vive en Rust dentro de `frontend/src-tauri/`.
2. **Sustituir pyannote.audio por sherpa-onnx** (vía crate `sherpa-rs`) para
   VAD y speaker diarization. Mantiene paridad funcional sin Python.
3. **Cortar MVP a v0.1**: recording + Whisper local + summary (Ollama/Claude) +
   meetings list + notes editor + export Markdown + onboarding 3 pasos.
4. **Windows-only para v0.1**, macOS pospuesto a v0.3.
5. **Mantener "lo mejor de WhisperX"** sin Python: whisper.cpp soporta
   word-level timestamps nativamente; combinar con sherpa-onnx para
   diarización en v0.2 da paridad funcional con WhisperX.

## Consecuencias

**Positivas:**
- Instalador único < 30 MB (sin runtime Python).
- Un solo proceso, sin IPC frágil entre Rust y FastAPI sidecar.
- CI más rápido (no hace falta `pip install` ni cachear venv).
- Demoable en 10-12 días (v0.1) en vez de 30+.
- Toda la criptografía y storage de tokens va por DPAPI (Windows) sin envoltorios Python.

**Negativas:**
- Curva de aprendizaje Rust mayor para contribuidores externos.
- Algunas librerías de NLP solo existen bien en Python (descartado en v0.1).
- Si en el futuro queremos añadir librerías Python específicas, habrá que
  rehacer la decisión o añadir un sidecar.

## Alternativas descartadas

- **Mantener FastAPI**: rechazado por coste de packaging y complejidad sin beneficio claro para el scope.
- **Reemplazar Tauri por Electron**: rechazado por tamaño de binario (5x mayor) y consumo de memoria.
- **Reemplazar Next.js por Vite + React puro**: viable pero next-intl y App Router aportan más en este caso.
- **WhisperX directamente**: requiere Python + pyannote, conflicto con decisión #1.

## Referencias

- [.claude/CLAUDE.md §2](./.claude/CLAUDE.md#2-stack-tcnico-fijado--no-cambiar-sin-hitl)
- [MEETFLOW_MASTER_PLAN.md §3](../../MEETFLOW_MASTER_PLAN.md)
- [whisper.cpp word timestamps](https://github.com/ggerganov/whisper.cpp/pull/1485)
- [sherpa-onnx speaker diarization](https://k2-fsa.github.io/sherpa/onnx/speaker-diarization/)
