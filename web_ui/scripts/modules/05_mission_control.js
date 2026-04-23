/**
 * SigmaOS Mission Control (v2.0)
 * Module 05: System-wide telemetry visualization backed by live AnalyticsEngine data.
 *
 * Architecture Fix:
 *  - refreshStats() now reads from AnalyticsEngine.getSummary() instead of Math.random().
 *  - Mission card clicks publish EventBus 'request_paradigm_switch' instead of console.log.
 *  - Subscribes to 'hud_update' for automatic real-time refreshes without polling.
 */

const MissionControl = {
    init() {
        this.overlay = document.getElementById('mission-control-overlay');
        this.setupControls();

        // Auto-refresh when AnalyticsEngine publishes new data
        if (window.EventBus) {
            EventBus.subscribe('hud_update', () => {
                if (this.overlay && !this.overlay.classList.contains('hidden')) {
                    this.refreshStats();
                }
            });
        }
    },

    toggle() {
        if (!this.overlay) return;
        this.overlay.classList.toggle('hidden');
        if (!this.overlay.classList.contains('hidden')) {
            this.refreshStats();
            UIUtils.appendLog('audit-log', 'Mission Control: Full-spectrum lattice scan initiated.', 'warning');
        }
    },

    setupControls() {
        const closeBtn = document.getElementById('btn-close-mission');
        if (closeBtn) closeBtn.onclick = () => this.toggle();

        // Mission cards fire paradigm switches through EventBus
        document.querySelectorAll('.mission-card').forEach(card => {
            card.addEventListener('click', () => {
                const title = card.querySelector('h3')?.textContent || 'Unknown';
                const target = card.dataset.paradigm;
                UIUtils.appendLog('audit-log', `MC: Dispatching [${title}]`, 'success');
                if (target && window.EventBus) {
                    EventBus.publish('request_paradigm_switch', { id: target });
                }
            });
        });
    },

    refreshStats() {
        // Pull live data from AnalyticsEngine
        const summary = window.AnalyticsEngine
            ? AnalyticsEngine.getSummary()
            : { cpu: '--', memKB: 0, processCount: '--', openWindows: '--', cpuTrend: 0 };

        const statMap = {
            'mc-cpu':      `${summary.cpu}%`,
            'mc-mem':      window.StringEngine ? StringEngine.formatBytes(summary.memKB * 1024) : `${summary.memKB}KB`,
            'mc-procs':    String(summary.processCount),
            'mc-windows':  String(summary.openWindows),
            'mc-trend':    `${summary.cpuTrend > 0 ? '▲' : '▼'} ${Math.abs(summary.cpuTrend)}%`,
        };

        Object.entries(statMap).forEach(([id, val]) => {
            const el = document.getElementById(id);
            if (el) el.textContent = val;
        });

        // Legacy: update any .mission-stat-val elements still in DOM
        document.querySelectorAll('.mission-stat-val').forEach(s => {
            if (s.textContent.includes('%')) {
                s.textContent = `${summary.cpu}%`;
            }
        });
    }

    selfEvolve() {
        const mutations = [
            "Optimizing lattice resonance...",
            "Expanding semantic context...",
            "Hardening silicon primitives...",
            "Refining cross-kernel synthesis..."
        ];
        const mutation = mutations[Math.floor(Math.random() * mutations.length)];
        console.log(`Σ://EVOLUTION [${this.shardId}]> ${mutation}`);
        this.lastMutation = mutation;
    }
};

window.MissionControl = MissionControl;
