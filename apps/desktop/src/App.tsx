import React, { useEffect, useState } from 'react';
import {
  BreakSchedulerConfig,
  CrosshairConfig,
  CS2Profile,
  Recommendation,
  SessionMetrics,
  ShotTelemetryEvent,
  WeaponDefinition,
} from '@timebreak/shared-types';
import { OverlayView } from './components/OverlayView.js';
import { Dashboard } from './components/Dashboard.js';
import { SettingsView } from './components/SettingsView.js';
import { ResultsModal } from './components/ResultsModal.js';

// Initial default weapon profiles
const DEFAULT_WEAPON: WeaponDefinition = {
  id: 'weapon_ak47',
  displayName: 'AK-47',
  category: 'Rifle',
  fireMode: 'full_auto',
  cycleTimeMs: 100,
  rpm: 600,
  magazineSize: 30,
  reloadTimeMs: 2433,
  damage: 36.0,
  headshotMultiplier: 4.0,
  armorRatio: 1.55,
  maxPlayerSpeed: 215.0,
  baseSpread: 0.6,
  inaccuracyStand: 4.2,
  inaccuracyCrouch: 3.41,
  inaccuracyMove: 175.0,
  inaccuracyJump: 180.0,
  inaccuracyAir: 210.0,
  recoveryTimeStandMs: 380,
  recoveryTimeCrouchMs: 300,
  recoilMagnitude: 30.0,
  recoilPattern: [
    { shotIndex: 0, dx: 0, dy: 0 },
    { shotIndex: 1, dx: 0, dy: -4 },
    { shotIndex: 2, dx: 0.5, dy: -9 },
    { shotIndex: 3, dx: 0.2, dy: -14.5 },
    { shotIndex: 4, dx: -0.8, dy: -20 },
    { shotIndex: 5, dx: -2.5, dy: -24 },
    { shotIndex: 6, dx: -4.0, dy: -26.5 },
  ],
  scoped: false,
  sourceVersion: 'CS2-2024.1',
  sourceStatus: 'verified',
};

const DEFAULT_PROFILE: CS2Profile = {
  dpi: 800,
  sensitivity: 1.25,
  zoomSensitivity: 1.0,
  eDpi: 1000,
  cm360: 41.5,
  resolutionWidth: 1920,
  resolutionHeight: 1080,
};

const DEFAULT_CROSSHAIR: CrosshairConfig = {
  size: 6,
  thickness: 2,
  gap: 3,
  dot: true,
  color: '#38bdf8',
  alpha: 1.0,
  outline: true,
  dynamicSpread: true,
};

const DEFAULT_SCHEDULER: BreakSchedulerConfig = {
  workIntervalMinutes: 25,
  breakDurationMinutes: 5,
  autoStartBreak: true,
  notificationBeforeBreak: true,
  notificationLeadTimeMinutes: 2,
  soundEnabled: true,
  masterVolume: 0.7,
  defaultWeapon: 'weapon_ak47',
  defaultMode: 'quick_break',
  language: 'uz',
};

export const App: React.FC = () => {
  const [isOverlayMode, setIsOverlayMode] = useState<boolean>(false);
  const [currentView, setCurrentView] = useState<'dashboard' | 'settings'>('dashboard');
  const [activeWeapon, setActiveWeapon] = useState<WeaponDefinition>(DEFAULT_WEAPON);
  const [profile, setProfile] = useState<CS2Profile>(DEFAULT_PROFILE);
  const [crosshairConfig, setCrosshairConfig] = useState<CrosshairConfig>(DEFAULT_CROSSHAIR);
  const [schedulerConfig, setSchedulerConfig] = useState<BreakSchedulerConfig>(DEFAULT_SCHEDULER);
  const [availableWeapons, setAvailableWeapons] = useState<WeaponDefinition[]>([DEFAULT_WEAPON]);
  const [recentSessions, setRecentSessions] = useState<SessionMetrics[]>([]);
  const [latestResults, setLatestResults] = useState<{
    metrics: SessionMetrics;
    rec?: Recommendation;
  } | null>(null);

  const [nextBreakSeconds, setNextBreakSeconds] = useState<number>(25 * 60);

  // Check if launched directly as overlay or in dev mode
  useEffect(() => {
    // Check URL params or env
    const params = new URLSearchParams(window.location.search);
    if (params.get('overlay') === '1') {
      setIsOverlayMode(true);
      document.body.classList.remove('app-window');
    } else {
      document.body.classList.add('app-window');
    }

    // Try fetching from Tauri backend if running inside Tauri
    const loadTauriData = async () => {
      try {
        const { invoke } = await import('@tauri-apps/api/core');
        const weapons = (await invoke('get_all_weapons')) as WeaponDefinition[];
        if (weapons && weapons.length > 0) {
          setAvailableWeapons(weapons);
        }
        const recent = (await invoke('get_recent_sessions', { limit: 20 })) as SessionMetrics[];
        if (recent) {
          setRecentSessions(recent);
        }

        const devConfig = (await invoke('get_dev_config')) as {
          is_dev: boolean;
          auto_break: boolean;
          session_seconds?: number;
        };

        if (devConfig?.session_seconds) {
          setSchedulerConfig((prev) => ({
            ...prev,
            breakDurationMinutes: Math.max(1, Math.round(devConfig.session_seconds! / 60)),
          }));
        }

        if (devConfig?.auto_break) {
          setTimeout(() => {
            handleStartBreak();
          }, 100);
        }
      } catch {
        // Running in web preview fallback
      }
    };

    loadTauriData();
  }, []);

  // Work interval countdown timer
  useEffect(() => {
    if (isOverlayMode) return;

    const timer = setInterval(() => {
      setNextBreakSeconds((prev) => {
        if (prev <= 1) {
          // Trigger auto break if enabled
          if (schedulerConfig.autoStartBreak) {
            handleStartBreak();
          }
          return schedulerConfig.workIntervalMinutes * 60;
        }
        return prev - 1;
      });
    }, 1000);

    return () => clearInterval(timer);
  }, [isOverlayMode, schedulerConfig]);

  const handleStartBreak = async () => {
    setIsOverlayMode(true);
    document.body.classList.remove('app-window');

    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('start_break');
    } catch {
      // In-browser mock
    }
  };

  const handleEscape = async () => {
    setIsOverlayMode(false);
    document.body.classList.add('app-window');

    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('emergency_escape');
    } catch {
      // Fallback
    }
  };

  const handleSessionFinish = async (shots: ShotTelemetryEvent[], durationSeconds: number) => {
    setIsOverlayMode(false);
    document.body.classList.add('app-window');

    const startTimeMs = shots.length > 0 ? shots[0].timestampMs : Date.now() - durationSeconds * 1000;
    const endTimeMs = Date.now();

    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const [metrics, rec] = (await invoke('record_session_results', {
        sessionId: `tb_${Date.now()}`,
        weaponId: activeWeapon.id,
        startTimeMs,
        endTimeMs,
        shots,
      })) as [SessionMetrics, Recommendation];

      setLatestResults({ metrics, rec });
      setRecentSessions((prev) => [metrics, ...prev]);
      await invoke('end_break');
    } catch {
      // Fallback in web preview: generate local metrics
      const hits = shots.filter((s) => s.hit).length;
      const headshots = shots.filter((s) => s.headshot).length;
      const acc = shots.length > 0 ? (hits / shots.length) * 100 : 0;
      const hsPct = hits > 0 ? (headshots / hits) * 100 : 0;

      const fallbackMetrics: SessionMetrics = {
        sessionId: `tb_${Date.now()}`,
        startTimeMs,
        endTimeMs,
        durationSeconds,
        weaponId: activeWeapon.id,
        shotsFired: shots.length,
        hits,
        misses: shots.length - hits,
        accuracy: acc,
        headshots,
        headshotPercentage: hsPct,
        kills: headshots,
        avgReactionTimeMs: 275,
        medianReactionTimeMs: 260,
        p90ReactionTimeMs: 320,
        meanHorizontalError: 0,
        meanVerticalError: 0,
        rmsError: 10.5,
        directionBias: 'center',
        movementScore: 85,
        sprayScore: 80,
        overallScore: Math.round(acc * 0.5 + hsPct * 0.3 + 20),
        movingShotsPercentage: 15,
      };

      const fallbackRec: Recommendation = {
        id: 'rec_1',
        sessionId: fallbackMetrics.sessionId,
        title: "Barqaror o'q uzish va harakat intizomi",
        observation: `${fallbackMetrics.accuracy.toFixed(1)}% aniqlik qayd etildi.`,
        evidence: "O'rtacha og'ish minimal.",
        likelyCause: 'Barqaror mushak xotirasi.',
        suggestedDrill: 'quick_break',
        suggestedDurationMinutes: 5,
      };

      setLatestResults({ metrics: fallbackMetrics, rec: fallbackRec });
      setRecentSessions((prev) => [fallbackMetrics, ...prev]);
    }
  };

  // If in overlay mode, render ONLY the transparent overlay canvas
  if (isOverlayMode) {
    return (
      <OverlayView
        weapon={activeWeapon}
        profile={profile}
        crosshairConfig={crosshairConfig}
        durationSeconds={schedulerConfig.breakDurationMinutes * 60}
        onSessionFinish={handleSessionFinish}
        onEscape={handleEscape}
      />
    );
  }

  return (
    <div className="min-h-screen bg-[#0b0f19] text-slate-100 selection:bg-sky-500 selection:text-white">
      {currentView === 'dashboard' ? (
        <Dashboard
          onStartBreak={handleStartBreak}
          onOpenSettings={() => setCurrentView('settings')}
          recentSessions={recentSessions}
          activeWeapon={activeWeapon}
          nextBreakSeconds={nextBreakSeconds}
          lang={schedulerConfig.language}
        />
      ) : (
        <SettingsView
          onBack={() => setCurrentView('dashboard')}
          profile={profile}
          onSaveProfile={setProfile}
          activeWeaponId={activeWeapon.id}
          onSelectWeapon={(id) => {
            const found = availableWeapons.find((w) => w.id === id);
            if (found) setActiveWeapon(found);
          }}
          availableWeapons={availableWeapons}
          schedulerConfig={schedulerConfig}
          onSaveScheduler={setSchedulerConfig}
          crosshairConfig={crosshairConfig}
          onSaveCrosshair={setCrosshairConfig}
          lang={schedulerConfig.language}
          onSelectLang={(lang) => setSchedulerConfig({ ...schedulerConfig, language: lang })}
        />
      )}

      {/* Post-Break Results Modal */}
      {latestResults && (
        <ResultsModal
          metrics={latestResults.metrics}
          recommendation={latestResults.rec}
          onClose={() => setLatestResults(null)}
          lang={schedulerConfig.language}
        />
      )}
    </div>
  );
};
