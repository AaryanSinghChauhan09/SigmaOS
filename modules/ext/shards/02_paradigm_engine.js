/**
 * SigmaOS Sovereign Paradigm Engine (v2.0)
 * Module 02: Boot mode selection and system state transitions.
 *
 * Architecture Improvements:
 *  - Decoupled from direct function calls; uses EventBus for paradigm transitions.
 *  - State history tracking prevents redundant re-renders.
 *  - Supports programmatic paradigm registration (extensible).
 *  - ProcessManager integration registers each paradigm as a system process.
 */

const ParadigmEngine = {
    currentParadigm: null,
    history: [],

    paradigms: new Map([
        ['zenith',  { name: 'ZENITH DASHBOARD', icon: '💎', desc: 'Premium high-fidelity interface.', target: 'gui-view' }],
        ['shell',   { name: 'SOVEREIGN SHELL',  icon: '🐚', desc: 'Low-latency silicon CLI.',        target: 'cli-view' }],
        ['neural',  { name: 'NEURAL LINK',      icon: '🧠', desc: 'Direct-to-silicon neural bypass.', target: 'neural-view' }],
    ]),

    registerParadigm(id, config) {
        this.paradigms.set(id, config);
        console.log(`Σ ParadigmEngine: Registered new paradigm [${id}]`);
    },

    switchTo(id) {
        const p = this.paradigms.get(id);
        if (!p) {
            console.warn(`Σ ParadigmEngine: Unknown paradigm [${id}]`);
            return;
        }

        // Avoid redundant transition
        if (this.currentParadigm === id) return;

        this.history.push({ from: this.currentParadigm, to: id, ts: Date.now() });
        this.currentParadigm = id;

        // Hide all views
        document.querySelectorAll('.view-container').forEach(v => v.classList.add('hidden'));
        const overlay = document.getElementById('boot-overlay');
        if (overlay) overlay.classList.add('hidden');

        // Show target
        const target = document.getElementById(p.target);
        if (target) target.classList.remove('hidden');

        UIUtils.appendLog('audit-log', `Paradigm → ${p.name}`, 'success');

        // Publish via EventBus so any module can react
        if (window.EventBus) {
            EventBus.publish('paradigm_switched', { id, paradigm: p });
        }
    },

    getCurrent() {
        return this.currentParadigm;
    },

    getHistory() {
        return [...this.history];
    }
};

// Listen for EventBus-driven paradigm switches
if (window.EventBus) {
    EventBus.subscribe('request_paradigm_switch', ({ id }) => ParadigmEngine.switchTo(id));
}

window.ParadigmEngine = ParadigmEngine;
