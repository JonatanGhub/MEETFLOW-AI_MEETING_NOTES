# MEETFLOW — Plan Maestro de Desarrollo desde Cero

> **Propósito de este documento:** Especificación completa y autocontenida para que un agente de IA construya MeetFlow desde cero en un workspace vacío, sin contexto previo. Incluye investigación, arquitectura, diseño, features, testing y distribución.

---

## 1. CONTEXTO & VISIÓN

### ¿Qué es MeetFlow?
MeetFlow es una aplicación de escritorio **privacy-first** para captura, transcripción y análisis inteligente de reuniones. Funciona 100% localmente — sin enviar audio ni texto a servidores de terceros — usando modelos de IA locales (Whisper para transcripción, Ollama/LLMs configurables para resúmenes).

### ¿Por qué se construye?
Las apps actuales (Otter.ai, Fireflies, Fathom, Meetily) o requieren conectividad cloud permanente (riesgo de privacidad) o son clones visuales entre sí. MeetFlow se diferencia en:
- **Identidad visual propia**: Ultra-dark premium (estética Linear.app/Vercel), no el gris genérico de Meetily
- **Super-app**: Onboarding, legal, ayuda, integraciones OAuth, todo de primera clase
- **Open Source**: GitHub público, instalador `.exe` / `.dmg` listo para no-técnicos

### Target de usuarios
Profesionales de empresa (ejecutivos, managers, consultores, sales reps) que hacen ≥5 reuniones/semana y quieren un registro privado, organizado y accionable de cada una.

### Modelo de negocio
**Open Source / Gratuito.** Repositorio público en GitHub. Sin licencias, sin suscripciones. Distribución mediante instaladores en GitHub Releases.

---

## 2. FASE 0 — INVESTIGACIÓN (el agente debe ejecutar esto primero)

Antes de escribir una sola línea de código, el agente debe investigar:

### 2.1 Análisis de competidores
Investigar con búsquedas web los siguientes competidores y documentar: features principales, precio, plataforma, diferenciador clave, puntos débiles (reviews negativas en Product Hunt, Reddit, G2, Capterra):

| App | URL |
|-----|-----|
| Otter.ai | otter.ai |
| Fireflies.ai | fireflies.ai |
| Fathom | fathom.video |
| tl;dv | tldv.io |
| Grain | grain.com |
| Avoma | avoma.com |
| MeetGeek | meetgeek.ai |
| Airgram | airgram.io |
| Sembly | sembly.ai |
| Read AI | read.ai |
| Meetily | github.com/meetily/meetily |
| Whisper-based local apps | GitHub search |

**Deliverable:** Tabla comparativa de features y matriz de posicionamiento.

### 2.2 Features que esperan los usuarios (investigar en Reddit/HN/reviews)
Buscar en: r/productivity, r/remotework, r/MacApps, r/windowsapps, Hacker News, Product Hunt comments.
Preguntas clave: ¿Qué falta en las apps actuales? ¿Qué dolor resuelven mal?

**Features más pedidas históricamente:**
- Speaker diarization (quién dijo qué)
- Action items automáticos con asignación de persona
- Integración nativa con calendario (ver reuniones antes de que ocurran)
- Exportar directamente a Notion/Slack/Jira
- Soporte multi-idioma en transcripción
- Tiempo real (ver transcripción mientras hablas, no después)
- Dashboard de productividad de reuniones
- Plantillas de reunión pre-configuradas
- Búsqueda full-text en todos los históricos
- Offline-first (sin internet)
- Modo de privacidad (desactivar recording en momentos sensibles)

### 2.3 Integraciones OAuth más valiosas (en orden de prioridad)
Investigar APIs y OAuth flows de:

**Tier 1 (MVP obligatorio):**
- Google OAuth 2.0 → acceso a Calendar, Meet (bot o import), Drive, Docs
- Microsoft OAuth 2.0 (MSAL) → acceso a Teams, Outlook Calendar, OneDrive, SharePoint

**Tier 2 (MVP deseable):**
- Notion OAuth → export de notas a páginas Notion
- Slack OAuth → post de resúmenes en canales

**Tier 3 (post-MVP):**
- Zoom OAuth (si hay integración futura de bot)
- Linear OAuth → crear issues desde action items
- Jira OAuth → crear tickets
- HubSpot OAuth → log de call en CRM
- GitHub OAuth → crear issues

Para cada integración documentar: URL de OAuth, scopes necesarios, rate limits, cómo registrar app en su developer console.

---

## 3. TECH STACK DECISIONS (fijadas, el agente no debe cambiarlas)

```
Frontend Desktop:  Tauri v2 (Rust) + Next.js 14 + TypeScript 5
UI Components:     shadcn/ui (Radix UI primitives) + Tailwind CSS 3.4
State:             React Context + Zustand (para estado global complejo)
Editor de notas:   BlockNote (editor de bloques tipo Notion)
HTTP/API:          Fetch + TanStack Query (react-query)
Forms:             React Hook Form + Zod
i18n:              next-intl (inglés + español desde el día 1)
Notificaciones:    Sonner (toasts) + Tauri plugin-notification
Íconos:            Lucide React
Animaciones:       Framer Motion (sparingly, para micro-interactions)

Backend:           FastAPI (Python 3.11) + SQLite + aiosqlite
LLM local:         Ollama (llama3.2, qwen2.5, mistral, phi3)
LLM cloud:         Claude API, OpenAI, Groq, OpenRouter (configurables)
Transcripción:     whisper-rs / whisper.cpp (integrado en Rust)
Audio:             cpal (captura) + WASAPI (Windows) + ScreenCaptureKit (macOS)
Speaker ID:        pyannote.audio (opcional, requiere token HuggingFace)

Package manager:   pnpm (frontend), pip/uv (backend), cargo (Rust)
Build:             Tauri CLI → NSIS installer (Windows) + DMG (macOS)
CI/CD:             GitHub Actions
Testing:           Vitest (unit) + Playwright (e2e) + pytest (backend)
```

**Importante:** El workspace DEBE tener la siguiente estructura desde el inicio:
```
meetflow/
├── .claude/                   # Configuración de Claude Code para el proyecto
│   ├── CLAUDE.md             # Instrucciones del proyecto para agentes IA
│   ├── settings.json         # Permisos, hooks, allowedTools
│   ├── settings.local.json   # Settings locales (gitignored)
│   ├── commands/             # Slash commands personalizados
│   │   ├── build.md          # /build → tauri build + empaquetado
│   │   ├── dev.md            # /dev → arranca frontend + backend + tauri
│   │   ├── test.md           # /test → vitest + pytest + cargo test
│   │   ├── release.md        # /release → bump version + tag + GitHub release
│   │   ├── translate.md      # /translate → sync claves i18n EN/ES
│   │   └── design-check.md   # /design-check → verifica colores y tipografía correctos
│   └── hooks/                # Descripciones de hooks (los hooks reales van en settings.json)
│       └── README.md         # Documenta qué hooks están activos y por qué
├── frontend/                  # Tauri app (Next.js + Rust)
│   ├── src/                  # React/TypeScript
│   ├── src-tauri/            # Rust backend
│   └── package.json
├── backend/                   # FastAPI server
│   ├── app/
│   └── requirements.txt
├── docs/                      # Documentación técnica
├── .github/                   # GitHub Actions workflows
├── README.md
├── PRIVACY.md
├── TERMS.md
├── CHANGELOG.md
├── CONTRIBUTING.md
├── LICENSE                    # MIT
└── Cargo.toml                 # Rust workspace root
```

### 3.1 Contenido de `.claude/settings.json` (base del proyecto)

```json
{
  "permissions": {
    "allow": [
      "Bash(pnpm *)",
      "Bash(cargo *)",
      "Bash(python *)",
      "Bash(git *)",
      "Bash(gh *)",
      "Bash(node *)"
    ],
    "deny": []
  },
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "cd frontend && pnpm exec prettier --write ${file} 2>/dev/null || true"
          }
        ]
      },
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "cd frontend && pnpm exec tsc --noEmit 2>&1 | head -20 || true"
          }
        ]
      }
    ]
  }
}
```

### 3.2 Contenido de `.claude/CLAUDE.md` (instrucciones del repo)

El agente debe crear este archivo con:
- Stack técnico del proyecto (copiado de este plan)
- Design system (paleta de colores, no cambiar)
- Reglas de código (TypeScript strict, funciones <50L, archivos <500L)
- Comandos de desarrollo (dev, build, test)
- Arquitectura de carpetas explicada
- Cómo añadir una nueva integración OAuth
- Cómo añadir una nueva clave i18n
- Instrucción de SIEMPRE usar `t('key')` para strings, NUNCA hardcoded

---

## 4. DISEÑO VISUAL — SISTEMA COMPLETO

### 4.1 Filosofía
**Ultra-dark premium**, inspirado en Linear.app y Vercel Dashboard. Principios:
- Oscuridad con profundidad (no negro plano sino capas de grises oscuros)
- Tipografía refinada con mucho peso en jerarquía
- Micro-animaciones sutiles (no flashy)
- Acentos de color usados con precisión (no everywhere)
- Glassmorphism mínimo solo para overlays flotantes

### 4.2 Paleta de colores (CSS Variables)

```css
/* Backgrounds */
--bg-base:        #09090B;   /* Fondo base de la app */
--bg-surface:     #111113;   /* Cards, paneles */
--bg-elevated:    #18181B;   /* Modales, dropdowns */
--bg-overlay:     #27272A;   /* Tooltips, menús */

/* Borders */
--border-subtle:  #1F1F23;   /* Divisores suaves */
--border-default: #27272A;   /* Bordes estándar */
--border-strong:  #3F3F46;   /* Bordes énfasis */

/* Text */
--text-primary:   #FAFAFA;   /* Texto principal */
--text-secondary: #A1A1AA;   /* Texto secundario/muted */
--text-tertiary:  #71717A;   /* Placeholders, hints */
--text-disabled:  #3F3F46;   /* Elementos desactivados */

/* Brand accent */
--accent:         #818CF8;   /* Indigo-400 — color de marca */
--accent-hover:   #6366F1;   /* Indigo-500 — hover */
--accent-subtle:  #1E1B4B;   /* Indigo background tenue */

/* Semantic */
--success:        #22C55E;
--success-subtle: #052E16;
--warning:        #F59E0B;
--warning-subtle: #1C1A00;
--error:          #EF4444;
--error-subtle:   #1C0A0A;

/* Recording indicator */
--recording:      #EF4444;   /* Rojo pulsante */
--recording-glow: rgba(239,68,68,0.3);

/* Gradient de marca */
--gradient-brand: linear-gradient(135deg, #6366F1 0%, #8B5CF6 50%, #A78BFA 100%);
```

### 4.3 Tipografía
```css
font-family: 'Geist', 'Inter', system-ui, -apple-system, sans-serif;

/* Jerarquía */
--text-xs:   0.75rem  / 1rem    /* 12px — captions, badges */
--text-sm:   0.875rem / 1.25rem /* 14px — body secundario */
--text-base: 1rem     / 1.5rem  /* 16px — body principal */
--text-lg:   1.125rem / 1.75rem /* 18px — subtítulos */
--text-xl:   1.25rem  / 1.75rem /* 20px — títulos sección */
--text-2xl:  1.5rem   / 2rem    /* 24px — títulos página */
--text-3xl:  1.875rem / 2.25rem /* 30px — hero texts */
```

### 4.4 Layout de la app
```
┌─────────────────────────────────────────────────────────────────┐
│  Sidebar (240px)        │  Main Content Area                    │
│  ─────────────────      │  ──────────────────────────────────── │
│  [Logo + App Name]      │  [Header: título + acciones]          │
│                         │                                        │
│  ● Recording (live)     │  [Content: varía por ruta]            │
│                         │                                        │
│  RECENT MEETINGS        │                                        │
│  · Meeting 1            │                                        │
│  · Meeting 2            │                                        │
│  · Meeting 3            │                                        │
│                         │                                        │
│  ───────────────        │                                        │
│  NAVIGATION             │                                        │
│  🎙 Record              │                                        │
│  📋 Meetings            │                                        │
│  📊 Analytics           │                                        │
│  🔗 Integrations        │                                        │
│  ⚙️  Settings            │                                        │
│  ❓ Help                │                                        │
│                         │                                        │
│  ───────────────        │                                        │
│  [User avatar + name]   │                                        │
└─────────────────────────┴────────────────────────────────────────┘
```

### 4.5 Componentes clave a construir
- `<RecordingButton>` — FAB pulsante con animación de onda cuando graba
- `<ModelCard>` — tarjeta de modelo descargable con progress bar
- `<MeetingCard>` — resumen compacto para lista de reuniones
- `<TranscriptPanel>` — panel de transcripción en tiempo real con speaker labels
- `<ActionItemChip>` — chip de action item con asignado + estado
- `<IntegrationCard>` — card de integración OAuth con estado conectado/desconectado
- `<OnboardingStep>` — contenedor de paso de onboarding con indicador de progreso

---

## 5. ARQUITECTURA DE FEATURES — ESPECIFICACIÓN COMPLETA

### 5.1 Sistema de Onboarding (nuevo, diferenciado)

El onboarding tiene **5 pasos** (no 4 como Meetily):

```
Step 1: WELCOME
  - Pantalla de bienvenida con logo animado (gradient animado)
  - Headline: "Your meetings. Your data. Your control."
  - 3 bullets: Privacy-first • AI-powered • Works offline
  - CTA: "Get started" (botón accent)

Step 2: PERMISSIONS
  - Solicitar permisos de micrófono + audio del sistema
  - macOS: explica que se necesita Screen Recording para audio de sistema
  - Windows: WASAPI loopback no necesita permiso especial
  - Test de audio en vivo (visualizador de nivel)

Step 3: CHOOSE YOUR MODEL (transcripción)
  - 4 opciones de modelo Whisper en cards:
    · Tiny (150MB)   — "Lightning fast, basic accuracy"
    · Small (500MB)  — "Great balance" [RECOMMENDED badge]
    · Medium (1.5GB) — "High accuracy"
    · Large (3GB)    — "Best quality, slower"
  - Cada card: nombre, tamaño, velocidad estimada, accuracy badge
  - Botón Download individual por card
  - Progress bar durante descarga
  - "Skip for now → Configure later in Settings"

Step 4: SUMMARY ENGINE
  El usuario elige cómo procesar los resúmenes. 3 modos:

  Modo A — Local Ollama:
    → Auto-detectar Ollama en localhost:11434
    → Si detectado: mostrar modelos disponibles para seleccionar (llama3.2, mistral, qwen2.5…)
    → Si NO detectado: preguntar al usuario:
        ① "I don't have Ollama" → botón "Install Ollama" (abre ollama.ai en browser)
        ② "I have Ollama on my network" → formulario para introducir:
              Host: [_____________] (ej: 192.168.1.50 o mi-servidor.local)
              Port: [11434]
              → Test connection button → listar modelos del servidor remoto
    → "100% private — no data leaves your network"

  Modo B — Cloud API (pay-per-use, usuario pone su clave):
    Provider selector con logos:
      · Anthropic Claude (claude-3-5-haiku recomendado por coste)
      · OpenAI (gpt-4o-mini recomendado)
      · Groq (gratuito con límites, muy rápido, llama3.1-8b)
      · OpenRouter (acceso a 200+ modelos con una sola API key)
      · Together AI (modelos open source en cloud barato)
      · Mistral AI (europeo, GDPR-friendly)
      · Perplexity API (si el usuario quiere RAG + web search)
    → Campo de API Key con show/hide + link "Get API key →"
    → Selector de modelo (dinámico según provider)
    → Test connection button (envía prompt de prueba)

  Modo C — Custom OpenAI-compatible endpoint:
    → Para quien tenga LM Studio, vLLM, LocalAI, Jan, Text Generation WebUI, etc.
    → Campos: Base URL + API Key (opcional) + Model name
    → Test connection button

  "Skip → Configure later in Settings > AI"

Step 5: INTEGRATIONS (nuevo paso exclusivo de MeetFlow)
  Grid de tarjetas agrupadas por categoría:

  COMMUNICATION & CALENDAR
    [Google]      Google Calendar + Meet + Drive + Docs
    [Microsoft]   Teams + Outlook Calendar + OneDrive
    [Slack]       Post summaries to channels
    [Discord]     Post summaries to servers (webhook, no OAuth)

  KNOWLEDGE & NOTES
    [Notion]      Export to Notion databases
    [Obsidian]    Export to local Obsidian vault (file path, no OAuth)
    [Logseq]      Export to Logseq graph (file path, no OAuth)

  PROJECT MANAGEMENT
    [Linear]      Create issues from action items
    [Jira]        Create tickets from action items
    [Asana]       Create tasks
    [ClickUp]     Create tasks
    [Trello]      Create cards
    [Monday]      Create items

  AI AGENTS (ver sección 5.8 — diferenciador clave)
    [Claude]      Send meeting to Claude → auto-execute action items
    [OpenAI]      Send to GPT-4 Agent → auto-create tasks + emails
    [n8n]         Trigger n8n workflow with meeting data
    [Make]        Trigger Make (Integromat) scenario
    [Zapier]      Trigger Zap with meeting payload

  CRM
    [HubSpot]     Log meeting + create follow-up tasks
    [Salesforce]  Log call activity + action items

  "Connect" en cada una → OAuth flow (o configuración de API key/URL)
  Todas opcionales. "Finish setup →" siempre disponible.
```

### 5.2 Recording Engine

```
Funcionalidad:
- Captura simultánea micrófono + audio del sistema
- Frecuencia: 48kHz, mono normalizado para Whisper
- Transcripción en tiempo real (streaming) mientras graba
- VAD (Voice Activity Detection) para filtrar silencios
- Pausa/resume con hotkey global (configurable)
- Indicador de recording en system tray
- Auto-guardar cada 30 segundos (crash recovery)
- Nombre de reunión: auto-detectado desde calendario o editable

Arquitectura Rust (src-tauri/src/audio/):
- devices/: enumeración de dispositivos por plataforma
- capture/: streams de micrófono y sistema
- pipeline.rs: mixing VAD-filtrado + recording path
- recording_manager.rs: orquestación
- recording_saver.rs: escritura WAV
```

### 5.3 Transcripción

```
Motor: whisper-rs (bindings Rust de whisper.cpp)
Modelos: ggml-tiny.bin, ggml-small.bin, ggml-medium.bin, ggml-large-v3-turbo.bin
Descarga: HuggingFace CDN (URLs estables, no CDN privado)
Storage: %APPDATA%\MeetFlow\models\ (Windows) / ~/Library/Application Support/MeetFlow/models/ (macOS)

Modos:
- Streaming: fragmentos de 30s → texto parcial mientras graba
- Batch: archivo completo post-recording → transcript final limpio
- Import: importar archivo de audio externo (MP3, WAV, M4A, MP4)

Multi-idioma: Whisper detecta idioma automáticamente o se fuerza en Settings
Speaker diarization: pyannote.audio (opcional, requiere HuggingFace token)
```

### 5.4 AI Analysis Engine

```
Post-transcripción, ejecutar (configurable cuáles activar):
1. Resumen ejecutivo (3-5 bullets)
2. Action items con persona asignada si se menciona
3. Temas principales (topic modeling)
4. Sentimiento general (positivo/neutral/negativo)
5. Meeting score (0-100 basado en duración, participantes, action items)
6. Key quotes (citas más importantes)
7. Next steps (decisiones tomadas)

Prompt system:
- Prompts templates editables en Settings > AI
- Variables: {transcript}, {meeting_title}, {date}, {duration}
- Soporte de prompts personalizados por tipo de reunión (1:1, standup, etc.)
```

### 5.5 Sistema de Integraciones OAuth

**Arquitectura:**
- Token storage: Tauri plugin-store (cifrado nativo)
- Refresh token automático en background
- Revoke desde Settings > Integrations

**Google Integration:**
```
OAuth scopes:
  - openid email profile
  - https://www.googleapis.com/auth/calendar.readonly
  - https://www.googleapis.com/auth/drive.file
  - https://www.googleapis.com/auth/documents

Features:
  - Listar próximas reuniones de Calendar → autocompletar nombre de reunión
  - Exportar notas/transcript a Google Doc
  - Guardar grabación en Google Drive
  - Detectar Google Meet en calendario → preparar para grabar

OAuth flow: PKCE (no client secret en desktop app)
Google Cloud Console: registrar app con redirect URI = meetflow://oauth/google
```

**Microsoft Integration:**
```
OAuth scopes (MSAL):
  - openid profile email
  - Calendars.Read
  - Files.ReadWrite
  - Chat.Read (Teams)

Features:
  - Listar reuniones de Outlook Calendar
  - Exportar a OneDrive
  - Ver Teams meetings → preparar contexto
  - Exportar a SharePoint (opcional)

OAuth flow: MSAL con redirect URI custom = meetflow://oauth/microsoft
```

**Notion Integration:**
```
OAuth: Notion OAuth (public integration)
Scopes: read_content, insert_content, update_content

Features:
  - Seleccionar database o página de destino
  - Exportar transcript + resumen como nueva página
  - Crear action items como items en Notion database

Setup: Registrar en https://www.notion.so/my-integrations
```

**Slack Integration:**
```
OAuth: Slack OAuth v2
Scopes: chat:write, channels:read, groups:read

Features:
  - Seleccionar canal de destino
  - Post de resumen post-reunión (configurable: auto o manual)
  - Formato: rich message con bullets de action items

Setup: Crear Slack App en api.slack.com/apps
```

**Integraciones Tier 2 (post-MVP, incluir UI placeholder "Coming soon"):**
- Zoom (bot de grabación)
- Linear (crear issues desde action items)
- Jira (crear tickets)
- HubSpot (log de reunión en CRM)
- GitHub (crear issues)
- ClickUp (crear tareas)

### 5.6 Meetings History & Organization

```
Vista lista:
  - Cards con: título, fecha, duración, score badge, primeros bullets del resumen
  - Filtros: por fecha, por duración, por score, por integración
  - Búsqueda full-text en transcript + notas + resúmenes

Vista detalle de reunión:
  - Header: título editable + metadata (fecha, duración, participantes detectados)
  - Tabs: Transcript | Notes | AI Summary | Action Items | Export

Notes editor:
  - BlockNote (editor de bloques tipo Notion)
  - Soporta: headings, bullets, checklists, code blocks, tables
  - Auto-guardado cada 2 segundos

Action Items:
  - Lista extraída por IA + capacidad de añadir manualmente
  - Checkboxes para marcar completados
  - Asignado a (nombre libre)
  - Due date opcional

Export:
  - Markdown (.md) — para Obsidian, etc.
  - PDF — para compartir
  - JSON — para integraciones custom
  - Directo a: Google Doc, Notion Page, Slack message
```

### 5.7 Analytics Dashboard

```
Métricas personales:
  - Horas totales en reuniones (semana/mes/total)
  - Meeting score promedio (trending)
  - Action items creados vs completados
  - Temas más frecuentes (word cloud o lista)
  - Horas pico de reuniones (heatmap)
  - Top participantes (si se detectan con diarización)

Gráficos:
  - Línea: meeting score en el tiempo
  - Barras: horas/semana en reuniones
  - Dona: tipos de reunión (standup, 1:1, all-hands...)
  - Tabla: action items pendientes más antiguos
```

### 5.8 AI AGENT EXECUTOR — Diferenciador clave de MeetFlow

**Concepto:** Tras cada reunión, el usuario puede activar un "AI Agent" que recibe todo
el contexto de la reunión (transcript, resumen, action items, decisiones, participantes)
y ejecuta acciones automáticas usando las integraciones conectadas. Como un asistente
que "lee las notas de la reunión y hace las tareas por ti".

**¿Por qué es viable?**
- Claude y GPT-4 tienen tool use / function calling maduro
- MeetFlow ya tiene los tokens OAuth guardados de cada integración
- Ningún competidor hace esto desde una app local — diferenciador real
- El usuario mantiene el control con previews antes de ejecutar

**Flujo de usuario:**
```
1. Reunión terminada → panel "Action Items"
2. Botón "🤖 Execute with AI Agent"
3. Modal: seleccionar agente (Claude / GPT-4 / n8n webhook / Make)
4. Checkboxes de permisos al agente:
   ☑ Create Linear issues for each action item
   ☑ Draft follow-up email to participants (DRAFT, never auto-send)
   ☑ Create Notion page with structured notes
   ☑ Schedule follow-up meeting in Calendar
   ☑ Post Slack summary
   ☐ Search web for context on topics discussed
5. "Run Agent →"
6. Log en tiempo real: "Creating Linear issue 'Feature X mockups'... ✓"
7. Resumen: "3 issues created, 1 email drafted, 1 Notion page created"
8. Botón "Undo" disponible 5 minutos después de ejecutar
```

**Implementación técnica (backend/app/ai_agent_executor.py):**
```python
def execute_post_meeting_agent(meeting_data, enabled_tools, provider="claude"):
    """
    Envía el contexto de la reunión al LLM con tool use.
    El LLM tiene acceso a las herramientas de las integraciones conectadas.
    """
    # Construir tools disponibles según integraciones activas del usuario
    tools = build_tools_from_integrations(enabled_tools, meeting_data.connected_integrations)
    
    system_prompt = """You are a post-meeting assistant with access to connected services.
    Based on the meeting transcript and action items, execute the agreed tasks.
    SAFETY RULES:
    - Never send emails automatically, always create drafts
    - Show a preview before any irreversible action
    - Only do what was explicitly agreed in the meeting"""
    
    context = f"""Meeting: {meeting_data.title} | {meeting_data.date}
Participants: {', '.join(meeting_data.participants)}
Summary: {meeting_data.summary}
Action Items: {format_action_items(meeting_data.action_items)}
Decisions: {format_decisions(meeting_data.decisions)}"""
    
    # Ejecutar en bucle tool use hasta completar (max 10 iteraciones)
    return run_agent_loop(provider, system_prompt, context, tools, max_iterations=10)
```

**Tools disponibles para el agente (según integraciones):**
```
create_linear_issue(title, description, assignee, priority)
create_jira_ticket(project, summary, description, assignee)
create_notion_page(parent_id, title, content_blocks)
create_calendar_event(title, date, attendees, description)
send_slack_message(channel, message, blocks)
create_email_draft(to, subject, body)    # DRAFT en Gmail/Outlook, nunca envío
create_hubspot_note(contact, content)
create_github_issue(repo, title, body, labels)
trigger_webhook(url, payload)            # n8n / Make / Zapier
```

**Para n8n / Make / Zapier — payload estándar:**
```json
{
  "meeting": {
    "id": "uuid", "title": "Q1 Planning", "date": "2026-04-24",
    "duration_minutes": 45, "participants": ["Ana López", "Carlos Ruiz"]
  },
  "summary": "Se decidió lanzar feature X en mayo...",
  "action_items": [
    {"text": "Ana crea mockups para feature X", "assignee": "Ana López", "due": "2026-04-30"}
  ],
  "decisions": ["Lanzar en mayo", "Budget aprobado"]
}
```

### 5.9 Integrations — Tabla completa con prioridades

| Categoría | Integración | Auth | MVP | v1.1 | v1.2 |
|---|---|---|:---:|:---:|:---:|
| **Productivity** | Google (Calendar+Meet+Drive+Docs) | OAuth PKCE | ✓ | | |
| | Microsoft (Teams+Outlook+OneDrive) | MSAL | ✓ | | |
| | Notion | OAuth | ✓ | | |
| | Obsidian | File path | | ✓ | |
| | Logseq | File path | | ✓ | |
| **Communication** | Slack | OAuth | ✓ | | |
| | Discord | Webhook URL | | ✓ | |
| | Email (SMTP) | User+Pass | | ✓ | |
| **Project Mgmt** | Linear | OAuth | ✓ | | |
| | Jira | OAuth | | ✓ | |
| | GitHub | OAuth | | ✓ | |
| | Asana | OAuth | | ✓ | |
| | ClickUp | API Key | | | ✓ |
| | Trello | OAuth | | | ✓ |
| | Monday.com | API Key | | | ✓ |
| **AI Agents** | Claude (Anthropic) | API Key | ✓ | | |
| | OpenAI GPT-4 | API Key | ✓ | | |
| | n8n | Webhook URL | ✓ | | |
| | Make (Integromat) | Webhook URL | | ✓ | |
| | Zapier | Webhook URL | | ✓ | |
| **CRM** | HubSpot | OAuth | | ✓ | |
| | Salesforce | OAuth | | | ✓ |
| | Pipedrive | API Key | | | ✓ |
| **Scheduling** | Calendly | API Key | | | ✓ |
| | Cal.com | API Key | | | ✓ |

### 5.10 Settings completas

```
Tabs en Settings:
  1. General
     - Idioma (EN/ES)
     - Tema (dark/light/system) — dark por defecto
     - Hotkey para start/stop recording
     - Startup behavior
     - Auto-update check

  2. Audio
     - Selección de micrófono
     - Selección de fuente de audio del sistema
     - Test de audio en vivo
     - Nivel de ganancia

  3. Transcription
     - Modelo Whisper activo (selector + descargar nuevos)
     - Idioma de transcripción (auto-detect o forzar)
     - Qualidad de transcripción (speed vs accuracy)
     - GPU acceleration on/off

  4. AI / Summary
     Sección A — Summary Engine:
     - Provider: Ollama local / Ollama red local (host:port) / Claude / OpenAI /
                 Groq / OpenRouter / Together AI / Mistral AI / Custom endpoint
     - Modelo (selector dinámico según provider, cargado desde API del provider)
     - API Key (encrypted con Tauri store)
     - URL custom (para Ollama red + endpoints OpenAI-compatible)
     - Test connection button
     - Max tokens / Temperatura / Top-P
     - Edición de prompts base (resumen, action items, temas, sentimiento)
     
     Sección B — AI Agent Executor:
     - Activar/desactivar el executor post-reunión
     - Provider del agente (puede ser diferente al de resúmenes)
     - Max iteraciones de tool use (defecto: 10)
     - Log de últimas ejecuciones del agente

  5. Integrations
     - Estado de cada integración (conectado/desconectado)
     - Botones Connect/Disconnect
     - Configuración específica por integración

  6. Privacy
     - Datos guardados localmente: ruta, tamaño
     - Exportar todos mis datos (ZIP)
     - Borrar todos los datos
     - No enviar analytics (opt-in, por defecto off)

  7. About
     - Versión, changelog
     - Licencia Open Source
     - Links: GitHub, Report a bug, Documentation
```

### 5.9 In-app Documentation System

```
Acceso: botón "?" en sidebar o Help menu

Secciones:
  1. Getting Started
     - Quick start guide (3 pasos: instalar → grabar → ver resumen)
     - Video embed o GIF animado de demo
  
  2. How it Works
     - Diagrama de privacidad (datos NUNCA salen del dispositivo)
     - Flujo: Audio → Whisper local → LLM local/cloud → SQLite
     - GPU acceleration explicada
  
  3. Transcription Models
     - Comparativa de modelos (tabla: tamaño, velocidad, accuracy)
     - Cuándo usar cada uno
     - Cómo descargar / cambiar
  
  4. Integrations Guide
     - Setup paso a paso de cada integración (con screenshots)
  
  5. Privacy & Security
     - Link a Privacy Policy completa
     - Explicación técnica de dónde van los datos
     - Cómo revocar accesos OAuth
  
  6. Keyboard Shortcuts
     - Lista completa de shortcuts
  
  7. FAQ
     - Preguntas frecuentes generadas desde reviews de competidores
  
  8. Terms & Conditions
     - Link a T&C completos
```

### 5.10 Legal Pages (obligatorias)

Crear como páginas dentro de la app Y como archivos en el repo:

**Privacy Policy** debe cubrir:
- Qué datos se procesan (audio, transcripts, notas)
- Dónde se almacenan (solo localmente, en %APPDATA%)
- Si se usa API cloud (Claude, OpenAI): qué datos se envían y bajo qué T&C del proveedor
- Integraciones OAuth: qué permisos se piden y para qué se usan exactamente
- Datos de analytics (si se activan): PostHog auto-hosted o desactivado por defecto
- Cómo borrar todos tus datos
- Contacto

**Terms & Conditions** debe cubrir:
- Licencia MIT (open source)
- Responsabilidad del usuario por cumplir leyes locales de grabación de reuniones
- Disclaimer: "Recording laws vary by jurisdiction. Ensure you have consent from all participants."
- Limitación de responsabilidad
- Servicio sin garantía (as-is)

---

## 6. SISTEMA DE MODELOS DESCARGABLES

### 6.1 Catálogo de modelos Whisper

```rust
// Definir en config.rs
pub const WHISPER_MODEL_CATALOG: &[ModelEntry] = &[
    ModelEntry {
        id: "tiny",
        display_name: "Tiny",
        description: "Fastest, basic accuracy. Good for quick notes.",
        size_mb: 77,
        accuracy: "Basic",
        speed: "Lightning fast",
        badge: None,
        hf_url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin",
    },
    ModelEntry {
        id: "small-q5_1",
        display_name: "Small",
        description: "Good balance of speed and quality.",
        size_mb: 181,
        accuracy: "Good",
        speed: "Fast",
        badge: Some("Recommended"),
        hf_url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small-q5_1.bin",
    },
    ModelEntry {
        id: "medium-q5_0",
        display_name: "Medium",
        description: "High accuracy, moderate speed.",
        size_mb: 514,
        accuracy: "High",
        speed: "Medium",
        badge: None,
        hf_url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium-q5_0.bin",
    },
    ModelEntry {
        id: "large-v3-turbo-q5_0",
        display_name: "Large v3 Turbo",
        description: "Best accuracy, optimized for speed.",
        size_mb: 547,
        accuracy: "Best",
        speed: "Medium",
        badge: Some("Best Value"),
        hf_url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v3-turbo-q5_0.bin",
    },
];
```

### 6.2 Download Manager

- Solo una descarga activa a la vez
- Events Tauri: `model-download-progress` {modelId, percent}, `model-download-complete` {modelId}, `model-download-error` {modelId, error}
- Resume en caso de fallo (si el servidor lo soporta, con Range headers)
- Verificación de checksum SHA256 post-descarga
- Cancelación de descarga activa

---

## 7. GITHUB SETUP

### 7.1 Estructura del repositorio

```
meetflow/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml            # Tests en PR
│   │   ├── release.yml       # Build + publish release en tag
│   │   └── security.yml      # Audit de dependencias semanal
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   └── PULL_REQUEST_TEMPLATE.md
├── frontend/
├── backend/
├── docs/
├── CLAUDE.md                 # Instrucciones para agentes IA en este repo
├── README.md                 # Con GIF de demo, instalación, features
├── LICENSE                   # MIT
├── PRIVACY.md                # Privacy Policy
├── TERMS.md                  # Terms & Conditions
├── CHANGELOG.md              # Historial de versiones
└── CONTRIBUTING.md           # Guía para contribuidores
```

### 7.2 CI/CD (GitHub Actions)

**`ci.yml`** — se ejecuta en cada PR:
```yaml
jobs:
  test-frontend:
    - pnpm install
    - pnpm run lint
    - pnpm run type-check
    - pnpm run test (vitest)
  
  test-backend:
    - pip install -r requirements.txt
    - pytest tests/ --cov=app --cov-report=xml
  
  build-check:
    - cargo check
    - cargo clippy -- -D warnings
```

**`release.yml`** — se ejecuta en tag `v*`:
```yaml
strategy:
  matrix:
    platform: [windows-latest, macos-latest]

steps:
  - Checkout
  - Setup Rust, Node, Python
  - Build frontend (pnpm build)
  - Build Tauri (tauri build)
  - Upload artifacts:
    · Windows: MeetFlow_x.x.x_x64-setup.exe (NSIS installer)
    · macOS: MeetFlow_x.x.x_x64.dmg
  - Create GitHub Release with installers attached
  - Update latest release notes from CHANGELOG.md
```

### 7.3 README.md (estructura)

```markdown
# MeetFlow

[Logo + Tagline]
[Demo GIF - 10 segundos mostrando grabación y resumen]

## Features
[Lista con iconos de features principales]

## Privacy
[Diagrama simple: tu mic → MeetFlow → tu disco. Nada en la nube.]

## Quick Install
### Windows
[Descargar .exe desde GitHub Releases]
### macOS
[Descargar .dmg desde GitHub Releases]

## Requirements
[CPU mínimo, RAM, espacio en disco]

## Integrations
[Grid de logos de integraciones soportadas]

## License
MIT — Open Source
```

---

## 8. FASES DE DESARROLLO

### FASE 1 — Investigación (1-2 días)
- [ ] Análisis de competidores (tabla comparativa)
- [ ] Research de features más pedidas (Reddit, HN, G2)
- [ ] Documentar OAuth flows de cada integración
- [ ] Definir diferenciadores claros frente a Meetily/Otter/Fireflies

### FASE 2 — Setup base (1 día)
- [ ] Crear workspace `meetflow/` con estructura completa
- [ ] Init Tauri v2 + Next.js 14 + shadcn/ui
- [ ] Configurar Tailwind con el design system definido (paleta, tipografía)
- [ ] Setup i18n (next-intl) con mensajes EN + ES
- [ ] Init FastAPI backend con SQLite
- [ ] Configurar GitHub repo con Actions
- [ ] Crear CLAUDE.md del repo

### FASE 3 — Core Recording (3-4 días)
- [ ] Audio device enumeration (Windows + macOS)
- [ ] Captura de micrófono + sistema (pipeline)
- [ ] Integración Whisper (modelo small por defecto)
- [ ] Download manager de modelos (con progress events)
- [ ] Transcripción streaming en tiempo real
- [ ] Auto-guardado de meetings en SQLite

### FASE 4 — UI principal (3-4 días)
- [ ] Design system components (shadcn customizados con dark theme)
- [ ] Sidebar + layout principal
- [ ] Recording interface (FAB + transcript panel)
- [ ] Meetings list con cards
- [ ] Meeting detail view (tabs: Transcript, Notes, AI, Actions)
- [ ] BlockNote editor para notas
- [ ] Settings completas (8 tabs)

### FASE 5 — Onboarding (2 días)
- [ ] 5 pasos de onboarding con nueva identidad visual
- [ ] Selector de modelos Whisper (con descarga)
- [ ] Selector de Summary Engine
- [ ] Pantalla de integraciones (step 5)

### FASE 6 — AI Analysis (2 días)
- [ ] Sistema de prompts configurable
- [ ] Integración Ollama (detect + model list)
- [ ] Integración APIs cloud (Claude, OpenAI, Groq)
- [ ] Extracción de: resumen, action items, temas, sentimiento, score

### FASE 7 — Integraciones OAuth (3-4 días)
- [ ] Google OAuth flow (PKCE, deep link: `meetflow://oauth/google`)
- [ ] Microsoft OAuth flow (MSAL, deep link: `meetflow://oauth/microsoft`)
- [ ] Notion OAuth
- [ ] Slack OAuth
- [ ] Linear OAuth
- [ ] UI de gestión de integraciones en Settings (tabla con estado + connect/disconnect)
- [ ] Export: Google Doc, Notion page, Slack message, Markdown, PDF, JSON

### FASE 7B — AI Agent Executor (2-3 días)
- [ ] `ai_agent_executor.py` con tool use loop (Claude + OpenAI)
- [ ] Definición de tools para cada integración conectada
- [ ] UI: modal de configuración del agente + checkboxes de permisos
- [ ] Log en tiempo real de acciones del agente (Tauri events)
- [ ] Webhook executor (n8n, Make, Zapier) con payload estándar
- [ ] Botón Undo (5 minutos)
- [ ] Tests de integración del executor con mocks de APIs

### FASE 8 — Analytics + Legal + Help (2 días)
- [ ] Dashboard de analytics con gráficos
- [ ] Privacy Policy y Terms & Conditions (en app + archivos)
- [ ] Sistema de documentación in-app (5 secciones)
- [ ] FAQ basado en pain points investigados

### FASE 9 — Testing + Polish (2-3 días)
- [ ] Tests unitarios (Vitest) — coverage >80%
- [ ] Tests e2e críticos (Playwright): onboarding, recording, export
- [ ] Tests backend (pytest) — coverage >80%
- [ ] Performance: app startup <3s, recording latency <500ms
- [ ] Accessibility: WCAG 2.1 AA mínimo

### FASE 10 — Distribution (1 día)
- [ ] Build Windows NSIS installer (.exe) via GitHub Actions
- [ ] Build macOS DMG via GitHub Actions
- [ ] Primera release (v0.1.0) en GitHub con changelogs
- [ ] README con GIF demo

---

## 9. TESTING STRATEGY

### Frontend (Vitest + Playwright)
```
Unit tests (Vitest):
  - src/utils/*.test.ts — todas las funciones utilitarias
  - src/hooks/*.test.ts — custom hooks
  - src/components/*.test.tsx — componentes críticos

E2E tests (Playwright):
  - tests/onboarding.spec.ts — flujo completo de onboarding
  - tests/recording.spec.ts — start/stop/pause recording
  - tests/meetings.spec.ts — crear/ver/borrar meeting
  - tests/settings.spec.ts — cambiar provider AI, modelo Whisper
  - tests/export.spec.ts — exportar a markdown/PDF

Coverage target: >80% en unit tests
```

### Backend (pytest)
```
tests/
  - test_meetings.py — CRUD de meetings
  - test_transcripts.py — guardar y recuperar transcripts
  - test_summarization.py — generación de resumen (mock LLM)
  - test_export.py — export a distintos formatos

Coverage target: >80%
```

### Rust (cargo test)
```
tests:
  - whisper_engine/tests — load model, transcribe sample
  - audio/tests — device enumeration mock
  - models/tests — download manager mock

Integration tests:
  - Record 10s of audio → transcript matches expected
```

---

## 10. PERFORMANCE TARGETS

```
Startup time:            < 3 segundos (app lista para grabar)
Recording start:         < 500ms desde pulsar botón
Transcript lag:          < 2 segundos detrás del audio (modelo small)
AI summary generation:   < 15 segundos (Ollama local llama3.2)
UI frame rate:           60fps constante
Memory footprint:        < 200MB idle, < 500MB recording con modelo small
```

---

## 11. REGLAS PARA EL AGENTE QUE IMPLEMENTE ESTO

1. **Leer este documento completo antes de escribir código**
2. Empezar siempre por la Fase 0 (investigación) — buscar en web competidores antes de diseñar
3. Respetar el design system definido (colores, tipografía) al 100% — no improvisar
4. Usar SIEMPRE TypeScript strict, nunca `any`
5. Funciones < 50 líneas, archivos < 500 líneas
6. Tests primero (TDD) para toda lógica de negocio
7. i18n desde el día 1 — nunca strings hardcodeados en el JSX, siempre `t('key')`
8. Nunca hardcodear API keys — siempre variables de entorno o Tauri store cifrado
9. Los modelos Whisper se descargan SOLO desde HuggingFace (URLs estables) — nunca desde CDN privado
10. OAuth flows SIEMPRE con PKCE (no client secret en app de escritorio)
11. Privacy first: ningún audio, transcript o nota sale del dispositivo sin acción explícita del usuario
12. Documentar cada Tauri command con JSDoc en el lado TypeScript
13. Cada feature debe tener su ruta de Settings correspondiente

---

## 12. ARCHIVOS CLAVE DE REFERENCIA PARA EL AGENTE

El agente puede consultar el codebase anterior de PrivFlow-AInotes como referencia técnica (NO copiar el diseño visual):

```
REUTILIZAR la lógica de:
  - Audio pipeline (Rust) — pipeline.rs es un sistema probado
  - Whisper engine — whisper_engine.rs + commands.rs
  - Recording manager — recording_manager.rs
  - FastAPI backend — estructura de db.py y main.py
  - Tauri command pattern — lib.rs como referencia

NO REUTILIZAR:
  - Design system visual (colores, componentes)
  - Onboarding flow
  - Branding/logo
  - Nombre del producto
  - Identifier del bundle (debe ser com.meetflow.app)
```

---

*Plan generado el 2026-04-24 para sesión nueva en workspace vacío.*
*Stack base: Tauri v2 + Rust + Next.js 14 + FastAPI Python*
