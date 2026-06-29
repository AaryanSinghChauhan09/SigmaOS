// appearance.js — Appearance tab: theme, accent, wallpaper, density
import { userPreferences } from '../db.js';
import { applyTheme, PALETTES } from '../theme.js';
import { renderLivePreview } from './live-preview.js';
import { showToast } from '../app.js';

const WALLPAPERS = [
  { id: 'phosphor-grid', label: 'Phosphor Grid',  bg: 'linear-gradient(135deg,#0a1628,#0d1f3c)' },
  { id: 'deep-space',    label: 'Deep Space',      bg: 'linear-gradient(135deg,#05070f,#0a0e1a)' },
  { id: 'aurora',        label: 'Aurora',          bg: 'linear-gradient(135deg,#0d1117,#1a2744 50%,#0d2b1a)' },
  { id: 'circuit',       label: 'Circuit',         bg: 'linear-gradient(135deg,#0a0e1a,#111827)' },
  { id: 'void',          label: 'Void',            bg: '#0a0a0f' },
  { id: 'minimal-light', label: 'Minimal Light',   bg: 'linear-gradient(135deg,#f8fafc,#e2e8f0)' },
];

const ACCENT_PRESETS = [
  '#06b6d4','#00ffc3','#f59e0b','#818cf8','#f43f5e','#22c55e','#fb923c','#e2e8f0',
];

export async function renderAppearancePage(container, userId, previewContainer) {
  const { data: prefs } = await userPreferences.get(userId);
  let local = { ...prefs };

  function refresh() {
    applyTheme(local);
    renderLivePreview(previewContainer, local);
  }

  container.innerHTML = `
    <div class="tab-content">
      <h2 class="tab-title">Appearance</h2>
      <p class="tab-subtitle">Customise the look and feel of your SigmaOS environment.</p>

      <!-- Theme Mode -->
      <section class="pref-section">
        <h3 class="pref-section-title">Theme Mode</h3>
        <div class="mode-toggle-group" role="group" aria-label="Theme mode">
          ${['dark','light','auto'].map(m => `
            <button class="mode-btn ${local.theme_mode===m?'active':''}"
                    data-mode="${m}" id="mode-${m}">
              ${m==='dark'?'🌙':m==='light'?'☀️':'🖥️'} ${m.charAt(0).toUpperCase()+m.slice(1)}
            </button>`).join('')}
        </div>
        <p class="pref-hint">Auto syncs with your OS dark/light preference.</p>
      </section>

      <!-- Accent Colour -->
      <section class="pref-section">
        <h3 class="pref-section-title">Accent Colour</h3>
        <div class="accent-row">
          <div class="accent-swatches">
            ${ACCENT_PRESETS.map(c => `
              <button class="accent-swatch ${local.accent_color===c?'active':''}"
                      style="background:${c};box-shadow:${local.accent_color===c?`0 0 0 2px #fff,0 0 0 4px ${c}`:''}"
                      data-color="${c}" title="${c}" aria-label="Accent ${c}"></button>
            `).join('')}
          </div>
          <div class="accent-custom">
            <label class="form-label" for="accent-hex">Custom</label>
            <div class="accent-custom-row">
              <input type="color" id="accent-color-picker" value="${local.accent_color}"
                     class="color-picker-native" />
              <input type="text" id="accent-hex" class="form-input accent-hex-input"
                     value="${local.accent_color}" maxlength="7" placeholder="#06b6d4" />
            </div>
          </div>
        </div>
      </section>

      <!-- Wallpaper -->
      <section class="pref-section">
        <h3 class="pref-section-title">Wallpaper</h3>
        <div class="wallpaper-grid">
          ${WALLPAPERS.map(w => `
            <button class="wallpaper-item ${local.wallpaper_id===w.id?'active':''}"
                    data-wp="${w.id}" title="${w.label}" aria-label="${w.label}">
              <span class="wallpaper-thumb" style="background:${w.bg}"></span>
              <span class="wallpaper-label">${w.label}</span>
            </button>`).join('')}
        </div>
      </section>

      <!-- UI Density -->
      <section class="pref-section">
        <h3 class="pref-section-title">UI Density</h3>
        <div class="density-group" role="group">
          ${['compact','comfortable','spacious'].map(d => `
            <label class="density-option ${local.ui_density===d?'active':''}">
              <input type="radio" name="density" value="${d}"
                     ${local.ui_density===d?'checked':''} class="sr-only" />
              <span class="density-label">${d}</span>
              <span class="density-demo density-demo-${d}">
                <span></span><span></span><span></span>
              </span>
            </label>`).join('')}
        </div>
      </section>

      <div class="pref-actions">
        <button class="btn btn-primary" id="appearance-save">Save Appearance</button>
        <button class="btn btn-ghost"   id="appearance-reset">Reset to Defaults</button>
      </div>
    </div>
  `;

  refresh();

  // Mode toggle
  container.querySelectorAll('.mode-btn').forEach(btn => {
    btn.addEventListener('click', () => {
      local.theme_mode = btn.dataset.mode;
      container.querySelectorAll('.mode-btn').forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      refresh();
    });
  });

  // Accent swatches
  container.querySelectorAll('.accent-swatch').forEach(sw => {
    sw.addEventListener('click', () => {
      local.accent_color = sw.dataset.color;
      document.getElementById('accent-hex').value = sw.dataset.color;
      document.getElementById('accent-color-picker').value = sw.dataset.color;
      container.querySelectorAll('.accent-swatch').forEach(s => {
        s.classList.remove('active');
        s.style.boxShadow = '';
      });
      sw.classList.add('active');
      sw.style.boxShadow = `0 0 0 2px #fff,0 0 0 4px ${sw.dataset.color}`;
      refresh();
    });
  });

  // Custom hex input
  document.getElementById('accent-hex').addEventListener('input', (e) => {
    const val = e.target.value;
    if (/^#[0-9a-f]{6}$/i.test(val)) {
      local.accent_color = val;
      document.getElementById('accent-color-picker').value = val;
      refresh();
    }
  });

  document.getElementById('accent-color-picker').addEventListener('input', (e) => {
    local.accent_color = e.target.value;
    document.getElementById('accent-hex').value = e.target.value;
    refresh();
  });

  // Wallpaper
  container.querySelectorAll('.wallpaper-item').forEach(btn => {
    btn.addEventListener('click', () => {
      local.wallpaper_id = btn.dataset.wp;
      container.querySelectorAll('.wallpaper-item').forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      refresh();
    });
  });

  // Density
  container.querySelectorAll('input[name="density"]').forEach(radio => {
    radio.addEventListener('change', () => {
      local.ui_density = radio.value;
      container.querySelectorAll('.density-option').forEach(o => o.classList.remove('active'));
      radio.closest('.density-option').classList.add('active');
      refresh();
    });
  });

  // Save
  document.getElementById('appearance-save').addEventListener('click', async () => {
    await userPreferences.update(userId, local);
    applyTheme(local);
    showToast('Appearance saved!', 'success');
  });

  // Reset
  document.getElementById('appearance-reset').addEventListener('click', async () => {
    local = { ...userPreferences.DEFAULT };
    await userPreferences.update(userId, local);
    renderAppearancePage(container, userId, previewContainer);
    showToast('Reset to defaults.', 'info');
  });
}
