# Contributing to MeetFlow

Thanks for your interest. MeetFlow is open source and contributions are welcome.

## Quick start

```bash
git clone https://github.com/JonatanGhub/MEETFLOW-AI_MEETING_NOTES.git
cd MEETFLOW-PRO_AI_NOTE_TAKER/frontend
pnpm install
pnpm tauri dev
```

Requirements:
- Node 20+ (we use 22.x)
- pnpm 9+
- Rust stable (`rustup default stable`)
- Visual Studio Build Tools 2022 with **Desktop development with C++** workload (Windows)
- WebView2 (pre-installed on Windows 11)

## Development workflow

1. Fork the repo and create a feature branch from `main`.
2. Follow the code style in [.claude/CLAUDE.md](./.claude/CLAUDE.md):
   - TypeScript strict, no `any`
   - Rust: `cargo clippy -- -D warnings` clean
   - Functions < 50 lines, files < 500 lines
   - All UI strings via `t('key')` (i18n)
3. Write tests for new logic (Vitest for TS, `cargo test` for Rust).
4. Run `pnpm lint`, `pnpm type-check`, and `cargo clippy` before committing.
5. Commit messages: [Conventional Commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `chore:`…).
6. Open a PR with a clear description and link any related issues.

## Adding an integration

See [docs/playbooks/adding-integration.md](./docs/playbooks/adding-integration.md) (TBD).

## Adding a translation key

See [docs/playbooks/i18n.md](./docs/playbooks/i18n.md) (TBD). Always update both
`en.json` and `es.json` in the same commit.

## Code of conduct

Be respectful. No harassment, no personal attacks. We follow the
[Contributor Covenant](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).
