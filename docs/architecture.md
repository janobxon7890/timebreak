# TimeBreak Architectural Blueprint

## 1. System Topology

TimeBreak is structured as a modular desktop application decoupling OS windowing, hardware input, simulation math, telemetry persistence, and visual rendering:

```
┌─────────────────────────────────────────────────────────────┐
│                    Operating System                         │
│   (macOS Cocoa / Windows Win32 / Linux Wayland/X11)         │
└──────────────┬──────────────────────────────┬───────────────┘
               │                              │
               ▼                              ▼
     ┌───────────────────┐          ┌───────────────────┐
     │  Platform Window  │          │   Raw Input Hooks │
     │  (NSWindow/Cocoa) │          │  (CGEvent/Quartz) │
     └─────────┬─────────┘          └─────────┬─────────┘
               │                              │
               ▼                              ▼
     ┌──────────────────────────────────────────────────┐
     │               Tauri 2 Core Host                  │
     │      (IPC Commands, Event Bus, Lifecycle)        │
     └─────────┬──────────────────────────────┬─────────┘
               │                              │
               ▼                              ▼
     ┌───────────────────┐          ┌───────────────────┐
     │  crates/core      │          │ crates/ballistics │
     │  • State Machine  │◄────────►│ • Weapon Manifest │
     │  • Player Stance  │          │ • Recoil Path     │
     │  • Target Spawner │          │ • Spread Inaccur. │
     └─────────┬─────────┘          └─────────┬─────────┘
               │                              │
               ▼                              ▼
     ┌───────────────────┐          ┌───────────────────┐
     │ crates/analytics  │          │  crates/storage   │
     │ • Telemetry Proc. │◄────────►│  • SQLite Migrat. │
     │ • Bias & RMS Stat │          │  • Offline Sync   │
     │ • Recommender     │          │                   │
     └───────────────────┘          └───────────────────┘
               ▲                              ▲
               └──────────────┬───────────────┘
                              │
               ┌──────────────┴───────────────┐
               │         Frontend UI          │
               │  • packages/renderer (Canvas)│
               │  • packages/ui (React/Zustand│
               │  • Settings, Results, i18n   │
               └──────────────────────────────┘
```

## 2. Layer Separation Guarantees
1. **Core Simulation Independence**: `crates/core` and `crates/ballistics` have ZERO dependencies on Tauri, Webview, or React. They are pure Rust crates with deterministic state transitions, enabling 100% golden tests without UI rendering.
2. **Platform Abstraction**: All platform specifics (NSWindow transparent level, Quartz mouse cursor capture, multi-monitor geometry) are encapsulated in `crates/platform` under the `OverlayBackend` and `RawInputProvider` traits.
3. **Telemetry Pipeline**: Each shot fired emits a strongly-typed `ShotTelemetryEvent`. Analytics are calculated purely from stored shot telemetry data in `crates/analytics`, never fabricated in UI.
4. **Storage Model**: SQLite via `rusqlite` with schema migrations in `crates/storage`. Batched async writes ensure simulation frames remain at 60-240 FPS without I/O blocking.

## 3. Coordinate Spaces
To ensure pixel-perfect hit registration across Retina, 4K, and mixed scale factor displays, TimeBreak isolates three distinct coordinate models:
- **Game Coordinate Space (Virtual Units)**: Normalized 1920x1080 baseline with angular FOV representation.
- **Logical Desktop Pixels**: OS coordinate space for window positioning and multi-monitor layout.
- **Physical Device Pixels**: Backing buffer resolution for HiDPI crisp canvas rendering.
