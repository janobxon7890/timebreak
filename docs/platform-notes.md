# Platform-Specific Implementation Notes

## macOS (Apple Silicon & Intel)
- **Window Architecture**:
  - Transparent NSWindow level set to `NSFloatingWindowLevel` (or `NSStatusWindowLevel`) so the overlay rests above all standard application windows.
  - Window collection behavior: `NSWindowCollectionBehaviorCanJoinAllSpaces | NSWindowCollectionBehaviorFullScreenAuxiliary`.
  - Transparent alpha buffer: NSWindow `backgroundColor = NSColor.clearColor`, `opaque = NO`, `hasShadow = NO`.
- **Input Capture**:
  - The overlay window captures direct mouse clicks and keyboard events while `Playing`.
  - Underlying applications (Telegram, IDEs, browsers) remain visually exposed but do not receive stray mouse clicks or keyboard inputs while TimeBreak is active.
  - `Escape` is bound directly as an emergency termination shortcut in both Rust and JS to instantly deactivate input capture and hide the overlay window.
- **Multi-Monitor Display Detection**:
  - Scans `NSScreen.screens` to extract logical coordinates, frame bounds, and `backingScaleFactor` (Retina 2.0x vs Standard 1.0x).

## Windows 10/11 Architecture (Design Blueprint)
- Borderless `WS_EX_LAYERED | WS_EX_TOPMOST` window.
- Direct2D / DirectComposition or transparent Webview2 composition.
- Windows Raw Input API (`RegisterRawInputDevices`) with `RIDEV_INPUTSINK`.

## Linux Architecture (Design Blueprint)
- X11: Composite extension ARGB visual with `_NET_WM_STATE_ABOVE`.
- Wayland: Layer Shell protocol (`zwlr_layer_shell_v1`) with overlay layer.
