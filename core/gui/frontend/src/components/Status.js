/**
 * gui/frontend/src/components/Status.js
 * System status panel — polls /status and /health every 10s
 */
import { getStatus, healthCheck } from '../api.js';

export function Status(mountPoint) {
    let lastStatus = null;
    let healthy = false;

    function render(text) {
        const container = document.getElementById(mountPoint);
        if (!container) return;
        const lines = (text || '').split('\n').filter(Boolean);
        container.innerHTML = `
        <div class="panel-section">
            <div class="status-header">
                <span class="t-title highlight-cyan">◉ SYSTEM STATUS</span>
                <span class="status-badge ${healthy ? 'pulse-ring' : 'error'}">${healthy ? 'SERVER ONLINE' : 'SERVER OFFLINE'}</span>
            </div>
            <div class="status-lines">
                ${lines.map(l => {
                    const isHeader = !l.startsWith(' ');
                    const css = isHeader ? 'status-section-header' : 'status-line';
                    return `<div class="${css}">${l}</div>`;
                }).join('')}
            </div>
            <div class="status-footer">
                <span class="muted">Last updated: ${new Date().toLocaleTimeString()}</span>
                <button class="cyber-btn small-btn secondary" id="btn-status-refresh">↺</button>
            </div>
        </div>`;
        document.getElementById('btn-status-refresh')?.addEventListener('click', refresh);
    }

    async function refresh() {
        const h = await healthCheck();
        healthy = h.ok;
        const r = await getStatus();
        lastStatus = r.data;
        render(r.data);
    }

    // Auto-refresh every 10s
    refresh();
    const interval = setInterval(refresh, 10000);
    return { refresh, stop: () => clearInterval(interval) };
}
