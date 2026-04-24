# MeetFlow — AI Meeting Intelligence

> **Privacy-first desktop app for meeting capture, transcription and AI analysis.**
> Ultra-dark premium design · Local Whisper models · AI Agent Executor · 25+ integrations

---

## 🚀 Status: Pre-development

This repository contains the **master plan** for building MeetFlow from scratch.
The actual app development starts in a fresh session using this plan as the specification.

## 📋 Master Plan

Read [`MEETFLOW_MASTER_PLAN.md`](./MEETFLOW_MASTER_PLAN.md) — the complete specification document for building the app, including:

- Research phase (competitors, features, integrations)
- Tech stack decisions (Tauri v2 + Rust + Next.js 14 + FastAPI)
- Full design system (Ultra-dark premium, Linear.app inspired)
- Feature specifications (Recording, Transcription, AI Analysis, OAuth Integrations)
- AI Agent Executor — post-meeting task automation with Claude/GPT-4 tool use
- 25+ integrations roadmap (Google, Microsoft, Notion, Slack, Linear, n8n, HubSpot...)
- `.claude/` workspace setup (commands, hooks, settings)
- Development phases + testing strategy + GitHub CI/CD

## 🛠 Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop shell | Tauri v2 (Rust) |
| Frontend | Next.js 14 + TypeScript + shadcn/ui |
| Transcription | whisper.cpp (local, via whisper-rs) |
| Backend | FastAPI (Python 3.11) + SQLite |
| LLM | Ollama (local) / Claude / OpenAI / Groq / OpenRouter |
| Build | NSIS (.exe) + DMG |

## 🔐 Privacy

All audio and transcripts stay on your device. No data sent to the cloud unless you explicitly configure a cloud LLM provider.

## 📄 License

MIT — Open Source

---

*Generated plan: 2026-04-24*
