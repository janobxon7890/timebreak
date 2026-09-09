import React, { useState } from 'react';
import {
  BreakSchedulerConfig,
  CrosshairConfig,
  CS2Profile,
  WeaponDefinition,
} from '@timebreak/shared-types';
import { translations } from '../i18n/translations.js';
import { ArrowLeft, Save, Check, Sliders, Shield, Globe } from 'lucide-react';

interface SettingsViewProps {
  onBack: () => void;
  profile: CS2Profile;
  onSaveProfile: (profile: CS2Profile) => void;
  activeWeaponId: string;
  onSelectWeapon: (weaponId: string) => void;
  availableWeapons: WeaponDefinition[];
  schedulerConfig: BreakSchedulerConfig;
  onSaveScheduler: (config: BreakSchedulerConfig) => void;
  crosshairConfig: CrosshairConfig;
  onSaveCrosshair: (config: CrosshairConfig) => void;
  lang: 'uz' | 'en';
  onSelectLang: (lang: 'uz' | 'en') => void;
}

export const SettingsView: React.FC<SettingsViewProps> = ({
  onBack,
  profile,
  onSaveProfile,
  activeWeaponId,
  onSelectWeapon,
  availableWeapons,
  schedulerConfig,
  onSaveScheduler,
  crosshairConfig,
  onSaveCrosshair,
  lang,
  onSelectLang,
}) => {
  const t = translations[lang];

  const [localProfile, setLocalProfile] = useState<CS2Profile>(profile);
  const [localScheduler, setLocalScheduler] = useState<BreakSchedulerConfig>(schedulerConfig);
  const [localCrosshair, setLocalCrosshair] = useState<CrosshairConfig>(crosshairConfig);
  const [savedToast, setSavedToast] = useState(false);

  // Derived CS2 metrics
  const edpi = localProfile.dpi * localProfile.sensitivity;
  const cm360 = (360 / (localProfile.sensitivity * 0.022) / localProfile.dpi) * 2.54;

  const handleSave = () => {
    onSaveProfile(localProfile);
    onSaveScheduler(localScheduler);
    onSaveCrosshair(localCrosshair);
    setSavedToast(true);
    setTimeout(() => setSavedToast(false), 2000);
  };

  return (
    <div className="flex flex-col gap-6 p-8 max-w-4xl mx-auto">
      {/* Header */}
      <div className="flex items-center justify-between">
        <button onClick={onBack} className="btn-secondary">
          <ArrowLeft className="w-4 h-4" />
          <span>Ortga</span>
        </button>

        <h1 className="text-xl font-bold text-slate-100 flex items-center gap-2">
          <Sliders className="w-5 h-5 text-sky-400" />
          {t.settings}
        </h1>

        <button onClick={handleSave} className="btn-primary">
          {savedToast ? <Check className="w-4 h-4 text-emerald-300" /> : <Save className="w-4 h-4" />}
          <span>{savedToast ? t.saved : t.saveSettings}</span>
        </button>
      </div>

      {/* Break Scheduler Section */}
      <div className="glass-panel p-6 flex flex-col gap-4">
        <h2 className="text-base font-bold text-slate-200 flex items-center gap-2 border-b border-white/5 pb-2">
          <Shield className="w-4 h-4 text-sky-400" />
          {t.breakSettings}
        </h2>

        <div className="grid grid-cols-2 gap-4">
          <div className="flex flex-col gap-1.5">
            <label className="text-xs text-slate-400 font-medium">
              {t.workIntervalMinutes}
            </label>
            <input
              type="number"
              min="1"
              max="120"
              value={localScheduler.workIntervalMinutes}
              onChange={(e) =>
                setLocalScheduler({
                  ...localScheduler,
                  workIntervalMinutes: parseInt(e.target.value) || 25,
                })
              }
              className="px-3 py-2 rounded-lg bg-slate-900 border border-slate-700 text-slate-100 font-mono text-sm focus:outline-none focus:border-sky-500"
            />
          </div>

          <div className="flex flex-col gap-1.5">
            <label className="text-xs text-slate-400 font-medium">
              {t.breakDurationMinutes}
            </label>
            <input
              type="number"
              min="1"
              max="30"
              value={localScheduler.breakDurationMinutes}
              onChange={(e) =>
                setLocalScheduler({
                  ...localScheduler,
                  breakDurationMinutes: parseInt(e.target.value) || 5,
                })
              }
              className="px-3 py-2 rounded-lg bg-slate-900 border border-slate-700 text-slate-100 font-mono text-sm focus:outline-none focus:border-sky-500"
            />
          </div>
        </div>

        <div className="flex items-center gap-3 pt-2">
          <input
            type="checkbox"
            id="autoStart"
            checked={localScheduler.autoStartBreak}
            onChange={(e) =>
              setLocalScheduler({
                ...localScheduler,
                autoStartBreak: e.target.checked,
              })
            }
            className="w-4 h-4 rounded bg-slate-900 border-slate-700 text-sky-500 focus:ring-0"
          />
          <label htmlFor="autoStart" className="text-sm text-slate-300">
            {t.autoStartBreak}
          </label>
        </div>
      </div>

      {/* CS2 Profile Section */}
      <div className="glass-panel p-6 flex flex-col gap-4">
        <h2 className="text-base font-bold text-slate-200 flex items-center gap-2 border-b border-white/5 pb-2">
          <Sliders className="w-4 h-4 text-rose-400" />
          {t.cs2Profile}
        </h2>

        <div className="grid grid-cols-3 gap-4">
          <div className="flex flex-col gap-1.5">
            <label className="text-xs text-slate-400 font-medium">{t.dpi}</label>
            <input
              type="number"
              min="100"
              max="16000"
              step="50"
              value={localProfile.dpi}
              onChange={(e) =>
                setLocalProfile({
                  ...localProfile,
                  dpi: parseFloat(e.target.value) || 800,
                })
              }
              className="px-3 py-2 rounded-lg bg-slate-900 border border-slate-700 text-slate-100 font-mono text-sm focus:outline-none focus:border-sky-500"
            />
          </div>

          <div className="flex flex-col gap-1.5">
            <label className="text-xs text-slate-400 font-medium">{t.sensitivity}</label>
            <input
              type="number"
              min="0.1"
              max="10.0"
              step="0.05"
              value={localProfile.sensitivity}
              onChange={(e) =>
                setLocalProfile({
                  ...localProfile,
                  sensitivity: parseFloat(e.target.value) || 1.0,
                })
              }
              className="px-3 py-2 rounded-lg bg-slate-900 border border-slate-700 text-slate-100 font-mono text-sm focus:outline-none focus:border-sky-500"
            />
          </div>

          <div className="flex flex-col gap-1.5">
            <label className="text-xs text-slate-400 font-medium">{t.zoomSensitivity}</label>
            <input
              type="number"
              min="0.1"
              max="3.0"
              step="0.1"
              value={localProfile.zoomSensitivity}
              onChange={(e) =>
                setLocalProfile({
                  ...localProfile,
                  zoomSensitivity: parseFloat(e.target.value) || 1.0,
                })
              }
              className="px-3 py-2 rounded-lg bg-slate-900 border border-slate-700 text-slate-100 font-mono text-sm focus:outline-none focus:border-sky-500"
            />
          </div>
        </div>

        <div className="grid grid-cols-2 gap-4 p-3 rounded-lg bg-slate-900/50 border border-white/5 font-mono text-xs">
          <div>
            <span className="text-slate-500">{t.edpi}: </span>
            <span className="text-sky-400 font-bold">{edpi.toFixed(0)}</span>
          </div>
          <div>
            <span className="text-slate-500">{t.cm360}: </span>
            <span className="text-emerald-400 font-bold">{cm360.toFixed(1)} cm</span>
          </div>
        </div>
      </div>

      {/* Weapon Selection */}
      <div className="glass-panel p-6 flex flex-col gap-4">
        <h2 className="text-base font-bold text-slate-200 border-b border-white/5 pb-2">
          {t.currentWeapon}
        </h2>

        <div className="grid grid-cols-5 gap-2.5">
          {availableWeapons.map((w) => {
            const isSelected = w.id === activeWeaponId;
            return (
              <button
                key={w.id}
                onClick={() => onSelectWeapon(w.id)}
                className={`p-3 rounded-lg border text-left transition-all ${
                  isSelected
                    ? 'bg-sky-500/20 border-sky-400 text-slate-100 shadow-md shadow-sky-500/20'
                    : 'bg-slate-900/50 border-slate-800 text-slate-400 hover:border-slate-700 hover:text-slate-200'
                }`}
              >
                <div className="font-bold text-xs font-mono">{w.displayName}</div>
                <div className="text-[10px] text-slate-500 mt-1">{w.category}</div>
              </button>
            );
          })}
        </div>

        <div className="pt-3 border-t border-white/5 flex items-center justify-between">
          <span className="text-xs text-slate-400">Krossxayr hajmi (Crosshair Size):</span>
          <input
            type="range"
            min="2"
            max="16"
            value={localCrosshair.size}
            onChange={(e) =>
              setLocalCrosshair({
                ...localCrosshair,
                size: parseInt(e.target.value) || 6,
              })
            }
            className="w-48 accent-sky-400 cursor-pointer"
          />
        </div>
      </div>

      {/* Language Section */}
      <div className="glass-panel p-6 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Globe className="w-5 h-5 text-sky-400" />
          <span className="text-sm font-medium text-slate-200">{t.language}</span>
        </div>

        <div className="flex gap-2">
          <button
            onClick={() => onSelectLang('uz')}
            className={`px-3 py-1.5 rounded-md text-xs font-bold font-mono transition-all ${
              lang === 'uz'
                ? 'bg-sky-500 text-white shadow-md'
                : 'bg-slate-800 text-slate-400 hover:text-slate-200'
            }`}
          >
            O'zbekcha
          </button>
          <button
            onClick={() => onSelectLang('en')}
            className={`px-3 py-1.5 rounded-md text-xs font-bold font-mono transition-all ${
              lang === 'en'
                ? 'bg-sky-500 text-white shadow-md'
                : 'bg-slate-800 text-slate-400 hover:text-slate-200'
            }`}
          >
            English
          </button>
        </div>
      </div>
    </div>
  );
};
