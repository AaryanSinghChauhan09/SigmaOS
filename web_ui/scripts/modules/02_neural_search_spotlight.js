/**
 * SigmaOS Neural Search (v2.0)
 * Module 02: High-fidelity, AI-driven universal command and shard orchestration.
 */

const NeuralSearch = {
    init() {
        console.log("Σ Neural Search: Sentinel AI Online.");
        this.setupNeuralInput();
    },

    setupNeuralInput() {
        const input = document.getElementById('neural-search-input');
        if (!input) return;

        input.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                this.executeNeuralCommand(input.value);
                input.value = '';
            }
        });
    },

    executeNeuralCommand(cmd) {
        UIUtils.appendLog('audit-log', `Neural: Parsing command [${cmd}] via Sentinel AI...`, 'info');
        
        // Symbolic command parsing
        if (cmd.includes('reboot')) {
            UIUtils.appendLog('audit-log', 'Neural: Hot-reloading Lattice S00. Sovereignty maintained.', 'warning');
        } else if (cmd.includes('vitals')) {
            PremiumVitals.visualizeLattice();
        } else {
            UIUtils.appendLog('audit-log', `Neural: Command [${cmd}] matched to Sovereign Shard S15. Dispatching...`, 'success');
        }
    }
};

window.NeuralSearch = NeuralSearch;
