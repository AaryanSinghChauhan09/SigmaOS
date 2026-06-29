// behavior.js — Behavior tab: startup apps, shortcuts, notifications, layout
import { functionalPrefs } from '../db.js';
import { showToast } from '../app.js';

const ALL_APPS = [
  { id: 'terminal',  label: 'Terminal',      icon: '⌨️' },
  { id: 'browser',   label: 'Browser',       icon: '🌐' },
  { id: 'files',     label: 'Files',         icon: '📁' },
  { id: 'settings',  label: 'Settings',      icon: '⚙️' },
  { id: 'sigma-top', label: 'sigma top',     icon: '📊' },
  { id: 'editor',    label: 'Code Editor',   icon: '📝' },
];

const ALL_APPS_NOTIF = ['terminal', 'browser', 'files', 'settings', 'sigma-top', 'editor'];

const SHORTCUTS_DEFAULT = {
  'Open Terminal':      'Ctrl+Alt+T',
  'Launch Browser':     'Ctrl+Alt+B',
  'Show Overview':      'Super',
  'Quick Search':       'Ctrl+Space',
  'Lock Screen':        'Super+L',
};

const LAYOUTS = [
  { id: 'default',   label: 'Default',    desc: 'Standard taskbar layout',          icon: '▤' },
  { id: 'developer', label: 'Developer',  desc: 'Terminal + browser side-by-side',  icon: '⊞' },
  { id: 'creative',  label: 'Creative',   desc: 'Max canvas single-app focus',      icon: '▣' },
  { id: 'minimal',   label: 'Minimal',    desc: 'Single app, no dock',              icon: '▢' },
];

export async function renderBehaviorPage(container, userId) {
  const { data: prefs } = await functionalPrefs.get(userId);
  let local = {
    startup_apps: [...(prefs.startup_apps || [])],
    notification_rules: { ...(prefs.notification_rules || {}) },
    keyboard_shortcuts: { ...SHORTCUTS_DEFAULT, ...(prefs.keyboard_shortcuts || {}) },
    workspace_layout: prefs.workspace_layout || 'default',
  };

  let capturingKey = null;

  container.innerHTML = `
    <div class="tab-content">
      <h2 class="tab-title">Behavior</h2>
      <p class="tab-subtitle">Configure startup, notifications, shortcuts, and workspace layout.</p>

      <!-- Startup Apps -->
      <section class="pref-section">
        <h3 class="pref-section-title">Startup Applications</h3>
        <p class="pref-hint">Selected apps launch automatically when SigmaOS boots.</p>
        <div class="startup-grid">
          ${ALL_APPS.map(app => `
            <label class="startup-item ${local.startup_apps.includes(app.id)?'active':''}">
              <input type="checkbox" class="sr-only startup-check"
                     value="${app.id}" ${local.startup_apps.includes(app.id)?'checked':''} />
              <span class="startup-icon">${app.icon}</span>
              <span class="startup-label">${app.label}</span>
              <span class="startup-check-mark">✓</span>
            </label>`).join('')}
        </div>
      </section>

      <!-- Notification Rules -->
      <section class="pref-section">
        <h3 class="pref-section-title">Notification Rules</h3>
        <div class="notif-table">
          <div class="notif-header">
            <span>App</span><span>Allow</span><span>Mute</span><span>Priority</span>
          </div>
          ${ALL_APPS.map(app => {
            const rule = local.notification_rules[app.id] || 'allow';
            return `<div class="notif-row">
              <span class="notif-app">${app.icon} ${app.label}</span>
              ${['allow','mute','priority'].map(r => `
                <label class="notif-radio-wrap">
                  <input type="radio" name="notif-${app.id}" value="${r}"
                         class="notif-radio" data-app="${app.id}"
                         ${rule===r?'checked':''} />
                  <span class="notif-radio-dot"></span>
                </label>`).join('')}
            </div>`;
          }).join('')}
        </div>
      </section>

      <!-- Keyboard Shortcuts -->
      <section class="pref-section">
        <h3 class="pref-section-title">Keyboard Shortcuts</h3>
        <p class="pref-hint">Click a shortcut to rebind it, then press your key combination.</p>
        <div class="shortcut-table">
          ${Object.entries(local.keyboard_shortcuts).map(([action, binding]) => `
            <div class="shortcut-row">
              <span class="shortcut-action">${action}</span>
              <button class="shortcut-key-btn" data-action="${action}">${binding}</button>
            </div>`).join('')}
        </div>
        <div class="shortcut-capture-hint" id="capture-hint" style="display:none">
          <span class="badge-accent">⌨ Press key combination… (Esc to cancel)</span>
        </div>
      </section>

      <!-- Workspace Layout -->
      <section class="pref-section">
        <h3 class="pref-section-title">Workspace Layout</h3>
        <div class="layout-grid">
          ${LAYOUTS.map(l => `
            <button class="layout-card ${local.workspace_layout===l.id?'active':''}"
                    data-layout="${l.id}">
              <span class="layout-icon">${l.icon}</span>
              <span class="layout-name">${l.label}</span>
              <span class="layout-desc">${l.desc}</span>
            </button>`).join('')}
        </div>
      </section>

      <div class="pref-actions">
        <button class="btn btn-primary" id="behavior-save">Save Behavior</button>
      </div>
    </div>
  `;

  // Startup checkboxes
  container.querySelectorAll('.startup-check').forEach(cb => {
    cb.addEventListener('change', () => {
      const label = cb.closest('.startup-item');
      if (cb.checked) {
        local.startup_apps.push(cb.value);
        label.classList.add('active');
      } else {
        local.startup_apps = local.startup_apps.filter(a => a !== cb.value);
        label.classList.remove('active');
      }
    });
  });

  // Notification radios
  container.querySelectorAll('.notif-radio').forEach(r => {
    r.addEventListener('change', () => {
      local.notification_rules[r.dataset.app] = r.value;
    });
  });

  // Keyboard shortcut capture
  container.querySelectorAll('.shortcut-key-btn').forEach(btn => {
    btn.addEventListener('click', () => {
      if (capturingKey) {
        document.querySelector(`[data-action="${capturingKey}"]`)?.classList.remove('capturing');
      }
      capturingKey = btn.dataset.action;
      btn.classList.add('capturing');
      btn.textContent = '…';
      document.getElementById('capture-hint').style.display = '';
    });
  });

  document.addEventListener('keydown', (e) => {
    if (!capturingKey) return;
    if (e.key === 'Escape') {
      const btn = container.querySelector(`[data-action="${capturingKey}"]`);
      if (btn) { btn.textContent = local.keyboard_shortcuts[capturingKey]; btn.classList.remove('capturing'); }
      capturingKey = null;
      document.getElementById('capture-hint').style.display = 'none';
      return;
    }
    e.preventDefault();
    const parts = [];
    if (e.ctrlKey)  parts.push('Ctrl');
    if (e.metaKey)  parts.push('Super');
    if (e.altKey)   parts.push('Alt');
    if (e.shiftKey) parts.push('Shift');
    const key = e.key === ' ' ? 'Space' : e.key.length === 1 ? e.key.toUpperCase() : e.key;
    if (!['Control','Meta','Alt','Shift'].includes(e.key)) parts.push(key);
    const binding = parts.join('+');
    local.keyboard_shortcuts[capturingKey] = binding;
    const btn = container.querySelector(`[data-action="${capturingKey}"]`);
    if (btn) { btn.textContent = binding; btn.classList.remove('capturing'); }
    capturingKey = null;
    document.getElementById('capture-hint').style.display = 'none';
  });

  // Layout picker
  container.querySelectorAll('.layout-card').forEach(card => {
    card.addEventListener('click', () => {
      local.workspace_layout = card.dataset.layout;
      container.querySelectorAll('.layout-card').forEach(c => c.classList.remove('active'));
      card.classList.add('active');
    });
  });

  // Save
  document.getElementById('behavior-save').addEventListener('click', async () => {
    await functionalPrefs.update(userId, local);
    showToast('Behavior saved!', 'success');
  });
}
