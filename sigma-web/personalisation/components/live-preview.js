// live-preview.js — Mock OS chrome that reflects current theme settings
export function renderLivePreview(container, prefs) {
  const accent = prefs?.accent_color || '#06b6d4';
  const mode   = prefs?.theme_mode  || 'dark';
  const wp     = prefs?.wallpaper_id || 'phosphor-grid';

  const bgMap = {
    'phosphor-grid':  'linear-gradient(135deg, #0a1628 0%, #0d1f3c 100%)',
    'deep-space':     'linear-gradient(135deg, #05070f 0%, #0a0e1a 100%)',
    'aurora':         'linear-gradient(135deg, #0d1117 0%, #1a2744 50%, #0d2b1a 100%)',
    'circuit':        'linear-gradient(135deg, #0a0e1a 0%, #111827 100%)',
    'void':           '#0a0a0f',
    'minimal-light':  'linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%)',
  };
  const bg = bgMap[wp] || bgMap['phosphor-grid'];

  container.innerHTML = `
    <div class="preview-frame" style="background:${bg}">
      <!-- Top bar -->
      <div class="preview-topbar" style="--preview-accent:${accent}">
        <div class="preview-topbar-left">
          <span class="preview-os-logo">Σ</span>
          <span class="preview-menu-item">File</span>
          <span class="preview-menu-item">Edit</span>
          <span class="preview-menu-item">View</span>
        </div>
        <div class="preview-topbar-center">
          <div class="preview-search">⌕ Search SigmaOS…</div>
        </div>
        <div class="preview-topbar-right">
          <span class="preview-status-dot" style="background:${accent}"></span>
          <span class="preview-clock">17:15</span>
          <span class="preview-tray-icon">📶</span>
          <span class="preview-tray-icon">🔋</span>
        </div>
      </div>

      <!-- Window -->
      <div class="preview-window glass-panel">
        <div class="preview-titlebar">
          <div class="preview-dots">
            <span class="preview-dot preview-dot-close"></span>
            <span class="preview-dot preview-dot-min"></span>
            <span class="preview-dot preview-dot-max" style="background:${accent}"></span>
          </div>
          <span class="preview-wintitle">sigma — bash</span>
        </div>
        <div class="preview-terminal">
          <div class="preview-line">
            <span class="preview-prompt" style="color:${accent}">σ</span>
            <span class="preview-cmd"> sigma doctor</span>
          </div>
          <div class="preview-line preview-out">Rust toolchain: <span style="color:#22c55e">OK</span></div>
          <div class="preview-line preview-out">QEMU:          <span style="color:#22c55e">OK</span></div>
          <div class="preview-line preview-out">Sovereign:     <span style="color:${accent}">ENABLED</span></div>
          <div class="preview-line">
            <span class="preview-prompt" style="color:${accent}">σ</span>
            <span class="preview-cursor">_</span>
          </div>
        </div>
      </div>

      <!-- Dock -->
      <div class="preview-dock">
        ${['Σ','⚙','📁','🌐','🔒'].map((icon,i) => `
          <div class="preview-dock-item ${i===0?'active':''}"
               style="${i===0?`box-shadow:0 0 12px ${accent}55;border-color:${accent}`:''}">${icon}</div>
        `).join('')}
      </div>

      <div class="preview-label">Live Preview</div>
    </div>
  `;
}
