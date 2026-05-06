---
description: Arranca MeetFlow en modo desarrollo (frontend + Tauri con HMR)
---

Ejecuta el entorno de desarrollo completo:

```bash
cd frontend && pnpm tauri dev
```

Notas:
- Tauri arranca Next.js dev server en `localhost:1420` y la ventana nativa.
- HMR activo para cambios en React/TypeScript.
- Cambios en Rust (`src-tauri/`) requieren rebuild automático (puede tardar ~10s).
- Logs Rust en stdout, logs frontend en DevTools (F12).
