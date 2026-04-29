/**
 * Σ Sovereign Settings Panel (v2.0)
 * ─────────────────────────────────────────────────────────────────────────────
 * Full-featured personalization panel rendered into the GUI.
 * All changes propagate through SigmaConfigBridge → sigma_config.json → CLI.
 */

class SovereignSettingsPanel {
    constructor() {
        this.init();
    }

    init() {
        // Hook into vertical tab settings icon
        const settingsTab = document.querySelector('.v-tab-settings');
        if (settingsTab) {
            settingsTab.addEventListener('click', () => this.show());
        }
        console.log('Σ://UI> Settings Panel Ready.');
    }

    show() {
        let panel = document.getElementById('settings-panel-overlay');
        if (panel) { panel.classList.toggle('hidden'); return; }

        panel = document.createElement('div');
        panel.id = 'settings-panel-overlay';
        panel.className = 'settings-overlay glass-panel';
        panel.innerHTML = this._buildHTML();
        document.body.appendChild(panel);
        this._bindEvents(panel);
        this._loadCurrentValues(panel);
    }

    _buildHTML() {
        return `
        <div class="settings-header">
            <div class="logo-container"><div class="sigma-glyph">⚙</div>
                <div class="title-wrapper"><h2 style="margin:0;font-size:14px;">SOVEREIGN SETTINGS</h2>
                <span class="version-badge">GUI ↔ CLI SYNCED</span></div>
            </div>
            <button class="cyber-btn small-btn" id="btn-close-settings">✕ CLOSE</button>
        </div>

        <div class="settings-body">

            <!-- ── Profile Section ───────────────────────────────────── -->
            <div class="settings-section">
                <h3 class="segment-title highlight-cyan">USER PROFILE</h3>
                <div class="settings-row">
                    <label>Username</label>
                    <input id="cfg-username" class="cli-input-box settings-input" type="text" placeholder="Ʃ_ZENITH" />
                </div>
                <div class="settings-row">
                    <label>Active Profile</label>
                    <select id="cfg-profile" class="ai-select" title="Profile">
                        <option value="default">Default</option>
                        <option value="developer">Developer</option>
                        <option value="secure">Secure</option>
                        <option value="minimal">Minimal</option>
                    </select>
                </div>
            </div>

            <!-- ── Appearance Section ────────────────────────────────── -->
            <div class="settings-section">
                <h3 class="segment-title highlight-magenta">APPEARANCE</h3>
                <div class="settings-row">
                    <label>Theme Preset</label>
                    <select id="cfg-theme" class="ai-select" title="Theme">
                        <option value="MATRIX">Matrix (Default)</option>
                        <option value="GHOST_MICA">Ghost Mica</option>
                        <option value="SOVEREIGN_GOLD">Sovereign Gold</option>
                    </select>
                </div>
                <div class="settings-row">
                    <label>Accent Color</label>
                    <input id="cfg-accent" type="color" class="settings-color-picker" value="#00f0ff" />
                </div>
                <div class="settings-row">
                    <label>Blur (px): <span id="blur-val">25</span></label>
                    <input id="cfg-blur" type="range" min="0" max="60" value="25" class="settings-slider" />
                </div>
                <div class="settings-row">
                    <label>Opacity: <span id="opacity-val">0.6</span></label>
                    <input id="cfg-opacity" type="range" min="0.1" max="1.0" step="0.05" value="0.6" class="settings-slider" />
                </div>
                <div class="settings-row">
                    <label>Font Scale: <span id="font-val">1.0</span></label>
                    <input id="cfg-fontScale" type="range" min="0.8" max="1.4" step="0.05" value="1.0" class="settings-slider" />
                </div>
                <div class="settings-row">
                    <label>Color Mode</label>
                    <select id="cfg-mode" class="ai-select" title="Color Mode">
                        <option value="DARK">Dark (Default)</option>
                        <option value="LIGHT">Light</option>
                    </select>
                </div>
            </div>

            <!-- ── System Section ────────────────────────────────────── -->
            <div class="settings-section">
                <h3 class="segment-title highlight-cyan">SYSTEM</h3>
                <div class="settings-row">
                    <label>Privacy Shield</label>
                    <label class="toggle-switch">
                        <input id="cfg-privacyShield" type="checkbox" checked />
                        <span class="toggle-slider"></span>
                    </label>
                </div>
                <div class="settings-row">
                    <label>Auto-Sync with GitHub</label>
                    <label class="toggle-switch">
                        <input id="cfg-auto_sync" type="checkbox" />
                        <span class="toggle-slider"></span>
                    </label>
                </div>
                <div class="settings-row">
                    <label>Sync Interval (s)</label>
                    <input id="cfg-sync_interval" type="number" class="cli-input-box settings-input" value="300" min="30" />
                </div>
            </div>

            <!-- ── CLI Reference ─────────────────────────────────────── -->
            <div class="settings-section">
                <h3 class="segment-title" style="color:#888">CLI EQUIVALENT</h3>
                <div class="cli-reference-box">
                    <code id="cli-reference-output">sigmactl set theme MATRIX</code>
                </div>
                <p class="card-desc-tiny" style="margin-top:6px;">
                    All settings here map 1:1 to <strong>sigmactl set &lt;key&gt; &lt;value&gt;</strong>
                </p>
            </div>

            <div class="settings-footer">
                <button class="cyber-btn small-btn" id="btn-save-settings">SAVE & APPLY</button>
                <button class="cyber-btn small-btn secondary" id="btn-reset-settings">RESET DEFAULTS</button>
            </div>
        </div>`;
    }

    _bindEvents(panel) {
        panel.querySelector('#btn-close-settings').onclick = () => panel.classList.add('hidden');

        // Live preview: sliders update labels + CSS instantly
        const sliders = [
            { id: 'cfg-blur',      label: 'blur-val',    prop: '--mica-blur',    fmt: v => `${v}px`,    cfg: 'blur' },
            { id: 'cfg-opacity',   label: 'opacity-val', prop: '--mica-opacity', fmt: v => v,           cfg: 'opacity' },
            { id: 'cfg-fontScale', label: 'font-val',    prop: null,             fmt: v => v,           cfg: 'fontScale' },
        ];
        sliders.forEach(({ id, label, prop, fmt, cfg }) => {
            const el = panel.querySelector(`#${id}`);
            const lb = panel.querySelector(`#${label}`);
            el?.addEventListener('input', () => {
                const v = el.value;
                if (lb) lb.textContent = v;
                if (prop) document.documentElement.style.setProperty(prop, fmt(v));
                this._updateCliRef(panel, cfg, v);
            });
        });

        panel.querySelector('#cfg-theme')?.addEventListener('change', e => {
            window.theme?.applyTheme(e.target.value);
            this._updateCliRef(panel, 'theme', e.target.value);
        });

        panel.querySelector('#cfg-accent')?.addEventListener('input', e => {
            document.documentElement.style.setProperty('--acc-cyan', e.target.value);
            this._updateCliRef(panel, 'accent', e.target.value);
        });

        panel.querySelector('#btn-save-settings').onclick = () => this._save(panel);
        panel.querySelector('#btn-reset-settings').onclick = () => this._reset(panel);
    }

    _updateCliRef(panel, key, value) {
        const ref = panel.querySelector('#cli-reference-output');
        if (ref) ref.textContent = `sigmactl set ${key} ${value}`;
    }

    _loadCurrentValues(panel) {
        const cfg = window.sigmaConfig?.get() || JSON.parse(localStorage.getItem('sigma_settings') || '{}');
        const set = (id, val) => {
            const el = panel.querySelector(`#${id}`);
            if (!el || val === undefined) return;
            if (el.type === 'checkbox') el.checked = !!val;
            else el.value = val;
        };
        set('cfg-username', cfg.username);
        set('cfg-profile', cfg.profile);
        set('cfg-theme', cfg.theme);
        set('cfg-accent', cfg.accent);
        set('cfg-blur', cfg.blur);
        set('cfg-opacity', cfg.opacity);
        set('cfg-fontScale', cfg.fontScale);
        set('cfg-mode', cfg.mode);
        set('cfg-privacyShield', cfg.privacyShield);
        set('cfg-auto_sync', cfg.auto_sync);
        set('cfg-sync_interval', cfg.sync_interval);
        ['blur-val','opacity-val','font-val'].forEach(id => {
            const src = id.replace('-val','');
            const el = panel.querySelector(`#${id}`);
            const src_el = panel.querySelector(`#cfg-${src}`);
            if (el && src_el) el.textContent = src_el.value;
        });
    }

    _save(panel) {
        const fields = [
            'username','profile','theme','accent','blur','opacity',
            'fontScale','mode','privacyShield','auto_sync','sync_interval'
        ];
        fields.forEach(key => {
            const el = panel.querySelector(`#cfg-${key}`);
            if (!el) return;
            const value = el.type === 'checkbox' ? el.checked : el.value;
            window.sigmaConfig?.set(key, value);
            window.settings?.set(key, value);
        });
        // Apply theme
        const themeEl = panel.querySelector('#cfg-theme');
        if (themeEl) window.theme?.applyTheme(themeEl.value);

        if (window.zenith?.taskbar) window.zenith.taskbar.notify('SETTINGS SAVED & APPLIED', 'OPTIMAL');
        console.log('Σ://UI> Settings saved and propagated to CLI config.');
    }

    _reset(panel) {
        const defaults = { theme: 'MATRIX', accent: '#00f0ff', blur: 25, opacity: 0.6, fontScale: 1.0, mode: 'DARK' };
        Object.entries(defaults).forEach(([k, v]) => {
            const el = panel.querySelector(`#cfg-${k}`);
            if (el) el.value = v;
            window.sigmaConfig?.set(k, v);
        });
        window.theme?.applyTheme('MATRIX');
        if (window.zenith?.taskbar) window.zenith.taskbar.notify('SETTINGS RESET TO DEFAULTS', 'WARN');
    }
}

window.SovereignSettingsPanel = SovereignSettingsPanel;
window.settingsPanel = new SovereignSettingsPanel();
