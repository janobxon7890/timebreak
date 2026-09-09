import { describe, it, expect } from 'vitest';
import { CS2Profile } from '@timebreak/shared-types';

describe('CS2 Profile Math', () => {
  it('calculates eDPI and cm/360 correctly', () => {
    const profile: CS2Profile = {
      dpi: 800,
      sensitivity: 1.0,
      zoomSensitivity: 1.0,
      eDpi: 800,
      cm360: 51.95,
      resolutionWidth: 1920,
      resolutionHeight: 1080,
    };

    const calculatedEDpi = profile.dpi * profile.sensitivity;
    expect(calculatedEDpi).toBe(800);

    const mYaw = 0.022;
    const countsPer360 = 360 / (profile.sensitivity * mYaw);
    const cm360 = (countsPer360 / profile.dpi) * 2.54;
    expect(Math.abs(cm360 - 51.95)).toBeLessThan(0.5);
  });
});
