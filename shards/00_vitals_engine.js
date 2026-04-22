/**
 * SigmaOS Sovereign Vitals Engine (v2.0)
 * Module 00: Real-time system health orchestration and HUD binding.
 *
 * Architecture Improvements:
 *  - Heartbeat now publishes richer payload (stability, entropy, shards, cpu, mem).
 *  - Subscribes to 'hud_update' from AnalyticsEngine to update DOM vitals cards.
 *  - Threshold monitoring: triggers emergency neutralization with hysteresis
 *    (only fires once per critical period, not on every tick).
 *  - updateVitalsHUD() binds live data to all .vitals-value elements by data-metric.
 */

const VitalsEngine = {
    _criticalActive: false,

    init() {
        console.log("Σ Vitals Engine v2.0: Multi-metric telemetry orchestration online.");
        this.startHeartbeat();

        // Bind to AnalyticsEngine HUD updates
        if (window.EventBus) {
            EventBus.subscribe('hud_update', (summary) => this.updateVitalsHUD(summary));
        }
    },

    startHeartbeat() {
        setInterval(() => {
            const health = this.calculateHealth();
            const payload = {
                health,
                shards: window.VitalsService ? VitalsService.activeShards : 0,
                cpu: window.AnalyticsEngine
                    ? parseFloat(AnalyticsEngine.getSummary().cpu) : 0,
                memKB: window.AnalyticsEngine
                    ? AnalyticsEngine.getSummary().memKB : 0,
                timestamp: Date.now()
            };

            if (window.EventBus) EventBus.publish('vitals_pulse', payload);

            // Threshold guard with hysteresis
            if (payload.cpu > 90 && !this._criticalActive) {
                this._criticalActive = true;
                this.triggerEmergencyNeutralization();
                setTimeout(() => { this._criticalActive = false; }, 30000);
            }
        }, 3000);
    },

    calculateHealth() {
        if (window.VitalsService) {
            return parseFloat(VitalsService.computeHealth().stability);
        }
        return parseFloat((95 + Math.random() * 5).toFixed(3));
    },

    updateVitalsHUD(summary) {
        const bindings = {
            'cpu_usage'     : `${summary.cpu}%`,
            'memory_usage'  : window.StringEngine
                ? StringEngine.formatBytes(summary.memKB * 1024) : `${summary.memKB}KB`,
            'process_count' : String(summary.processCount),
            'open_windows'  : String(summary.openWindows),
        };

        // Bind to elements with data-metric attribute
        Object.entries(bindings).forEach(([metric, val]) => {
            document.querySelectorAll(`[data-metric="${metric}"]`)
                .forEach(el => { el.textContent = val; });
        });

        // Also update legacy .vitals-value elements by class sequence
        const cards = document.querySelectorAll('.vitals-card');
        const values = [summary.cpu + '%', bindings.memory_usage, summary.processCount, summary.openWindows];
        cards.forEach((card, i) => {
            const vEl = card.querySelector('.vitals-value');
            if (vEl && values[i] !== undefined) vEl.textContent = values[i];
        });
    },

    triggerEmergencyNeutralization() {
        UIUtils.appendLog('audit-log', 'VITALS: CRITICAL CPU LOAD. Initiating emergency shard sweep...', 'danger');
        if (window.TaskManager) TaskManager.neutralizeNonEssential();
        if (window.Notifications) Notifications.push('Emergency: Low-priority shards neutralized.', 'danger');
    }
};

window.VitalsEngine = VitalsEngine;
