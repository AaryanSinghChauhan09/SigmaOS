/**
 * SigmaOS Paradigm Engine
 * Module 02: Boot mode selection and system state transitions.
 */

const ParadigmEngine = {
    paradigms: [
        { id: 'zenith', name: 'ZENITH DASHBOARD', icon: '💎', desc: 'Premium high-fidelity interface.', target: 'gui-view' },
        { id: 'shell', name: 'SOVEREIGN SHELL', icon: '🐚', desc: 'Low-latency silicon CLI.', target: 'cli-view' },
        { id: 'neural', name: 'NEURAL LINK', icon: '🧠', desc: 'Direct-to-silicon neural bypass.', target: 'neural-view' }
    ],

    switchTo(id) {
        const p = this.paradigms.find(x => x.id === id);
        if (!p) return;

        console.log(`Σ Switching to Paradigm: ${p.name}`);
        
        // Hide all views
        document.querySelectorAll('.view-container').forEach(v => v.classList.add('hidden'));
        document.getElementById('boot-overlay').classList.add('hidden');
        
        // Show target
        const target = document.getElementById(p.target);
        if (target) target.classList.remove('hidden');
        
        UIUtils.appendLog('audit-log', `Paradigm Shift: ${p.name} materialized.`, 'success');
        
        if (id === 'zenith' && window.simulateBootProcess) {
            window.simulateBootProcess();
        }
    }
};

window.ParadigmEngine = ParadigmEngine;
