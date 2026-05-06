# Changelog

All notable changes to MeetFlow will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial workspace scaffolding (Tauri v2 + Next.js 14 + TypeScript + Tailwind)
- `.claude/` configuration with custom slash commands (`/dev`, `/build`, `/test`, `/release`, `/design-check`, `/translate`)
- Project CLAUDE.md with stack decisions and code rules
- Privacy Policy and Terms & Conditions
- MIT License

### Decisions
- **2026-05-02**: Backend Python/FastAPI eliminated. All backend logic lives in Rust inside Tauri.
- **2026-05-02**: pyannote replaced by `sherpa-onnx` (via `sherpa-rs`) for VAD + speaker diarization.
- **2026-05-02**: Windows-only for v0.1. macOS support deferred to v0.3+.
- **2026-05-02**: MVP scope cut: v0.1 = recording + transcription + summary + notes + export. OAuth integrations move to v0.2.

## [0.1.0] - TBD

_Initial public release. Pending build._
