/**
 * SigmaOS Zenith Dashboard
 * Module 02: High-fidelity orchestration and lattice visualization.
 */

document.addEventListener("DOMContentLoaded", () => {
    const coverageVal = document.getElementById('coverage-val');
    
    // 1. Initialize Lattice Visualizer
    LatticeVisualizer.init('lattice-grid');

    // 2. Start Sub-Systems
    SovereignTelemetry.init();
    DashboardOrchestrator.init();

    // 3. Sovereign Boot Simulation
    const simulateBootProcess = async () => {
        UIUtils.appendLog('audit-log', 'Establishing C11 Absolute Purity Handshake...', 'system');
        await new Promise(r => setTimeout(r, 600));
        
        const suites = SovereignRegistry.getAllSuites();
        for (let i = 0; i < suites.length; i++) {
            const suite = suites[i];
            await new Promise(r => setTimeout(r, 40 + Math.random() * 80));
            
            LatticeVisualizer.updateSuiteStatus(suite.id, 'active');
            
            const hash = Math.random().toString(16).substr(2, 8);
            UIUtils.appendLog('audit-log', `Integrity Verified: ${suite.id}_${suite.name} (0x${hash})`, 'success');
            
            if (coverageVal) coverageVal.textContent = `${Math.round(((i + 1) / suites.length) * 100)}%`;
        }
        
        UIUtils.appendLog('audit-log', 'ALL 33 SUITES MATERIALLY HARMONIZED.', 'system');
        UIUtils.appendLog('audit-log', 'SOVEREIGNTY ASCENDED.', 'system');
    };

    // 5. Explorer Integration
    const explorerList = document.getElementById('explorer-list');
    if (explorerList) {
        SovereignFS.loadDirectory('/', 'explorer-list', 'current-path');
        document.getElementById('btn-up-dir').onclick = () => SovereignFS.goUp('explorer-list', 'current-path');
    }

    // 6. Automation Logic
    document.querySelectorAll('#automations-view .cyber-btn').forEach(btn => {
        btn.addEventListener('click', async () => {
            const taskName = btn.previousElementSibling.querySelector('strong').textContent;
            UIUtils.appendLog('audit-log', `[AUTOMATION] Dispatching: ${taskName}`, 'warning');
            btn.disabled = true;
            await new Promise(r => setTimeout(r, 1000));
            UIUtils.appendLog('audit-log', `[AUTOMATION] ${taskName} sequence complete.`, 'success');
            btn.disabled = false;
        });
    });

    // 7. Global Exposure
    window.simulateBootProcess = simulateBootProcess;
});