# TimeBreak Telemetry & Analytics Formulations

## 1. Raw Shot Telemetry Schema
Each shot fired during gameplay registers a discrete telemetry record:
- $t_{shot}$: High-resolution monotonic timestamp (ms)
- $(x_{aim}, y_{aim})$: Screen coordinates of the crosshair at trigger time
- $(x_{target}, y_{target})$: Center of target hitbox
- $(x_{head}, y_{head})$: Center of head hitbox
- $(e_x, e_y) = (x_{aim} - x_{target}, y_{aim} - y_{target})$: Aim error vector
- $(r_x, r_y)$: Recoil displacement vector
- $(s_x, s_y)$: Seeded stochastic spread vector
- $(x_{final}, y_{final}) = (x_{aim} + r_x + s_x, y_{aim} + r_y + s_y)$: Final projectile hit coordinates
- $v_{player}$: Virtual player speed at time of shot ($px/s$)
- $\text{Stance}$: `Standing`, `Crouched`, `Airborne`, `CounterStrafing`
- $\text{HitResult}$: `None`, `Head`, `Chest`, `Stomach`, `Arms`, `Legs`

## 2. Statistical Metrics & Mathematical Formulations

### Accuracy & Headshot Rate
$$\text{Overall Accuracy} = \frac{\text{Hits}}{\text{Shots Fired}} \times 100\%$$
$$\text{Headshot \%} = \frac{\text{Head Hits}}{\text{Total Hits}} \times 100\%$$

### Reaction Time Metrics
For target $i$ spawned at $t_{spawn, i}$ and first shot at $t_{shot, i}$:
$$\text{Reaction Time}_i = t_{shot, i} - t_{spawn, i}$$
- **Median Reaction Time**: $\tilde{R} = \text{median}(\{R_1, \dots, R_n\})$
- **P90 Reaction Time**: 90th percentile to ignore outliers

### Signed Aim Error & Directional Bias
$$\bar{e}_x = \frac{1}{N}\sum_{i=1}^N e_{x, i}, \quad \bar{e}_y = \frac{1}{N}\sum_{i=1}^N e_{y, i}$$
$$\text{Radial Error}_i = \sqrt{e_{x, i}^2 + e_{y, i}^2}$$
$$\text{RMS Error} = \sqrt{\frac{1}{N}\sum_{i=1}^N (e_{x, i}^2 + e_{y, i}^2)}$$

**Directional Classification**:
- High: $\bar{e}_y < -\tau$ (in inverted screen Y)
- Low: $\bar{e}_y > \tau$
- Left: $\bar{e}_x < -\tau$
- Right: $\bar{e}_x > \tau$

### Movement Discipline Score
$$\text{Movement Score} = 100 \times \left(1 - \frac{\text{Shots with } v_{player} > 34\% \cdot v_{max}}{\text{Total Shots}}\right)$$
Identifies whether player fires before counter-strafing to a complete stop.

### Spray Control Quality
Given optimal compensation vector $\vec{C}_{opt, k} = -\vec{R}_k$ for shot $k$ in a spray:
$$\text{Spray Score} = \max\left(0, 100 - \frac{1}{K}\sum_{k=1}^K \|\vec{M}_k - \vec{C}_{opt, k}\|\right)$$
where $\vec{M}_k$ is the user's mouse pull-down displacement.
