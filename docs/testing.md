# TimeBreak Testing Strategy & Golden Suites

## 1. Test Layers

### Rust Unit & Golden Suites
- `crates/ballistics`:
  - Recoil determinism: verify exact pattern offsets for 30-round AK-47 spray.
  - Spread calculation: verify stance penalties (Standing < Crouched < Moving < Airborne).
  - Recovery time: verify accuracy recovery rate after firing stops.
- `crates/core`:
  - Player state machine transitions (Walking -> CounterStrafing -> Stopped).
  - Target hit testing against multi-part hierarchical hitboxes (head, chest, limbs).
  - Deterministic session replay given matching PRNG seed.
- `crates/analytics`:
  - Signed aim error validation against synthetic test sets (e.g. +20px right bias).
  - Synthetic low-head placement detecting recommendation.
  - Movement error scoring.

### Automated Frontend & Renderer Tests
- Canvas overlay rendering loop at 60/120 FPS.
- Coordinate transformation between Retina physical pixels and logical game coordinates.
- Escape hotkey immediate deactivation test.

## 2. Running Test Suites
```bash
make test
```
Executes both Rust workspace tests and webview/TypeScript suites.
