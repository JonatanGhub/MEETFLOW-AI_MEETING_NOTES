---
description: Build de release con instalador NSIS (.exe)
---

```bash
cd frontend && pnpm tauri build
```

Output:
- `frontend/src-tauri/target/release/MeetFlow.exe` — binario standalone
- `frontend/src-tauri/target/release/bundle/nsis/MeetFlow_<version>_x64-setup.exe` — instalador

Validar antes de release:
1. Tamaño del .exe instalador < 30 MB (sin modelos Whisper)
2. Firma de código si hay certificado disponible
3. Ejecutar el .exe en VM limpia para verificar runtime deps
