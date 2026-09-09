import React from 'react';
import {
  Crosshair,
  Play,
  Settings as SettingsIcon,
  Target,
  Zap,
  Clock,
  TrendingUp,
  Flame,
} from 'lucide-react';
import { SessionMetrics, WeaponDefinition } from '@timebreak/shared-types';
import { translations } from '../i18n/translations.js';

interface DashboardProps {
  onStartBreak: () => void;
  onOpenSettings: () => void;
  recentSessions: SessionMetrics[];
  activeWeapon: WeaponDefinition;
  nextBreakSeconds: number;
  lang: 'uz' | 'en';
}

export const Dashboard: React.FC<DashboardProps> = ({
  onStartBreak,
  onOpenSettings,
  recentSessions,
  activeWeapon,
  nextBreakSeconds,
  lang,
}) => {
  const t = translations[lang];

  // Aggregated stats
  const totalSessions = recentSessions.length;
  const avgAcc =
    totalSessions > 0
      ? recentSessions.reduce((acc, s) => acc + s.accuracy, 0) / totalSessions
      : 0;
  const avgHs =
    totalSessions > 0
      ? recentSessions.reduce((acc, s) => acc + s.headshotPercentage, 0) / totalSessions
      : 0;
  const avgReaction =
    totalSessions > 0
      ? recentSessions.reduce((acc, s) => acc + s.avgReactionTimeMs, 0) / totalSessions
      : 0;

  const mins = Math.floor(nextBreakSeconds / 60);
  const secs = Math.floor(nextBreakSeconds % 60);
  const nextBreakStr = `${mins}:${secs < 10 ? '0' : ''}${secs}`;

  return (
    <div className="flex flex-col gap-6 p-8 max-w-5xl mx-auto">
      {/* Top Header Card */}
      <div className="glass-panel p-6 flex items-center justify-between shadow-xl">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-xl bg-sky-500/20 border border-sky-400/30 flex items-center justify-center text-sky-400">
            <Crosshair className="w-6 h-6" />
          </div>
          <div>
            <h1 className="text-2xl font-bold text-slate-100 flex items-center gap-2">
              {t.appTitle}
              <span className="text-xs px-2 py-0.5 rounded-full bg-sky-500/10 text-sky-400 border border-sky-500/20 font-mono">
                CS2 Engine
              </span>
            </h1>
            <p className="text-sm text-slate-400">{t.tagline}</p>
          </div>
        </div>

        <div className="flex items-center gap-4">
          <div className="flex items-center gap-2 px-4 py-2 rounded-lg bg-slate-900/60 border border-white/5 font-mono text-sm text-slate-300">
            <Clock className="w-4 h-4 text-sky-400" />
            <span>{t.nextBreakIn}</span>
            <span className="font-bold text-sky-400">{nextBreakStr}</span>
          </div>

          <button onClick={onStartBreak} className="btn-primary">
            <Play className="w-4 h-4 fill-current" />
            {t.startBreakNow}
          </button>

          <button onClick={onOpenSettings} className="btn-secondary">
            <SettingsIcon className="w-4 h-4" />
          </button>
        </div>
      </div>

      {/* Overview Stat Cards */}
      <div className="grid grid-cols-4 gap-4">
        <div className="glass-panel p-5 flex flex-col gap-1">
          <span className="text-xs font-medium text-slate-400 flex items-center gap-1.5">
            <Target className="w-3.5 h-3.5 text-sky-400" />
            {t.accuracy}
          </span>
          <span className="text-3xl font-bold text-slate-100 font-mono">
            {avgAcc > 0 ? `${avgAcc.toFixed(1)}%` : '--'}
          </span>
          <span className="text-xs text-slate-500">O'rtacha aniqlik</span>
        </div>

        <div className="glass-panel p-5 flex flex-col gap-1">
          <span className="text-xs font-medium text-slate-400 flex items-center gap-1.5">
            <Flame className="w-3.5 h-3.5 text-rose-400" />
            {t.headshots}
          </span>
          <span className="text-3xl font-bold text-slate-100 font-mono">
            {avgHs > 0 ? `${avgHs.toFixed(1)}%` : '--'}
          </span>
          <span className="text-xs text-slate-500">Boshga tegish ulushi</span>
        </div>

        <div className="glass-panel p-5 flex flex-col gap-1">
          <span className="text-xs font-medium text-slate-400 flex items-center gap-1.5">
            <Zap className="w-3.5 h-3.5 text-amber-400" />
            {t.avgReaction}
          </span>
          <span className="text-3xl font-bold text-slate-100 font-mono">
            {avgReaction > 0 ? `${avgReaction.toFixed(0)} ms` : '--'}
          </span>
          <span className="text-xs text-slate-500">Birinchi nishon reaktsiyasi</span>
        </div>

        <div className="glass-panel p-5 flex flex-col gap-1">
          <span className="text-xs font-medium text-slate-400 flex items-center gap-1.5">
            <TrendingUp className="w-3.5 h-3.5 text-emerald-400" />
            {t.todaySessions}
          </span>
          <span className="text-3xl font-bold text-slate-100 font-mono">
            {totalSessions}
          </span>
          <span className="text-xs text-slate-500">Yakunlangan tanaffuslar</span>
        </div>
      </div>

      {/* Active Weapon Card */}
      <div className="glass-panel p-5 flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="px-3 py-1.5 rounded-md bg-slate-900/80 border border-slate-700 text-sky-400 font-mono font-bold text-lg">
            {activeWeapon.displayName}
          </div>
          <div className="text-sm text-slate-400">
            <span>{t.currentWeapon} </span>
            <span className="text-slate-200 font-semibold">{activeWeapon.displayName}</span>
            <span className="text-slate-500 text-xs ml-2">
              ({activeWeapon.rpm} RPM • {activeWeapon.magazineSize} round mag • {activeWeapon.sourceStatus} CS2 profile)
            </span>
          </div>
        </div>

        <div className="text-xs font-mono text-slate-400 flex items-center gap-2">
          <span>Qaytish (Recoil):</span>
          <span className="text-emerald-400 font-semibold">Deterministik Spray</span>
        </div>
      </div>

      {/* Recent Sessions Table */}
      <div className="glass-panel p-6 flex flex-col gap-4">
        <h2 className="text-lg font-bold text-slate-100 flex items-center gap-2">
          <Clock className="w-4 h-4 text-sky-400" />
          {t.sessionHistory}
        </h2>

        {recentSessions.length === 0 ? (
          <div className="text-center py-10 text-slate-500 text-sm">
            {t.noSessionsYet}
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full text-left text-sm text-slate-300">
              <thead className="text-xs uppercase text-slate-500 border-b border-white/5 font-mono">
                <tr>
                  <th className="py-2.5 px-3">Qurol</th>
                  <th className="py-2.5 px-3">O'qlar</th>
                  <th className="py-2.5 px-3">Aniqlik</th>
                  <th className="py-2.5 px-3">HS %</th>
                  <th className="py-2.5 px-3">Reaksiya</th>
                  <th className="py-2.5 px-3">Og'ish (Bias)</th>
                  <th className="py-2.5 px-3">Ball</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-white/5">
                {recentSessions.map((s) => (
                  <tr key={s.sessionId} className="hover:bg-white/[0.02] transition-colors">
                    <td className="py-3 px-3 font-mono font-medium text-slate-200">
                      {s.weaponId.replace('weapon_', '').toUpperCase()}
                    </td>
                    <td className="py-3 px-3 font-mono">{s.shotsFired}</td>
                    <td className="py-3 px-3 font-mono text-sky-400 font-semibold">
                      {s.accuracy.toFixed(1)}%
                    </td>
                    <td className="py-3 px-3 font-mono text-rose-400">
                      {s.headshotPercentage.toFixed(1)}%
                    </td>
                    <td className="py-3 px-3 font-mono">
                      {s.avgReactionTimeMs > 0 ? `${s.avgReactionTimeMs.toFixed(0)} ms` : '--'}
                    </td>
                    <td className="py-3 px-3">
                      <span className="text-xs px-2 py-0.5 rounded bg-slate-800 text-slate-300 border border-slate-700">
                        {s.directionBias}
                      </span>
                    </td>
                    <td className="py-3 px-3 font-mono font-bold text-emerald-400">
                      {s.overallScore.toFixed(0)}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  );
};
