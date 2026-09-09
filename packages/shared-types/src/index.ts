export type AppState =
  | 'Idle'
  | 'Scheduled'
  | 'PreparingBreak'
  | 'Countdown'
  | 'Playing'
  | 'Paused'
  | 'Results'
  | 'ReturningToWork';

export type GameMode =
  | 'quick_break'
  | 'headshot'
  | 'flick'
  | 'reaction'
  | 'tracking'
  | 'spray_control'
  | 'counter_strafe'
  | 'peek_practice';

export type FireMode = 'semi_auto' | 'full_auto' | 'burst';

export type PlayerStance =
  | 'Standing'
  | 'Walking'
  | 'Running'
  | 'Crouching'
  | 'Crouched'
  | 'Jumping'
  | 'Airborne'
  | 'Landing'
  | 'CounterStrafing';

export interface RecoilPoint {
  shotIndex: number;
  dx: number;
  dy: number;
}

export interface WeaponDefinition {
  id: string;
  displayName: string;
  category: 'Rifle' | 'Sniper' | 'Pistol' | 'SMG';
  fireMode: FireMode;
  cycleTimeMs: number;
  rpm: number;
  magazineSize: number;
  reloadTimeMs: number;
  damage: number;
  headshotMultiplier: number;
  armorRatio: number;
  maxPlayerSpeed: number;
  baseSpread: number;
  inaccuracyStand: number;
  inaccuracyCrouch: number;
  inaccuracyMove: number;
  inaccuracyJump: number;
  inaccuracyAir: number;
  recoveryTimeStandMs: number;
  recoveryTimeCrouchMs: number;
  recoilMagnitude: number;
  recoilPattern: RecoilPoint[];
  scoped: boolean;
  zoomFovMultiplier?: number;
  sourceVersion: string;
  sourceStatus: 'verified' | 'derived' | 'approximated';
}

export type HitboxType =
  | 'head'
  | 'upper_chest'
  | 'lower_chest'
  | 'stomach'
  | 'left_arm'
  | 'right_arm'
  | 'left_leg'
  | 'right_leg';

export interface HitboxRect {
  type: HitboxType;
  x: number;
  y: number;
  width: number;
  height: number;
  damageMultiplier: number;
}

export type TargetPose =
  | 'standing'
  | 'crouched'
  | 'head_only'
  | 'head_shoulder'
  | 'half_body_left'
  | 'half_body_right';

export type TargetBehaviour =
  | 'static'
  | 'pop_up'
  | 'strafe'
  | 'peek_left'
  | 'peek_right'
  | 'reaction';

export interface TargetEntity {
  id: string;
  spawnTimeMs: number;
  lifetimeMs: number;
  x: number;
  y: number;
  vx: number;
  vy: number;
  width: number;
  height: number;
  pose: TargetPose;
  behaviour: TargetBehaviour;
  hitboxes: HitboxRect[];
  visibleFraction: number;
  isAlive: boolean;
  screenId: string;
  baseY?: number;
  nextTurnTimeMs?: number;
  movementType?: 'strafe' | 'roam' | 'sine' | 'static';
}

export interface ShotTelemetryEvent {
  shotId: string;
  sessionId: string;
  timestampMs: number;
  weaponId: string;
  shotIndex: number;
  ammoBefore: number;
  targetId?: string;
  targetPose?: TargetPose;
  targetVisibleFraction?: number;
  aimX: number;
  aimY: number;
  targetCenterX?: number;
  targetCenterY?: number;
  headCenterX?: number;
  headCenterY?: number;
  aimErrorX?: number;
  aimErrorY?: number;
  aimErrorDistance?: number;
  recoilX: number;
  recoilY: number;
  spreadX: number;
  spreadY: number;
  finalShotX: number;
  finalShotY: number;
  playerVelocityX: number;
  playerVelocityY: number;
  playerSpeed: number;
  stance: PlayerStance;
  airborne: boolean;
  scoped: boolean;
  hit: boolean;
  hitbox?: HitboxType;
  headshot: boolean;
  reactionTimeMs?: number;
  screenId: string;
}

export type DirectionBias =
  | 'center'
  | 'high'
  | 'low'
  | 'left'
  | 'right'
  | 'high_left'
  | 'high_right'
  | 'low_left'
  | 'low_right';

export interface SessionMetrics {
  sessionId: string;
  startTimeMs: number;
  endTimeMs: number;
  durationSeconds: number;
  weaponId: string;
  shotsFired: number;
  hits: number;
  misses: number;
  accuracy: number;
  headshots: number;
  headshotPercentage: number;
  kills: number;
  avgReactionTimeMs: number;
  medianReactionTimeMs: number;
  p90ReactionTimeMs: number;
  meanHorizontalError: number;
  meanVerticalError: number;
  rmsError: number;
  directionBias: DirectionBias;
  movementScore: number;
  sprayScore: number;
  overallScore: number;
  movingShotsPercentage: number;
}

export interface Recommendation {
  id: string;
  sessionId: string;
  title: string;
  observation: string;
  evidence: string;
  likelyCause: string;
  suggestedDrill: GameMode;
  suggestedDurationMinutes: number;
}

export interface CS2Profile {
  dpi: number;
  sensitivity: number;
  zoomSensitivity: number;
  eDpi: number;
  cm360: number;
  resolutionWidth: number;
  resolutionHeight: number;
}

export interface CrosshairConfig {
  size: number;
  thickness: number;
  gap: number;
  dot: boolean;
  color: string;
  alpha: number;
  outline: boolean;
  dynamicSpread: boolean;
}

export interface BreakSchedulerConfig {
  workIntervalMinutes: number;
  breakDurationMinutes: number;
  autoStartBreak: boolean;
  notificationBeforeBreak: boolean;
  notificationLeadTimeMinutes: number;
  soundEnabled: boolean;
  masterVolume: number;
  defaultWeapon: string;
  defaultMode: GameMode;
  language: 'uz' | 'en';
}
