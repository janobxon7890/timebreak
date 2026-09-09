import {
  CrosshairConfig,
  PlayerStance,
  TargetEntity,
} from '@timebreak/shared-types';

export interface HitMarker {
  x: number;
  y: number;
  isHeadshot: boolean;
  spawnTimeMs: number;
}

export class OverlayRenderer {
  private ctx: CanvasRenderingContext2D;
  public debugMode: boolean = false;
  private hitMarkers: HitMarker[] = [];

  constructor(ctx: CanvasRenderingContext2D) {
    this.ctx = ctx;
  }

  public addHitMarker(x: number, y: number, isHeadshot: boolean) {
    this.hitMarkers.push({
      x,
      y,
      isHeadshot,
      spawnTimeMs: performance.now(),
    });
  }

  public clear(width: number, height: number) {
    // Crucial: 100% transparent clear. No background fills!
    this.ctx.clearRect(0, 0, width, height);
  }

  public renderTargets(targets: TargetEntity[], _nowMs: number) {
    for (const t of targets) {
      if (!t.isAlive) continue;

      // Draw silhouette practice target
      this.ctx.save();
      this.ctx.translate(t.x, t.y);

      if (t.visibleFraction !== undefined && t.visibleFraction < 1.0) {
        this.ctx.globalAlpha = Math.max(0.1, t.visibleFraction);
      }

      // Hitbox debug rendering if enabled
      if (this.debugMode) {
        for (const hb of t.hitboxes) {
          this.ctx.strokeStyle =
            hb.type === 'head' ? 'rgba(255, 50, 50, 0.9)' : 'rgba(50, 200, 255, 0.6)';
          this.ctx.lineWidth = 1;
          this.ctx.strokeRect(hb.x, hb.y, hb.width, hb.height);
        }
      }

      // Base silhouette body styling (CS-style clean training mannequin)
      this.drawTargetSilhouette(t.width, t.height, t.pose);

      this.ctx.restore();
    }
  }

  private drawTargetSilhouette(width: number, height: number, pose: string) {
    this.ctx.fillStyle = '#1e293b'; // Slate-800 silhouette
    this.ctx.strokeStyle = '#38bdf8'; // Sky-400 glowing outline for high visibility over dark/light IDEs
    this.ctx.lineWidth = 2;

    if (pose === 'head_only') {
      // Circle head
      const radius = width * 0.45;
      this.ctx.beginPath();
      this.ctx.arc(width * 0.5, height * 0.5, radius, 0, Math.PI * 2);
      this.ctx.fill();
      this.ctx.stroke();

      // Head visor indicator
      this.ctx.fillStyle = '#ef4444'; // Red target dot
      this.ctx.beginPath();
      this.ctx.arc(width * 0.5, height * 0.5, radius * 0.35, 0, Math.PI * 2);
      this.ctx.fill();
      return;
    }

    // Standard mannequin silhouette
    // 1. Head
    const headRadius = width * 0.22;
    const headCenterY = height * 0.12;
    this.ctx.beginPath();
    this.ctx.arc(width * 0.5, headCenterY, headRadius, 0, Math.PI * 2);
    this.ctx.fill();
    this.ctx.stroke();

    // Red head dot
    this.ctx.fillStyle = '#f43f5e';
    this.ctx.beginPath();
    this.ctx.arc(width * 0.5, headCenterY, headRadius * 0.3, 0, Math.PI * 2);
    this.ctx.fill();

    // 2. Torso & Arms
    this.ctx.fillStyle = '#0f172a';
    this.ctx.beginPath();
    // Rounded shoulders and chest
    this.ctx.roundRect(width * 0.1, height * 0.25, width * 0.8, height * 0.42, 6);
    this.ctx.fill();
    this.ctx.stroke();

    // 3. Legs (if standing/crouched)
    if (pose !== 'head_shoulder') {
      this.ctx.beginPath();
      this.ctx.roundRect(width * 0.15, height * 0.68, width * 0.3, height * 0.3, 4);
      this.ctx.roundRect(width * 0.55, height * 0.68, width * 0.3, height * 0.3, 4);
      this.ctx.fill();
      this.ctx.stroke();
    }
  }

  public renderCrosshair(x: number, y: number, config: CrosshairConfig, spreadRadius: number = 0) {
    this.ctx.save();
    this.ctx.translate(x, y);

    const size = config.size;
    const thickness = config.thickness;
    const gap = config.gap + (config.dynamicSpread ? spreadRadius * 0.5 : 0);

    this.ctx.strokeStyle = config.color;
    this.ctx.lineWidth = thickness;
    this.ctx.lineCap = 'square';

    if (config.outline) {
      this.ctx.shadowColor = 'rgba(0,0,0,0.8)';
      this.ctx.shadowBlur = 2;
    }

    // Top line
    this.ctx.beginPath();
    this.ctx.moveTo(0, -gap);
    this.ctx.lineTo(0, -(gap + size));
    this.ctx.stroke();

    // Bottom line
    this.ctx.beginPath();
    this.ctx.moveTo(0, gap);
    this.ctx.lineTo(0, gap + size);
    this.ctx.stroke();

    // Left line
    this.ctx.beginPath();
    this.ctx.moveTo(-gap, 0);
    this.ctx.lineTo(-(gap + size), 0);
    this.ctx.stroke();

    // Right line
    this.ctx.beginPath();
    this.ctx.moveTo(gap, 0);
    this.ctx.lineTo(gap + size, 0);
    this.ctx.stroke();

    // Center dot
    if (config.dot) {
      this.ctx.fillStyle = config.color;
      this.ctx.fillRect(-thickness * 0.5, -thickness * 0.5, thickness, thickness);
    }

    this.ctx.restore();
  }

  public renderHitMarkers(nowMs: number) {
    const aliveMarkers: HitMarker[] = [];

    for (const hm of this.hitMarkers) {
      const elapsed = nowMs - hm.spawnTimeMs;
      if (elapsed > 200) continue; // 200ms marker duration

      const alpha = 1.0 - elapsed / 200;
      this.ctx.save();
      this.ctx.translate(hm.x, hm.y);

      this.ctx.strokeStyle = hm.isHeadshot
        ? `rgba(239, 68, 68, ${alpha})`
        : `rgba(255, 255, 255, ${alpha})`;
      this.ctx.lineWidth = hm.isHeadshot ? 2.5 : 1.5;

      const size = hm.isHeadshot ? 8 : 5;
      this.ctx.beginPath();
      this.ctx.moveTo(-size, -size);
      this.ctx.lineTo(size, size);
      this.ctx.moveTo(size, -size);
      this.ctx.lineTo(-size, size);
      this.ctx.stroke();

      this.ctx.restore();
      aliveMarkers.push(hm);
    }

    this.hitMarkers = aliveMarkers;
  }

  public renderHUD(
    width: number,
    height: number,
    remainingSeconds: number,
    ammo: number,
    magazineSize: number,
    kills: number,
    accuracy: number,
    stance: PlayerStance,
    velocity: number
  ) {
    this.ctx.save();

    // Clean, minimalist developer HUD in bottom center / corners
    const mins = Math.floor(remainingSeconds / 60);
    const secs = Math.floor(remainingSeconds % 60);
    const timeStr = `${mins}:${secs < 10 ? '0' : ''}${secs}`;

    // Top timer pill
    this.ctx.fillStyle = 'rgba(15, 23, 42, 0.75)';
    this.ctx.beginPath();
    this.ctx.roundRect(width * 0.5 - 60, 20, 120, 36, 18);
    this.ctx.fill();
    this.ctx.strokeStyle = 'rgba(255, 255, 255, 0.15)';
    this.ctx.lineWidth = 1;
    this.ctx.stroke();

    this.ctx.fillStyle = '#f8fafc';
    this.ctx.font = '600 16px -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif';
    this.ctx.textAlign = 'center';
    this.ctx.textBaseline = 'middle';
    this.ctx.fillText(timeStr, width * 0.5, 38);

    // Bottom Right: Ammo and Stats
    const hudX = width - 180;
    const hudY = height - 70;
    this.ctx.fillStyle = 'rgba(15, 23, 42, 0.75)';
    this.ctx.beginPath();
    this.ctx.roundRect(hudX, hudY, 160, 50, 10);
    this.ctx.fill();
    this.ctx.strokeStyle = 'rgba(255, 255, 255, 0.1)';
    this.ctx.stroke();

    this.ctx.textAlign = 'left';
    this.ctx.fillStyle = '#38bdf8';
    this.ctx.font = '700 20px monospace';
    this.ctx.fillText(`${ammo}/${magazineSize}`, hudX + 16, hudY + 30);

    this.ctx.textAlign = 'right';
    this.ctx.fillStyle = '#94a3b8';
    this.ctx.font = '500 12px sans-serif';
    this.ctx.fillText(`Kills: ${kills}`, hudX + 144, hudY + 22);
    this.ctx.fillText(`Acc: ${accuracy.toFixed(0)}%`, hudX + 144, hudY + 38);

    // Bottom Left: Movement indicator
    const isStationary = velocity < 50;
    this.ctx.fillStyle = isStationary ? 'rgba(34, 197, 94, 0.2)' : 'rgba(239, 68, 68, 0.2)';
    this.ctx.beginPath();
    this.ctx.roundRect(20, height - 55, 140, 35, 8);
    this.ctx.fill();

    this.ctx.textAlign = 'center';
    this.ctx.font = '600 12px sans-serif';
    this.ctx.fillStyle = isStationary ? '#4ade80' : '#f87171';
    this.ctx.fillText(
      isStationary ? `● READY (${stance})` : `▲ MOVING (${velocity.toFixed(0)})`,
      90,
      height - 37
    );

    this.ctx.restore();
  }
}
