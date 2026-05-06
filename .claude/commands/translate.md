---
description: Sincroniza claves i18n entre EN y ES detectando faltantes
---

Compara `frontend/src/messages/en.json` vs `es.json` y reporta keys que faltan
en alguno de los dos archivos. Para cada key faltante:
1. Si falta en ES → traducir desde EN
2. Si falta en EN → marcar como bug (EN es el idioma fuente de verdad)
3. Las claves jerárquicas se comparan recursivamente

Output esperado: lista de keys + sugerencias de traducción + escribir el JSON corregido.
