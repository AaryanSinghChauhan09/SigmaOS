// theme.js — ThemeProvider
// Reads user_preferences and injects CSS custom properties onto <html>.
// Called on every preference change for instant live preview.

import { userPreferences } from './db.js';

const PALETTES = {
  'phosphor-green': { primary: '#00ffc3', accent: '#06b6d4', secondary: '#f59e0b' },
  'ocean':          { primary: '#38bdf8', accent: '#818cf8', secondary: '#34d399' },
  'amber':          { primary: '#f59e0b', accent: '#f97316', secondary: '#a3e635' },
  'monochrome':     { primary: '#e2e8f0', accent: '#94a3b8', secondary: '#64748b' },
  'custom':         { primary: null, accent: null, secondary: null },
};

const DENSITY = {
  compact:      { spacing: '0.6', fontScale: '0.88', radius: '4px' },
  comfortable:  { spacing: '1.0', fontScale: '1.0',  radius: '8px' },
  spacious:     { spacing: '1.4', fontScale: '1.05', radius: '12px' },
};

/**
 * Apply preferences object to CSS custom properties on <html>.
 * @param {object} prefs  user_preferences row
 */
export function applyTheme(prefs) {
  const root = document.documentElement;

  // Mode
  root.setAttribute('data-mode', prefs.theme_mode || 'dark');

  // Accent colour (explicit hex takes priority over palette)
  const accent = prefs.accent_color || '#06b6d4';
  root.style.setProperty('--color-accent', accent);
  root.style.setProperty('--color-accent-rgb', hexToRgb(accent));

  // Density
  const d = DENSITY[prefs.ui_density] || DENSITY.comfortable;
  root.style.setProperty('--spacing-scale', d.spacing);
  root.style.setProperty('--font-scale',    d.fontScale);
  root.style.setProperty('--radius',        d.radius);
}

/**
 * Bootstrap: load prefs from DB and apply immediately.
 */
export async function initTheme(userId) {
  if (!userId) return;
  const { data } = await userPreferences.get(userId);
  if (data) applyTheme(data);
}

function hexToRgb(hex) {
  const r = parseInt(hex.slice(1,3),16);
  const g = parseInt(hex.slice(3,5),16);
  const b = parseInt(hex.slice(5,7),16);
  return `${r} ${g} ${b}`;
}

export { PALETTES, DENSITY };
