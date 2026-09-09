import React, { useEffect, useRef } from 'react';
import { GameEngine } from '../overlay/GameEngine.js';
import {
  CrosshairConfig,
  CS2Profile,
  ShotTelemetryEvent,
  WeaponDefinition,
} from '@timebreak/shared-types';

interface OverlayViewProps {
  weapon: WeaponDefinition;
  profile: CS2Profile;
  crosshairConfig: CrosshairConfig;
  durationSeconds: number;
  onSessionFinish: (shots: ShotTelemetryEvent[], durationSeconds: number) => void;
  onEscape: () => void;
}

export const OverlayView: React.FC<OverlayViewProps> = ({
  weapon,
  profile,
  crosshairConfig,
  durationSeconds,
  onSessionFinish,
  onEscape,
}) => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const engineRef = useRef<GameEngine | null>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    // Handle high DPI
    const dpr = window.devicePixelRatio || 1;
    canvas.width = window.innerWidth * dpr;
    canvas.height = window.innerHeight * dpr;

    const ctx = canvas.getContext('2d');
    if (ctx) {
      ctx.scale(dpr, dpr);
    }

    const engine = new GameEngine(
      canvas,
      weapon,
      profile,
      crosshairConfig,
      {
        onSessionFinish,
        onEscape,
      }
    );
    engineRef.current = engine;
    engine.start(durationSeconds);

    const handleResize = () => {
      if (canvas) {
        canvas.width = window.innerWidth * dpr;
        canvas.height = window.innerHeight * dpr;
        if (ctx) ctx.scale(dpr, dpr);
      }
    };

    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
      engine.stop();
    };
  }, [weapon, profile, crosshairConfig, durationSeconds, onSessionFinish, onEscape]);

  return (
    <canvas
      ref={canvasRef}
      className="overlay-canvas"
      style={{
        width: '100vw',
        height: '100vh',
        background: 'transparent',
      }}
    />
  );
};
