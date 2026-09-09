# Architecture Decision Record: macOS Transparent System-Wide Overlay

## Context
TimeBreak requires a 100% transparent, borderless, always-on-top window overlay across monitors that renders targets and crosshairs directly on top of the user's active desktop windows without intercepting desktop pixels, modifying underlying windows, or darkening/blurring backgrounds.

## Decision
We use Tauri 2 with a custom macOS native Cocoa hook. Specifically:
1. The overlay window is configured with transparent alpha:
   - `transparent: true` in `tauri.conf.json`
   - `decorations: false`
   - `always_on_top: true`
   - `skip_taskbar: true`
2. On macOS via `objc` / `cocoa` / `tauri::WebviewWindow`:
   - Set NSWindow `level` to `NSFloatingWindowLevel` (or `kCGStatusWindowLevelKey`).
   - Set `backgroundColor` to `[NSColor clearColor]`.
   - Set `isOpaque = NO`.
   - Set `hasShadow = NO`.
   - Set `collectionBehavior` to `[NSWindowCollectionBehaviorCanJoinAllSpaces, NSWindowCollectionBehaviorFullScreenAuxiliary]`.
3. An `OverlayBackend` trait is established in `crates/platform` so that native `wgpu` or platform-specific Windows (`WS_EX_LAYERED`) and Linux backends can be swapped without modifying game simulation or telemetry code.

## Consequences & Safety Fail-Safes
- Underlying windows (VS Code, Chrome, Telegram, Terminal) remain visible through the transparent webview.
- When gameplay activates, the overlay receives mouse clicks and WASD keystrokes so they do not type into Telegram or IDE code.
- Pressing `Escape` at any time immediately calls `hide()` and restores desktop control.
