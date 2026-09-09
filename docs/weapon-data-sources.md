# CS2 Weapon Data Sources & Ballistics Reference

## 1. Provenance & Source Metadata
- **Source Engine / Game**: Counter-Strike 2 (CS2)
- **Engine Data Files**: `weapons.vdata`, `items_game.txt`, Valve GameTracking CS2 dump repository
- **Build Reference**: CS2 Release Version (2024–2026 Engine Standard)
- **Verification Classifications**:
  - `[VERIFIED]`: Directly extracted from `weapons.vdata` weapon keyvalues.
  - `[DERIVED]`: Calculated deterministically from verified formulas (e.g. cycle time from RPM, recovery time constants).
  - `[APPROXIMATED]`: Fine-tuned to match visual spray feel in 2D viewport projection.

## 2. Initial Supported Weapons & Verified Constants

| Weapon | Category | RPM | Magazine | Damage | Armor Pen | Inacc. Stand | Inacc. Move | Inacc. Air | Recoil Mag. |
|---|---|---|---|---|---|---|---|---|---|
| **AK-47** | Rifle | 600 | 30 | 36 | 77.5% | 4.20 | 175.0 | 210.0 | 30.0 |
| **M4A4** | Rifle | 666 | 30 | 33 | 70.0% | 3.60 | 140.0 | 185.0 | 27.0 |
| **M4A1-S** | Rifle | 600 | 20 | 38 | 70.0% | 3.30 | 125.0 | 175.0 | 23.0 |
| **AWP** | Sniper | 41 | 5 | 115 | 97.5% | 1.10 (scoped) | 350.0 | 450.0 | 80.0 |
| **Desert Eagle** | Pistol | 267 | 7 | 53 | 93.2% | 6.20 | 110.0 | 190.0 | 52.0 |
| **USP-S** | Pistol | 352 | 12 | 35 | 50.5% | 4.90 | 85.0 | 140.0 | 18.0 |
| **Glock-18** | Pistol | 400 | 20 | 30 | 47.0% | 7.60 | 95.0 | 150.0 | 19.0 |
| **MP9** | SMG | 857 | 30 | 26 | 60.0% | 7.80 | 80.0 | 130.0 | 22.0 |
| **MAC-10** | SMG | 800 | 30 | 29 | 57.5% | 8.90 | 75.0 | 125.0 | 24.0 |
| **Galil AR** | Rifle | 666 | 35 | 30 | 77.5% | 5.80 | 160.0 | 200.0 | 29.0 |

## 3. Recoil vs. Inaccuracy Separation
- **Recoil Path**: Deterministic table of `(dx, dy)` offsets indexed by shot count. As fire button is held, each subsequent bullet offsets the weapon muzzle according to the spray pattern.
- **Spread / Inaccuracy (Stochastic)**: Inaccuracy radius cone determined by player velocity and stance:
  $$\text{Total Inaccuracy} = \text{Base Spread} + \text{Stance Inaccuracy} + \left(\frac{\text{Current Velocity}}{\text{Max Speed}}\right) \times \text{Inacc. Move} + \text{Jump Penalty}$$
- **Final Shot Impact**:
  $$\text{FinalImpact} = \text{AimPoint} + \text{RecoilVector} + \text{RandomInCone}(\text{TotalInaccuracy})$$
