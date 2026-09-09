import React, { useEffect, useRef, useState } from 'react';
import { GameEngine } from '../overlay/GameEngine.js';
import {
  CrosshairConfig,
  CS2Profile,
  ShotTelemetryEvent,
  WeaponDefinition,
} from '@timebreak/shared-types';
import { Maximize2, Minimize2, Settings, X, Crosshair, EyeOff } from 'lucide-react';

interface OverlayViewProps {
  weapon: WeaponDefinition;
  profile: CS2Profile;
  crosshairConfig: CrosshairConfig;
  durationSeconds: number;
  onSessionFinish: (shots: ShotTelemetryEvent[], durationSeconds: number) => void;
  onEscape: () => void;
  onOpenDashboard?: () => void;
  onHide?: () => void;
}

export const OverlayView: React.FC<OverlayViewProps> = ({
  weapon,
  profile,
  crosshairConfig,
  durationSeconds,
  onSessionFinish,
  onEscape,
  onOpenDashboard,
  onHide,
}) => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const engineRef = useRef<GameEngine | null>(null);
  const [isPipMode, setIsPipMode] = useState<boolean>(false);

  const handleHide = async () => {
    if (onHide) {
      onHide();
      return;
    }
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('hide_overlay');
    } catch {
      onEscape();
    }
  };

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    // Handle high DPI
    const dpr = window.devicePixelRatio || 1;
    canvas.width = window.innerWidth * dpr;
    canvas.height = window.innerHeight * dpr;

    const engine = new GameEngine(
      canvas,
      weapon,
      profile,
      crosshairConfig,
      {
        onSessionFinish,
        onEscape,
        onHide: handleHide,
      }
    );
    engineRef.current = engine;
    engine.start(durationSeconds);

    const handleResize = () => {
      if (canvas) {
        const currentDpr = window.devicePixelRatio || 1;
        canvas.width = window.innerWidth * currentDpr;
        canvas.height = window.innerHeight * currentDpr;
      }
    };

    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
      engine.stop();
    };
  }, [weapon, profile, crosshairConfig, durationSeconds, onSessionFinish, onEscape]);

  const togglePip = async () => {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const isFullscreen = (await invoke('toggle_fullscreen')) as boolean;
      setIsPipMode(!isFullscreen);
    } catch {
      setIsPipMode((prev) => !prev);
    }
  };

  return (
    <div
      className="relative w-screen h-screen select-none overflow-hidden"
      style={{
        background: 'transparent',
        backgroundColor: 'transparent',
        cursor: 'none',
      }}
    >
      {/* 100% Transparent Fullscreen/PiP Game Canvas */}
      <canvas
        ref={canvasRef}
        className="overlay-canvas"
        style={{
          width: '100vw',
          height: '100vh',
          background: 'transparent',
          backgroundColor: 'transparent',
          cursor: 'none',
        }}
      />

      {/* Floating Transparent Controls Toolbar (Telegram PiP style) */}
      <div className="absolute top-3 left-4 right-4 z-40 flex items-center justify-between pointer-events-none">
        {/* Left Badge: Status & Active Weapon & Hotkey */}
        <div className="pointer-events-auto flex items-center gap-2 px-3 py-1.5 rounded-full bg-slate-950/60 backdrop-blur-md border border-white/10 shadow-lg text-xs font-mono text-slate-200">
          <Crosshair className="w-3.5 h-3.5 text-sky-400" />
          <span className="font-bold text-sky-400">TimeBreak</span>
          <span className="text-slate-500">•</span>
          <span>{weapon.displayName}</span>
          <span className="text-slate-500">•</span>
          <span className="text-slate-300">
            Hot key: <kbd className="px-1.5 py-0.5 rounded bg-amber-500/20 text-amber-300 border border-amber-500/30 font-bold">H</kbd> yashirish
          </span>
        </div>

        {/* Right Action Buttons */}
        <div className="pointer-events-auto flex items-center gap-2">
          {/* Hide overlay button (H hotkey) */}
          <button
            onClick={handleHide}
            title="Ekrandan yashirish (Hot key: 'H')"
            className="px-2.5 py-1.5 rounded-full bg-slate-950/60 hover:bg-slate-900/80 backdrop-blur-md border border-white/10 text-slate-300 hover:text-amber-400 transition-all shadow-lg cursor-pointer flex items-center gap-1.5 text-xs font-mono"
          >
            <EyeOff className="w-3.5 h-3.5" />
            <span className="font-bold text-amber-400">H</span>
            <span className="text-slate-300 hidden sm:inline">Yashirish</span>
          </button>

          {/* PiP / Fullscreen Toggle */}
          <button
            onClick={togglePip}
            title={isPipMode ? 'To\'liq ekran (Fullscreen)' : 'Telegram PiP oynasi (Kichik oyna)'}
            className="p-2 rounded-full bg-slate-950/60 hover:bg-slate-900/80 backdrop-blur-md border border-white/10 text-slate-300 hover:text-sky-400 transition-all shadow-lg cursor-pointer"
          >
            {isPipMode ? <Maximize2 className="w-3.5 h-3.5" /> : <Minimize2 className="w-3.5 h-3.5" />}
          </button>

          {/* Settings / Dashboard Modal Toggle */}
          {onOpenDashboard && (
            <button
              onClick={onOpenDashboard}
              title="Sozlamalar va Statistika"
              className="p-2 rounded-full bg-slate-950/60 hover:bg-slate-900/80 backdrop-blur-md border border-white/10 text-slate-300 hover:text-sky-400 transition-all shadow-lg cursor-pointer"
            >
              <Settings className="w-3.5 h-3.5" />
            </button>
          )}

          {/* Emergency Escape / Exit */}
          <button
            onClick={onEscape}
            title="Tanaffusni yakunlash (Esc)"
            className="p-2 rounded-full bg-slate-950/60 hover:bg-rose-950/80 backdrop-blur-md border border-white/10 text-slate-300 hover:text-rose-400 transition-all shadow-lg cursor-pointer"
          >
            <X className="w-3.5 h-3.5" />
          </button>
        </div>
      </div>
    </div>
  );
};
