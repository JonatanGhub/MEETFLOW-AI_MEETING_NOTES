---
description: Verifica que el código respeta el design system (paleta + tipografía)
---

Revisa que no haya colores hex hardcodeados ni fuentes fuera del sistema:

```bash
# Detectar hex colors en JSX/TSX (excepto en globals.css)
cd frontend/src && grep -rE "#[0-9a-fA-F]{3,8}" --include="*.tsx" --include="*.ts" | grep -v "globals.css"

# Detectar font-family hardcoded
cd frontend/src && grep -rE "font-family|fontFamily" --include="*.tsx" --include="*.ts"

# Detectar strings sin t() en JSX
cd frontend/src && grep -rnE ">[A-Z][a-zA-Z ]{3,}<" --include="*.tsx" | grep -v "{t("
```

Cualquier hit → abrir issue o corregir antes de commit.
Tokens válidos están en `frontend/src/styles/globals.css` (CSS vars) y en
`frontend/tailwind.config.ts` (tokens Tailwind).
