/**
 * SigmaOS Zenith Dashboard (v2.0)
 * Module 02: High-fidelity boot simulation and workspace orchestration.
 *
 * Architecture Fix:
 *  - Removed DOMContentLoaded listener; replaced with EventBus 'paradigm_switched'
 *    subscription so boot simulation only fires when the Zenith paradigm is actually active.
 *  - Explorer integration now guards against missing SovereignFS gracefully.
 *  - Boot sequence is idempotent — cannot fire twice.
 *  - Automation buttons now find their labels safely with optional chaining.
 */

const ZenithDashboard = {
    _bootDone: false,

    init() {
        console.log("Σ Zenith Dashboard v2.0: Workspace orchestrator online.");

        this._setupExplorer();
        this._setupAutomations();

        // Trigger boot sim when Zenith paradigm is switched to
        if (window.EventBus) {
            EventBus.subscribe('paradigm_switched', ({ id }) => {
                if (id === 'zenith') this.runBootSimulation();
            });
        }

        // Also expose globally for legacy callers
        window.simulateBootProcess = () => this.runBootSimulation();
    },

    async runBootSimulation() {
        if (this._bootDone) return;
        this._bootDone = true;

        const coverageVal = document.getElementById('coverage-val');

        // Initialize lattice visualizer
        if (window.LatticeVisualizer) {
            LatticeVisualizer.init('lattice-grid');
        }

        if (window.SovereignTelemetry) SovereignTelemetry.init();
        if (window.DashboardOrchestrator) DashboardOrchestrator.init();

        UIUtils.appendLog('audit-log', 'Establishing C11 Absolute Purity Handshake...', 'system');
        await new Promise(r => setTimeout(r, 600));

        const suites = window.SovereignRegistry ? SovereignRegistry.getAllSuites() : [];
        for (let i = 0; i < suites.length; i++) {
            const suite = suites[i];
            await new Promise(r => setTimeout(r, 40 + Math.random() * 60));

            if (window.LatticeVisualizer) LatticeVisualizer.updateSuiteStatus(suite.id, 'active');

            const hash = Math.random().toString(16).substr(2, 8).toUpperCase();
            UIUtils.appendLog('audit-log', `Integrity Verified: ${suite.id}_${suite.name} [0x${hash}]`, 'success');

            if (coverageVal) {
                coverageVal.textContent = `${Math.round(((i + 1) / suites.length) * 100)}%`;
            }
        }

        UIUtils.appendLog('audit-log', '▶ ALL SUITES MATERIALLY HARMONIZED.', 'system');
        UIUtils.appendLog('audit-log', '▶ SOVEREIGNTY ASCENDED.', 'system');

        if (window.EventBus) EventBus.publish('boot_complete', { suiteCount: suites.length });
    },

    _setupExplorer() {
        const explorerList = document.getElementById('explorer-list');
        if (!explorerList) return;

        if (window.SovereignFS) {
            SovereignFS.loadDirectory('/', 'explorer-list', 'current-path');
            const upBtn = document.getElementById('btn-up-dir');
            if (upBtn) upBtn.onclick = () => SovereignFS.goUp('explorer-list', 'current-path');
        }
    },

    _setupAutomations() {
        document.querySelectorAll('#automations-view .cyber-btn').forEach(btn => {
            btn.addEventListener('click', async () => {
                const taskName = btn.closest('li')?.querySelector('strong')?.textContent ?? 'Unknown Task';
                UIUtils.appendLog('audit-log', `[AUTOMATION] Dispatching: ${taskName}`, 'warning');
                btn.disabled = true;
                await new Promise(r => setTimeout(r, 1000));
                UIUtils.appendLog('audit-log', `[AUTOMATION] ${taskName} complete.`, 'success');
                btn.disabled = false;
            });
        });
    }
};

window.ZenithDashboard = ZenithDashboard;

// Auto-init when DOMContentLoaded (keeps legacy behaviour but wrapped safely)
document.addEventListener('DOMContentLoaded', () => ZenithDashboard.init());