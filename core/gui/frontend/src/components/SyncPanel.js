/**
 * gui/frontend/src/components/SyncPanel.js
 * One-click GitHub sync with live log output
 */
import { sync } from '../api.js';

export function SyncPanel(mountPoint) {
    const container = document.getElementById(mountPoint);
    if (!container) return;

    let syncLog = [];

    function addLog(msg, type = 'info') {
        const ts = new Date().toLocaleTimeString();
        syncLog.push({ ts, msg, type });
        renderLog();
    }

    function renderLog() {
        const logEl = document.getElementById('sync-log');
        if (!logEl) return;
        logEl.innerHTML = syncLog.slice(-20).reverse().map(l => `
            <div class="log-line log-${l.type}">
                <span class="log-ts">${l.ts}</span>
                <span class="log-msg">${l.msg}</span>
            </div>`).join('');
    }

    container.innerHTML = `
    <div class="panel-section">
        <div class="sync-status-row">
            <span class="t-title highlight-cyan">⬡ GITHUB SYNC</span>
            <span id="sync-indicator" class="status-badge">IDLE</span>
        </div>
        <div class="panel-toolbar">
            <button class="cyber-btn" id="btn-sync-now">↑ SYNC NOW</button>
            <button class="cyber-btn secondary" id="btn-sync-clear">✕ CLEAR LOG</button>
        </div>
        <div id="sync-log" class="sync-log"></div>
    </div>`;

    document.getElementById('btn-sync-now')?.addEventListener('click', async () => {
        const indicator = document.getElementById('sync-indicator');
        if (indicator) { indicator.textContent = 'SYNCING...'; indicator.className = 'status-badge pulse-ring'; }
        addLog('Initiating GitHub sync...', 'info');

        const r = await sync();

        if (indicator) {
            indicator.textContent = r.ok ? 'UP TO DATE' : 'ERROR';
            indicator.className = `status-badge ${r.ok ? '' : 'error'}`;
        }
        addLog(r.data, r.ok ? 'ok' : 'error');
        window.sigmaNotify?.(r.ok ? 'GITHUB SYNC: OK' : `SYNC FAILED: ${r.data}`, r.ok ? 'OPTIMAL' : 'WARN');
    });

    document.getElementById('btn-sync-clear')?.addEventListener('click', () => {
        syncLog = []; renderLog();
    });
}
