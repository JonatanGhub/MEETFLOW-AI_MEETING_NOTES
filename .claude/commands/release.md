---
description: Bump versión, tag y disparar GitHub release
---

Pasos (HITL — confirmar cada uno):

1. **Verificar estado limpio**
   ```bash
   git status     # debe estar clean
   git checkout main && git pull
   ```

2. **Bump versión** en estos 3 archivos (deben coincidir):
   - `frontend/package.json` → `version`
   - `frontend/src-tauri/Cargo.toml` → `version`
   - `frontend/src-tauri/tauri.conf.json` → `version`

3. **Actualizar CHANGELOG.md** con sección nueva (formato Keep a Changelog).

4. **Commit + tag**:
   ```bash
   git add -A
   git commit -m "chore(release): vX.Y.Z"
   git tag -s vX.Y.Z -m "Release vX.Y.Z"
   git push origin main --tags
   ```

5. GitHub Action `release.yml` se dispara con el tag → build + crea Release con installers.

6. **Validación post-release**:
   - Descargar installer del Release page
   - Instalar en máquina limpia (idealmente VM)
   - Smoke test: onboarding → grabar 30s → ver transcript
