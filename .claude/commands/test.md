---
description: Ejecuta toda la suite de tests (Vitest + cargo test + Playwright)
---

Tests en orden de velocidad:

```bash
# 1. Unit tests TS (Vitest) — segundos
cd frontend && pnpm test --run

# 2. Unit tests Rust (cargo) — segundos
cd frontend/src-tauri && cargo test

# 3. Lint + type-check
cd frontend && pnpm lint && pnpm type-check
cd frontend/src-tauri && cargo clippy -- -D warnings

# 4. E2E (Playwright) — minutos, opcional en dev
cd frontend && pnpm test:e2e
```

Coverage targets: >80% Vitest unit, >80% cargo test módulos core.
