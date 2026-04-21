/**
 * SigmaOS Mission Control
 * Module 05: System-wide telemetry visualization and resource orchestration.
 */

const MissionControl = {
    init() {
        this.overlay = document.getElementById('mission-control-overlay');
        this.setupControls();
    },

    toggle() {
        if (!this.overlay) return;
        this.overlay.classList.toggle('hidden');
        if (!this.overlay.classList.contains('hidden')) {
            this.refreshStats();
            UIUtils.appendLog('audit-log', 'Mission Control: Initializing full-spectrum scan...', 'warning');
        }
    },

    setupControls() {
        const closeBtn = document.getElementById('btn-close-mission');
        if (closeBtn) closeBtn.onclick = () => this.toggle();

        // Paradigm shift integration
        document.querySelectorAll('.mission-card').forEach(card => {
            card.addEventListener('click', () => {
                const title = card.querySelector('h3').textContent;
                console.log(`Mission Control: Dispatching ${title}`);
                UIUtils.appendLog('audit-log', `MC: ${title} routine invoked.`, 'success');
            });
        });
    },

    refreshStats() {
        console.log("Σ Mission Control: Refreshing Sovereign Vitals...");
        // This will eventually integrate with SovereignTelemetry.js
        const stats = document.querySelectorAll('.mission-stat-val');
        stats.forEach(s => {
            if (s.textContent.includes('%')) s.textContent = (Math.random() * 5 + 95).toFixed(1) + '%';
        });
    }
};

window.MissionControl = MissionControl;
