import React from 'react';
import { Recommendation, SessionMetrics } from '@timebreak/shared-types';
import { translations } from '../i18n/translations.js';
import { CheckCircle2, AlertTriangle, ArrowRight, Crosshair, Award } from 'lucide-react';

interface ResultsModalProps {
  metrics: SessionMetrics;
  recommendation?: Recommendation;
  onClose: () => void;
  lang: 'uz' | 'en';
}

export const ResultsModal: React.FC<ResultsModalProps> = ({
  metrics,
  recommendation,
  onClose,
  lang,
}) => {
  const t = translations[lang];

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/70 backdrop-blur-md">
      <div className="glass-panel w-full max-w-2xl p-8 flex flex-col gap-6 shadow-2xl animate-in fade-in zoom-in duration-200">
        {/* Header */}
        <div className="flex items-center justify-between border-b border-white/10 pb-4">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-full bg-emerald-500/20 text-emerald-400 flex items-center justify-center">
              <CheckCircle2 className="w-6 h-6" />
            </div>
            <div>
              <h2 className="text-xl font-bold text-slate-100">{t.breakComplete}</h2>
              <p className="text-xs text-slate-400 font-mono">
                {metrics.durationSeconds}s mashg'ulot yakunlandi
              </p>
            </div>
          </div>

          <div className="flex items-center gap-2 px-3 py-1 rounded-full bg-sky-500/10 border border-sky-500/20 text-sky-400 font-mono font-bold text-sm">
            <Award className="w-4 h-4" />
            <span>Umumiy ball: {metrics.overallScore.toFixed(0)}/100</span>
          </div>
        </div>

        {/* 5 Core Metric Cards */}
        <div className="grid grid-cols-5 gap-3 text-center">
          <div className="p-3 rounded-lg bg-slate-900/60 border border-white/5">
            <span className="text-xs text-slate-400">{t.accuracy}</span>
            <div className="text-2xl font-bold text-sky-400 font-mono mt-1">
              {metrics.accuracy.toFixed(1)}%
            </div>
            <span className="text-[10px] text-slate-500">{metrics.hits}/{metrics.shotsFired} o'q</span>
          </div>

          <div className="p-3 rounded-lg bg-slate-900/60 border border-white/5">
            <span className="text-xs text-slate-400">{t.headshots}</span>
            <div className="text-2xl font-bold text-rose-400 font-mono mt-1">
              {metrics.headshotPercentage.toFixed(1)}%
            </div>
            <span className="text-[10px] text-slate-500">{metrics.headshots} HS</span>
          </div>

          <div className="p-3 rounded-lg bg-slate-900/60 border border-white/5">
            <span className="text-xs text-slate-400">{t.avgReaction}</span>
            <div className="text-2xl font-bold text-amber-400 font-mono mt-1">
              {metrics.avgReactionTimeMs > 0 ? `${metrics.avgReactionTimeMs.toFixed(0)}ms` : '--'}
            </div>
            <span className="text-[10px] text-slate-500">P90: {metrics.p90ReactionTimeMs.toFixed(0)}ms</span>
          </div>

          <div className="p-3 rounded-lg bg-slate-900/60 border border-white/5">
            <span className="text-xs text-slate-400">{t.sprayControl}</span>
            <div className="text-2xl font-bold text-emerald-400 font-mono mt-1">
              {metrics.sprayScore.toFixed(0)}
            </div>
            <span className="text-[10px] text-slate-500">RMS: {metrics.rmsError.toFixed(1)}px</span>
          </div>

          <div className="p-3 rounded-lg bg-slate-900/60 border border-white/5">
            <span className="text-xs text-slate-400">{t.movementScore}</span>
            <div className="text-2xl font-bold text-purple-400 font-mono mt-1">
              {metrics.movementScore.toFixed(0)}
            </div>
            <span className="text-[10px] text-slate-500">{metrics.movingShotsPercentage.toFixed(0)}% harakat</span>
          </div>
        </div>

        {/* Telemetry Observations */}
        <div className="p-4 rounded-lg bg-slate-900/80 border border-white/5 flex flex-col gap-2">
          <div className="text-xs font-semibold uppercase text-slate-400 tracking-wider">
            Telemetriya xulosalari
          </div>
          <div className="flex flex-col gap-1.5 text-sm text-slate-300">
            <div className="flex items-center gap-2">
              <Crosshair className="w-4 h-4 text-sky-400" />
              <span>
                Nishon og'ishi (Aim Bias):{' '}
                <strong className="text-slate-100">{metrics.directionBias}</strong>
              </span>
            </div>
            {metrics.movingShotsPercentage > 15 && (
              <div className="flex items-center gap-2 text-amber-400 text-xs">
                <AlertTriangle className="w-4 h-4" />
                <span>
                  {metrics.movingShotsPercentage.toFixed(0)}% {t.movingMissesNotice}
                </span>
              </div>
            )}
          </div>
        </div>

        {/* Coach Recommendation */}
        {recommendation && (
          <div className="p-4 rounded-lg bg-sky-950/40 border border-sky-500/30 flex flex-col gap-2">
            <div className="text-xs font-semibold text-sky-400 flex items-center gap-1.5">
              <Award className="w-3.5 h-3.5" />
              {t.recommendation}
            </div>
            <div className="font-bold text-slate-100 text-sm">
              {recommendation.title}
            </div>
            <p className="text-xs text-slate-300">
              {recommendation.observation}
            </p>
            <div className="text-xs text-slate-400">
              <strong>Tavsiya: </strong> {recommendation.suggestedDrill} mashqi (
              {recommendation.suggestedDurationMinutes} daqiqa).
            </div>
          </div>
        )}

        {/* Return to Work Button */}
        <div className="flex justify-end pt-2">
          <button onClick={onClose} className="btn-primary">
            <span>Ishga qaytish</span>
            <ArrowRight className="w-4 h-4" />
          </button>
        </div>
      </div>
    </div>
  );
};
