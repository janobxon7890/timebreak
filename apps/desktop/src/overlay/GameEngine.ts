import {
  CrosshairConfig,
  CS2Profile,
  HitboxType,
  PlayerStance,
  ShotTelemetryEvent,
  TargetEntity,
  TargetPose,
  WeaponDefinition,
} from '@timebreak/shared-types';
import { OverlayRenderer, sound } from '@timebreak/renderer';

export interface GameEngineCallbacks {
  onSessionFinish: (shots: ShotTelemetryEvent[], durationSeconds: number) => void;
  onEscape: () => void;
}

export class GameEngine {
  private canvas: HTMLCanvasElement;
  private renderer: OverlayRenderer;
  private callbacks: GameEngineCallbacks;

  // Session State
  public sessionId: string = '';
  public startTimeMs: number = 0;
  public sessionDurationSeconds: number = 300; // 5 minutes default
  public isRunning: boolean = false;

  // Weapon & Profile
  public weapon: WeaponDefinition;
  public profile: CS2Profile;
  public crosshairConfig: CrosshairConfig;

  // Ammo & Recoil
  public currentAmmo: number = 30;
  public isReloading: boolean = false;
  private lastShotTimeMs: number = 0;
  private shotIndex: number = 0;
  private currentRecoilX: number = 0;
  private currentRecoilY: number = 0;

  // Virtual Player Physics
  public playerX: number = 0;
  public playerY: number = 0;
  public playerVx: number = 0;
  public playerVy: number = 0;
  public stance: PlayerStance = 'Standing';
  public isCrouching: boolean = false;
  public isAirborne: boolean = false;
  private keysHeld: Record<string, boolean> = {};

  // Aim Crosshair Position (exact mouse/touchpad cursor coordinate)
  public crosshairX: number = 0;
  public crosshairY: number = 0;

  // Screen & DPI Helpers
  public get logicalWidth(): number {
    return this.canvas.clientWidth || window.innerWidth;
  }

  public get logicalHeight(): number {
    return this.canvas.clientHeight || window.innerHeight;
  }

  public get dpr(): number {
    return window.devicePixelRatio || 1;
  }

  // Targets & Telemetry
  public targets: TargetEntity[] = [];
  public shots: ShotTelemetryEvent[] = [];
  public kills: number = 0;
  private lastSpawnTimeMs: number = 0;
  private targetCounter: number = 0;

  // Animation frame
  private animFrameId: number = 0;
  private lastFrameTimeMs: number = 0;

  constructor(
    canvas: HTMLCanvasElement,
    weapon: WeaponDefinition,
    profile: CS2Profile,
    crosshairConfig: CrosshairConfig,
    callbacks: GameEngineCallbacks
  ) {
    this.canvas = canvas;
    const ctx = canvas.getContext('2d', { alpha: true });
    if (!ctx) throw new Error('Failed to obtain 2d canvas context');
    this.renderer = new OverlayRenderer(ctx);
    this.weapon = weapon;
    this.profile = profile;
    this.crosshairConfig = crosshairConfig;
    this.callbacks = callbacks;
    this.currentAmmo = weapon.magazineSize;
  }

  public start(durationSeconds: number = 300) {
    this.sessionId = `tb_${Date.now()}`;
    this.startTimeMs = performance.now();
    this.lastFrameTimeMs = this.startTimeMs;
    this.sessionDurationSeconds = durationSeconds;
    this.isRunning = true;
    this.shots = [];
    this.targets = [];
    this.kills = 0;
    this.currentAmmo = this.weapon.magazineSize;
    this.shotIndex = 0;

    // Center crosshair initially
    this.crosshairX = this.logicalWidth / 2;
    this.crosshairY = this.logicalHeight / 2;

    this.bindEvents();
    this.loop();
  }

  public stop() {
    this.isRunning = false;
    cancelAnimationFrame(this.animFrameId);
    this.unbindEvents();
    this.renderer.clear(this.logicalWidth, this.logicalHeight);
  }

  private bindEvents() {
    window.addEventListener('keydown', this.handleKeyDown);
    window.addEventListener('keyup', this.handleKeyUp);
    window.addEventListener('mousemove', this.handleMouseMove);
    window.addEventListener('mousedown', this.handleMouseDown);
  }

  private unbindEvents() {
    window.removeEventListener('keydown', this.handleKeyDown);
    window.removeEventListener('keyup', this.handleKeyUp);
    window.removeEventListener('mousemove', this.handleMouseMove);
    window.removeEventListener('mousedown', this.handleMouseDown);
  }

  private handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === 'Escape') {
      e.preventDefault();
      this.stop();
      this.callbacks.onEscape();
      return;
    }

    if (e.key === 'r' || e.key === 'R') {
      this.reload();
      return;
    }

    if (e.key === 'F1') {
      this.renderer.debugMode = !this.renderer.debugMode;
      return;
    }

    if (e.key === 'F8') {
      this.finishSession();
      return;
    }

    this.keysHeld[e.key.toLowerCase()] = true;
  };

  private handleKeyUp = (e: KeyboardEvent) => {
    this.keysHeld[e.key.toLowerCase()] = false;
  };

  private handleMouseMove = (e: MouseEvent) => {
    // Exact 1:1 synchronization with touchpad/mouse cursor
    const rect = this.canvas.getBoundingClientRect();
    this.crosshairX = e.clientX - rect.left;
    this.crosshairY = e.clientY - rect.top;
  };

  private handleMouseDown = (e: MouseEvent) => {
    if (e.button === 0) {
      this.shoot();
    }
  };

  private reload() {
    if (this.isReloading || this.currentAmmo >= this.weapon.magazineSize) return;
    this.isReloading = true;
    sound.playReload();
    setTimeout(() => {
      this.currentAmmo = this.weapon.magazineSize;
      this.isReloading = false;
    }, this.weapon.reloadTimeMs);
  }

  private shoot() {
    const now = performance.now();
    if (now - this.lastShotTimeMs < this.weapon.cycleTimeMs) return;
    if (this.isReloading) return;

    if (this.currentAmmo <= 0) {
      sound.playEmptyClick();
      return;
    }

    this.currentAmmo -= 1;
    this.lastShotTimeMs = now;

    // Recoil advance
    const pattern = this.weapon.recoilPattern;
    if (pattern.length > 0) {
      const idx = Math.min(this.shotIndex, pattern.length - 1);
      this.currentRecoilX = pattern[idx].dx;
      this.currentRecoilY = pattern[idx].dy;
      this.shotIndex += 1;
    }

    // Spread calculation
    const speed = Math.sqrt(this.playerVx * this.playerVx + this.playerVy * this.playerVy);
    let baseInacc = this.isCrouching ? this.weapon.inaccuracyCrouch : this.weapon.inaccuracyStand;
    if (this.isAirborne) baseInacc = this.weapon.inaccuracyAir;

    const movePenalty = speed > 50 ? (speed / this.weapon.maxPlayerSpeed) * this.weapon.inaccuracyMove : 0;
    const totalInacc = this.weapon.baseSpread + baseInacc + movePenalty;

    // Sample random angle in disk
    const r = totalInacc * Math.sqrt(Math.random());
    const theta = Math.random() * 2 * Math.PI;
    const spreadX = r * Math.cos(theta);
    const spreadY = r * Math.sin(theta);

    const finalShotX = this.crosshairX + this.currentRecoilX + spreadX;
    const finalShotY = this.crosshairY + this.currentRecoilY + spreadY;

    // Sound
    sound.playGunshot(this.weapon.id.includes('silencer'));

    // Check hits against active targets
    let hit = false;
    let headshot = false;
    let hitHitbox: HitboxType | undefined;
    let hitTarget: TargetEntity | undefined;

    for (const t of this.targets) {
      if (!t.isAlive) continue;

      for (const hb of t.hitboxes) {
        const absX = t.x + hb.x;
        const absY = t.y + hb.y;
        if (
          finalShotX >= absX &&
          finalShotX <= absX + hb.width &&
          finalShotY >= absY &&
          finalShotY <= absY + hb.height
        ) {
          hit = true;
          hitHitbox = hb.type;
          hitTarget = t;
          if (hb.type === 'head') {
            headshot = true;
          }
          break;
        }
      }
      if (hit) break;
    }

    if (hit && hitTarget) {
      if (headshot) {
        sound.playHeadshotDink();
        this.kills += 1;
        hitTarget.isAlive = false;
      } else {
        sound.playBodyHit();
      }
      this.renderer.addHitMarker(finalShotX, finalShotY, headshot);
    }

    // Record shot telemetry
    const reactionTimeMs = hitTarget ? Math.round(now - hitTarget.spawnTimeMs) : undefined;
    const telemetry: ShotTelemetryEvent = {
      shotId: `shot_${this.shots.length + 1}`,
      sessionId: this.sessionId,
      timestampMs: Math.round(now),
      weaponId: this.weapon.id,
      shotIndex: this.shotIndex,
      ammoBefore: this.currentAmmo + 1,
      targetId: hitTarget?.id,
      targetPose: hitTarget?.pose,
      aimX: this.crosshairX,
      aimY: this.crosshairY,
      targetCenterX: hitTarget ? hitTarget.x + hitTarget.width * 0.5 : undefined,
      targetCenterY: hitTarget ? hitTarget.y + hitTarget.height * 0.5 : undefined,
      headCenterX: hitTarget ? hitTarget.x + hitTarget.width * 0.5 : undefined,
      headCenterY: hitTarget ? hitTarget.y + hitTarget.height * 0.15 : undefined,
      aimErrorX: hitTarget ? this.crosshairX - (hitTarget.x + hitTarget.width * 0.5) : undefined,
      aimErrorY: hitTarget ? this.crosshairY - (hitTarget.y + hitTarget.height * 0.15) : undefined,
      aimErrorDistance: hitTarget
        ? Math.hypot(
            this.crosshairX - (hitTarget.x + hitTarget.width * 0.5),
            this.crosshairY - (hitTarget.y + hitTarget.height * 0.15)
          )
        : undefined,
      recoilX: this.currentRecoilX,
      recoilY: this.currentRecoilY,
      spreadX,
      spreadY,
      finalShotX,
      finalShotY,
      playerVelocityX: this.playerVx,
      playerVelocityY: this.playerVy,
      playerSpeed: speed,
      stance: this.stance,
      airborne: this.isAirborne,
      scoped: false,
      hit,
      hitbox: hitHitbox,
      headshot,
      reactionTimeMs,
      screenId: 'main',
    };

    this.shots.push(telemetry);
  }

  private loop = () => {
    if (!this.isRunning) return;

    const now = performance.now();
    const dt = Math.min(0.05, (now - this.lastFrameTimeMs) / 1000);
    this.lastFrameTimeMs = now;

    this.updatePhysics(dt, now);
    this.updateSpawner(now);
    this.render(now);

    // Check timer expiration
    const elapsedSeconds = (now - this.startTimeMs) / 1000;
    if (elapsedSeconds >= this.sessionDurationSeconds) {
      this.finishSession();
      return;
    }

    this.animFrameId = requestAnimationFrame(this.loop);
  };

  private updatePhysics(dt: number, now: number) {
    this.isCrouching = !!this.keysHeld['control'] || !!this.keysHeld['c'];

    let wishX = 0;
    let wishY = 0;
    if (this.keysHeld['d']) wishX += 1;
    if (this.keysHeld['a']) wishX -= 1;
    if (this.keysHeld['s']) wishY += 1;
    if (this.keysHeld['w']) wishY -= 1;

    const maxSpeed = this.isCrouching ? this.weapon.maxPlayerSpeed * 0.34 : this.weapon.maxPlayerSpeed;
    const accel = 5.5 * maxSpeed;
    const friction = 5.2;

    if (wishX !== 0 || wishY !== 0) {
      // Counter-strafing
      const dot = this.playerVx * wishX + this.playerVy * wishY;
      if (dot < 0) {
        this.stance = 'CounterStrafing';
        this.playerVx += wishX * accel * 2.5 * dt;
        this.playerVy += wishY * accel * 2.5 * dt;
      } else {
        this.playerVx += wishX * accel * dt;
        this.playerVy += wishY * accel * dt;
      }
    } else {
      const speed = Math.hypot(this.playerVx, this.playerVy);
      if (speed > 0.1) {
        const drop = speed * friction * dt;
        const newSpeed = Math.max(0, speed - drop);
        this.playerVx = (this.playerVx / speed) * newSpeed;
        this.playerVy = (this.playerVy / speed) * newSpeed;
      } else {
        this.playerVx = 0;
        this.playerVy = 0;
      }
    }

    // Clamp speed
    const currentSpeed = Math.hypot(this.playerVx, this.playerVy);
    if (currentSpeed > maxSpeed) {
      this.playerVx = (this.playerVx / currentSpeed) * maxSpeed;
      this.playerVy = (this.playerVy / currentSpeed) * maxSpeed;
    }

    // Stance update
    if (this.isCrouching) {
      this.stance = currentSpeed > 10 ? 'Crouching' : 'Crouched';
    } else if (currentSpeed < 10) {
      this.stance = 'Standing';
    } else if (currentSpeed < maxSpeed * 0.52) {
      this.stance = 'Walking';
    } else {
      this.stance = 'Running';
    }

    // Recoil recovery
    const recoveryTime = this.isCrouching
      ? this.weapon.recoveryTimeCrouchMs
      : this.weapon.recoveryTimeStandMs;
    const elapsedSinceShot = now - this.lastShotTimeMs;
    if (elapsedSinceShot >= recoveryTime) {
      this.shotIndex = 0;
      this.currentRecoilX = 0;
      this.currentRecoilY = 0;
    } else if (this.shotIndex > 0) {
      const decay = 1.0 - elapsedSinceShot / recoveryTime;
      this.currentRecoilX *= decay;
      this.currentRecoilY *= decay;
    }

    // Update targets
    const marginX = 50;
    const marginY = 50;
    const maxY = Math.max(200, this.logicalHeight - 130);

    for (const t of this.targets) {
      t.x += t.vx * dt;

      // Handle sine wave or direct vertical movement
      if (t.movementType === 'sine' && t.baseY !== undefined) {
        t.y = Math.min(maxY, Math.max(marginY, t.baseY + Math.sin((now - t.spawnTimeMs) * 0.0035) * 35));
      } else {
        t.y += t.vy * dt;
      }

      // Horizontal boundary bouncing
      if (t.x < marginX && t.vx < 0) t.vx = Math.abs(t.vx);
      if (t.x + t.width > this.logicalWidth - marginX && t.vx > 0) t.vx = -Math.abs(t.vx);

      // Vertical boundary bouncing for roaming targets
      if (t.y < marginY && t.vy < 0) t.vy = Math.abs(t.vy);
      if (t.y + t.height > maxY && t.vy > 0) t.vy = -Math.abs(t.vy);

      // CS2-style periodic counter-strafe direction switches
      if (t.nextTurnTimeMs && now >= t.nextTurnTimeMs) {
        t.vx = -t.vx;
        t.nextTurnTimeMs = now + 1600 + Math.random() * 2400;
      }

      // Extended lifetime (45-60s) & smooth fade out in final 2.5 seconds
      const remainingLifetime = t.lifetimeMs - (now - t.spawnTimeMs);
      if (remainingLifetime <= 0) {
        t.isAlive = false;
      } else if (remainingLifetime < 2500) {
        t.visibleFraction = Math.max(0.1, remainingLifetime / 2500);
      } else {
        t.visibleFraction = 1.0;
      }
    }

    this.targets = this.targets.filter((t) => t.isAlive);
  }

  private updateSpawner(now: number) {
    // Keep up to 4 concurrent targets on screen
    if (this.targets.length >= 4) return;
    if (now - this.lastSpawnTimeMs < 900) return;

    this.lastSpawnTimeMs = now;
    this.targetCounter += 1;

    const poses: TargetPose[] = ['standing', 'standing', 'crouched', 'head_only'];
    const pose = poses[Math.floor(Math.random() * poses.length)];

    // 80% moving targets for excellent tracking and direction practice
    const moveRoll = Math.random();
    let movementType: 'strafe' | 'roam' | 'sine' | 'static' = 'static';
    let vx = 0;
    let vy = 0;
    let nextTurnTimeMs: number | undefined;

    if (moveRoll < 0.40) {
      // Horizontal ADAD counter-strafing
      movementType = 'strafe';
      vx = (Math.random() < 0.5 ? 1 : -1) * (90 + Math.random() * 60);
      nextTurnTimeMs = now + 1500 + Math.random() * 2000;
    } else if (moveRoll < 0.75) {
      // Multi-directional roaming (diagonal tracking)
      movementType = 'roam';
      vx = (Math.random() < 0.5 ? 1 : -1) * (80 + Math.random() * 50);
      vy = (Math.random() < 0.5 ? 1 : -1) * (40 + Math.random() * 40);
      nextTurnTimeMs = now + 2000 + Math.random() * 2500;
    } else if (moveRoll < 0.88) {
      // Evasive wave bobbing
      movementType = 'sine';
      vx = (Math.random() < 0.5 ? 1 : -1) * (100 + Math.random() * 40);
      nextTurnTimeMs = now + 2000 + Math.random() * 2000;
    } else {
      // Static placement
      movementType = 'static';
    }

    const width = 60;
    const height = pose === 'standing' ? 140 : pose === 'crouched' ? 95 : 35;

    const spawnMarginX = Math.min(100, this.logicalWidth * 0.1);
    const spawnMarginY = Math.min(70, this.logicalHeight * 0.1);
    const availW = Math.max(120, this.logicalWidth - spawnMarginX * 2 - width);
    const availH = Math.max(120, this.logicalHeight - spawnMarginY * 2 - height - 80);
    const x = spawnMarginX + Math.random() * availW;
    const y = spawnMarginY + Math.random() * availH;

    const hitboxes = [
      {
        type: 'head' as HitboxType,
        x: width * 0.25,
        y: 0,
        width: width * 0.5,
        height: height * 0.25,
        damageMultiplier: 4.0,
      },
      {
        type: 'upper_chest' as HitboxType,
        x: width * 0.1,
        y: height * 0.25,
        width: width * 0.8,
        height: height * 0.4,
        damageMultiplier: 1.0,
      },
    ];

    // Extended persistent lifetime: 45 to 60 seconds (stay on screen until shot)
    const lifetimeMs = 45000 + Math.floor(Math.random() * 15000);

    const target: TargetEntity = {
      id: `target_${this.targetCounter}`,
      spawnTimeMs: now,
      lifetimeMs,
      x,
      y,
      vx,
      vy,
      width,
      height,
      pose,
      behaviour: movementType === 'static' ? 'static' : 'strafe',
      hitboxes,
      visibleFraction: 1.0,
      isAlive: true,
      screenId: 'main',
      baseY: y,
      nextTurnTimeMs,
      movementType,
    };

    this.targets.push(target);
  }

  private render(now: number) {
    const dpr = this.dpr;
    const w = this.logicalWidth;
    const h = this.logicalHeight;

    const ctx = this.canvas.getContext('2d');
    if (ctx) {
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    }

    this.renderer.clear(w, h);

    // 1. Render active target silhouettes
    this.renderer.renderTargets(this.targets, now);

    // 2. Render hit markers
    this.renderer.renderHitMarkers(now);

    // 3. Render CS2 Crosshair (unified with mouse/touchpad coordinate)
    const speed = Math.hypot(this.playerVx, this.playerVy);
    this.renderer.renderCrosshair(this.crosshairX, this.crosshairY, this.crosshairConfig, speed * 0.08);

    // 4. Render Minimal HUD
    const elapsed = (now - this.startTimeMs) / 1000;
    const remaining = Math.max(0, this.sessionDurationSeconds - elapsed);
    const hits = this.shots.filter((s) => s.hit).length;
    const acc = this.shots.length > 0 ? (hits / this.shots.length) * 100 : 0;

    this.renderer.renderHUD(
      w,
      h,
      remaining,
      this.currentAmmo,
      this.weapon.magazineSize,
      this.kills,
      acc,
      this.stance,
      speed
    );
  }

  private finishSession() {
    this.stop();
    const durationSeconds = Math.round((performance.now() - this.startTimeMs) / 1000);
    this.callbacks.onSessionFinish(this.shots, durationSeconds);
  }
}
