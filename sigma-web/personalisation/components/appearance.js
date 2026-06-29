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

// Workspace presets: named bundles of appearance prefs
const WORKSPACE_PRESETS_KEY = 'sigma_workspace_presets';
function loadPresets() {
  try { return JSON.parse(localStorage.getItem(WORKSPACE_PRESETS_KEY) || '{}'); }
  catch { return {}; }
}
function savePresets(p) { localStorage.setItem(WORKSPACE_PRESETS_KEY, JSON.stringify(p)); }

const BUILTIN_PRESETS = {
  Work:    { theme_mode:'dark',  accent_color:'#06b6d4', ui_density:'comfortable', wallpaper_id:'circuit'       },
  Gaming:  { theme_mode:'dark',  accent_color:'#00ffc3', ui_density:'compact',     wallpaper_id:'phosphor-grid' },
  Creative:{ theme_mode:'dark',  accent_color:'#818cf8', ui_density:'spacious',    wallpaper_id:'aurora'        },
  Focus:   { theme_mode:'light', accent_color:'#06b6d4', ui_density:'comfortable', wallpaper_id:'minimal-light' },
};

export async function renderAppearancePage(container, userId, previewContainer) {
  const { data: prefs } = await userPreferences.get(userId);
  let local = { ...prefs };

  function refresh() {
    applyTheme(local);
    renderLivePreview(previewContainer, local);
  }

  const allPresets = { ...BUILTIN_PRESETS, ...loadPresets() };

  container.innerHTML = `
    <div class="tab-content">
      <h2 class="tab-title">Appearance</h2>
      <p class="tab-subtitle">Customise the look and feel of your SigmaOS environment.</p>

      <!-- Workspace Presets -->
      <section class="pref-section">
        <h3 class="pref-section-title">Workspace Presets</h3>
        <p class="pref-hint">Save and switch complete appearance configurations.</p>
        <div class="presets-row">
          <div class="preset-chips" id="preset-chips">
            ${Object.keys({ ...BUILTIN_PRESETS, ...loadPresets() }).map(name => `
              <button class="preset-chip" data-preset="${name}">${name}</button>
            `).join('')}
          </div>
          <div class="preset-actions">
            <input class="form-input preset-name-input" id="preset-name-input"
                   placeholder="New preset name…" maxlength="24" />
            <button class="btn btn-ghost btn-sm" id="preset-save-btn">Save Current</button>
          </div>
        </div>
      </section>

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

  // Custom wallpaper upload slot
  const uploadInput = container.querySelector('#wallpaper-upload-input');
  if (uploadInput) {
    uploadInput.addEventListener('change', (e) => {
      const file = e.target.files[0];
      if (!file) return;
      const reader = new FileReader();
      reader.onload = (ev) => {
        // Store data-URL as custom wallpaper_id
        local.wallpaper_id = ev.target.result;
        container.querySelectorAll('.wallpaper-item').forEach(b => b.classList.remove('active'));
        const uploadBtn = container.querySelector('.wallpaper-upload-btn');
        if (uploadBtn) uploadBtn.classList.add('active');
        // Update live preview background directly
        const frame = previewContainer.querySelector('.preview-frame');
        if (frame) frame.style.background = `center/cover url("${ev.target.result}")`;
        showToast('Custom wallpaper applied!', 'success');
      };
      reader.readAsDataURL(file);
    });
  }

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

  // ── Workspace Presets ────────────────────────────────────────────────────
  container.querySelectorAll('.preset-chip').forEach(chip => {
    chip.addEventListener('click', () => {
      const name = chip.dataset.preset;
      const preset = { ...BUILTIN_PRESETS, ...loadPresets() }[name];
      if (!preset) return;
      Object.assign(local, preset);
      refresh();
      // Re-render so all controls reflect the loaded preset
      renderAppearancePage(container, userId, previewContainer);
      showToast(`Preset "${name}" applied.`, 'success');
    });
  });

  document.getElementById('preset-save-btn')?.addEventListener('click', async () => {
    const nameEl = document.getElementById('preset-name-input');
    const name = nameEl?.value.trim();
    if (!name) { showToast('Enter a preset name first.', 'error'); return; }
    const saved = loadPresets();
    saved[name] = {
      theme_mode:  local.theme_mode,
      accent_color: local.accent_color,
      ui_density:  local.ui_density,
      wallpaper_id: local.wallpaper_id,
    };
    savePresets(saved);
    await userPreferences.update(userId, local);
    showToast(`Preset "${name}" saved!`, 'success');
    if (nameEl) nameEl.value = '';
    // Re-render to show new chip
    renderAppearancePage(container, userId, previewContainer);
  });
}
