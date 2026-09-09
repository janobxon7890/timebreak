# TimeBreak Product Specification

## 1. Product Summary
TimeBreak brings professional CS2-style aim mechanics into micro-break intervals directly over the user's active desktop environment.

## 2. Core Functional Requirements
1. **Desktop Transparency**: Full alpha-channel transparency windowing. The user's active code editor, browser, terminal, or communication apps remain 100% visible beneath gameplay elements.
2. **Break Lifecycle & State Machine**:
   - `Idle`: Background monitor waiting for timer or hotkey.
   - `Scheduled`: Work interval countdown active.
   - `PreparingBreak`: Lead-time audio/visual alert.
   - `Countdown`: 3-second ready overlay before first target exposure.
   - `Playing`: Active target spawning, ballistics simulation, input capture.
   - `Paused`: Temporary session hold with pause screen.
   - `Results`: Session statistics, directional error charts, coaching recommendation.
   - `ReturningToWork`: Clean teardown of overlays, restore desktop focus.
3. **Multi-Monitor Gameplay**: Targets spawn across all active screens with respect to monitor bounds and DPI scaling.
4. **Input Capture & Safety**:
   - Game inputs (mouse clicks, WASD, Space, Ctrl, R) do not leak through to underlying desktop windows.
   - Immediate fail-safe: pressing `Escape` at any time instantly terminates active session and tears down overlays.
5. **CS2 Aim Profiling**:
   - DPI and in-game sensitivity calibration.
   - eDPI and cm/360 calculations.
   - Virtual view-angles and crosshair displacement.
6. **Telemetry & Personal Recommender**:
   - Stores every shot: weapon, timestamp, target hitbox, intended aim, recoil offset, spread offset, player velocity, and hit status.
   - Heuristic coaching engine diagnosing low/high aim biases, counter-strafe timing flaws, and spray pull-down deficiencies.
