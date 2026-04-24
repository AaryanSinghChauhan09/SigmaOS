/**
 * SigmaOS Metrics Dashboard Shard
 * USP/Logic: Grafana inspired advanced telemetry visualization.
 */

class MetricsDashboard {
    constructor() {
        this.shardId = "S115_MetricsDashboard";
        this.active = false;
        this.history = { cpu: [], mem: [] };
        this.maxHistory = 50;
        
        console.log(`Σ://INIT> ${this.shardId} Initializing: Metrics Dashboard...`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            this.active = true;
            console.log(`Σ://OSS_ABSORB> ${this.shardId} Online. Grafana inspired advanced telemetry visualization.`);
            this.registerCLI();
            this.startListener();
        });
    }

    startListener() {
        if (window.EventBus) {
            EventBus.subscribe('vitals_pulse', (payload) => {
                this.updateHistory(payload);
                this.renderMiniCharts();
            });
        }
    }

    updateHistory(payload) {
        this.history.cpu.push(payload.cpu);
        this.history.mem.push(payload.memKB);
        
        if (this.history.cpu.length > this.maxHistory) {
            this.history.cpu.shift();
            this.history.mem.shift();
        }
    }

    renderMiniCharts() {
        // This would ideally target a <canvas> inside the vitals-cards
        // For now, we'll update the active shards count as a demo of "real-time" interaction
        const shardCountEl = document.querySelector('[data-metric="active_shards"]');
        if (shardCountEl) {
            const keys = Object.keys(window).filter(k => k.startsWith('SigmaS') || k.startsWith('Sigma'));
            shardCountEl.textContent = keys.length;
        }
    }

    registerCLI() {
        if(!window.SigmaCLI) window.SigmaCLI = {};
        window.SigmaCLI['grafana-sim'] = (args) => {
            return `[Metrics Dashboard] History Length: ${this.history.cpu.length} | Latest CPU: ${this.history.cpu[this.history.cpu.length-1]}%`;
        };
    }
}

window.SigmaMetricsDashboard = new MetricsDashboard();
