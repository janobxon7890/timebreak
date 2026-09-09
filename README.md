# TIMEBREAK 🎯

**Counter-Strike 2 inspired micro-break aim trainer directly on top of your desktop.**

TimeBreak is a system-wide desktop application designed for developers and computer power users. Rather than leaving your IDE, browser, or active window during a Pomodoro or work break, TimeBreak renders a **100% transparent overlay** on top of your existing desktop. Target silhouettes peek, strafe, and move while your underlying code, terminals, and windows remain completely visible.

Shoot targets with authentic CS2 ballistics (spray recoil patterns, stance/velocity inaccuracy, counter-strafing, recovery times), and receive deep shot telemetry and directional bias analytics when your break ends.

---

## 🚀 Quick Start (Ubuntu / Debian Linux & macOS)

TimeBreak tizmga moslashuvchan (cross-platform). Loyiha `make dev` yoki `make setup` buyrug'i berilishi bilan operatsion tizimni (Ubuntu/Debian, Fedora, Arch yoki macOS) avtomatik aniqlaydi va kerakli kutubxonalarni o'zi o'rnatib ishga tushadi.

### Boshlash:

```bash
# 1. To'g'ridan-to'g'ri dasturni ishga tushirish (kamchiliklar bo'lsa avtomatik o'rnatadi):
make dev

# yoki maxsus 40 sekundlik tezkor mashg'ulot rejimi:
make dev-game

# 2. Tizim kutubxonalarini oldindan qo'lda sozlash (ixtiyoriy):
make setup

# 3. Testlarni ishga tushirish:
make test

# 4. Production release paketini yig'ish:
make build
```

---

## 🕹️ Controls & In-Game Hotkeys

| Key / Action | Function |
|---|---|
| **Mouse Aim** | Raw / relative crosshair aiming (CS2 sensitivity scale) |
| **Mouse Left** | Shoot weapon (semi/full auto, recoil progression) |
| **Mouse Right** | Zoom / scope toggle (AWP / scoped weapons) |
| **W / A / S / D** | Virtual movement & counter-strafing (affects spread) |
| **Left Ctrl** | Crouch stance (improves recoil recovery & spread) |
| **Space** | Jump (triggers airborne inaccuracy state) |
| **R** | Reload magazine |
| **Escape (Esc)** | **Emergency Exit / End Break** (immediately hides overlay and returns control to desktop) |

### Developer Quick-Keys (in `TIMEBREAK_DEV=1` mode)
- `F1` — Toggle Realtime Diagnostics HUD (FPS, stance, velocity, recoil index, spread angle)
- `F2` — Force spawn static practice target
- `F3` — Force spawn moving peek target
- `F4` — Force spawn head-only target
- `F8` — End session immediately and trigger analytics calculation

---

## 🏛️ Architecture

```text
timebreak/
├── apps/
│   └── desktop/            # Tauri 2 Desktop Shell (Rust + React/TS)
├── crates/
│   ├── core/               # State machine, player stance, timers, session loop
│   ├── ballistics/         # Recoil patterns, weapon manifests, CS2 spread physics
│   ├── analytics/          # Shot-by-shot telemetry, directional bias, coaching engine
│   ├── storage/            # SQLite schema migrations, offline persistent telemetry
│   └── platform/           # macOS transparent overlay, Cocoa NSWindow, display APIs
├── packages/
│   ├── shared-types/       # Common TypeScript types mirrored from Rust Serde schemas
│   ├── renderer/           # High-performance Canvas/WebGL target & HUD overlay renderer
│   └── ui/                 # React dashboard, telemetry charts, settings & i18n
├── assets/
│   ├── weapons/            # CS2 verified weapon profiles (AK-47, M4A4, AWP, etc.)
│   ├── targets/            # Neutral vector silhouette hitboxes
│   └── sounds/             # Low-latency synthesized sound assets
└── docs/                   # Architectural blueprints, analytics formulas, CS2 data sources
```

---

## 🛡️ Privacy & Security Boundaries
- **Zero background interception**: TimeBreak never takes screenshots of your desktop, never reads pixels behind the overlay, and never inspects window titles or contents.
- **Standalone Simulation**: TimeBreak is 100% standalone. It never injects code into Counter-Strike or any other application, never reads CS memory, and is completely offline.
