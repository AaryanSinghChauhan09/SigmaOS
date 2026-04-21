/**
 * SigmaOS Sovereign Analytics Engine (v2.0)
 * Module 05: Real-time performance analysis with rolling histograms, trend detection,
 *             and full ProcessManager integration for resource accounting.
 *
 * Architecture Improvements:
 *  - Subscribes to process_registered / process_terminated events to track live process count.
 *  - Tracks memory, CPU, open windows and process count - not just CPU.
 *  - Publishes 'hud_update' event for HUD widgets to consume.
 *  - Exposes getSummary() for a one-shot dashboard snapshot.
 */

const AnalyticsEngine = {
    history: [],
    MAX_HISTORY: 200,
    processCount: 0,

    init() {
        console.log("Σ Analytics Engine v2.0: Multi-metric performance tracking active.");

        // Subscribe to process events for accurate tracking
        if (window.EventBus) {
            EventBus.subscribe('process_registered', () => this.processCount++);
            EventBus.subscribe('process_terminated', () => this.processCount = Math.max(0, this.processCount - 1));
        }

        // Periodic multi-metric sampling
        setInterval(() => this._sample(), 4000);
    },

    _sample() {
        const cpu = this._getCPU();
        const mem = this._getMemKB();
        const winCount = window.ZenithWindowManager
            ? ZenithWindowManager.registry.size : 0;

        this.logMetrics('cpu_usage', cpu);
        this.logMetrics('mem_usage_kb', mem);
        this.logMetrics('process_count', this.processCount);
        this.logMetrics('open_windows', winCount);

        if (window.EventBus) {
            EventBus.publish('hud_update', this.getSummary());
        }

        // Threshold alerts
        if (cpu > 85) {
            UIUtils.appendLog('audit-log', `[ANALYTICS] CRITICAL: CPU at ${cpu.toFixed(1)}%.`, 'danger');
            if (window.EventBus) EventBus.publish('critical_load', { cpu });
        }
    },

    _getCPU() {
        // Use ProcessManager if available for more realistic accounting
        if (window.ProcessManager) {
            return Math.min(99, parseFloat(ProcessManager.getTotalCPU()) + 5 + Math.random() * 5);
        }
        return 10 + Math.random() * 20;
    },

    _getMemKB() {
        if (window.ProcessManager) {
            return ProcessManager.getTotalMemKB();
        }
        return 4096 + Math.round(Math.random() * 1024);
    },

    logMetrics(metric, value) {
        const entry = { timestamp: Date.now(), metric, value };
        this.history.push(entry);
        if (this.history.length > this.MAX_HISTORY) this.history.shift();
        if (window.EventBus) EventBus.publish('analytics_update', entry);
    },

    getTrend(metric) {
        const filtered = this.history.filter(e => e.metric === metric);
        if (filtered.length < 2) return 0;
        return filtered[filtered.length - 1].value - filtered[filtered.length - 2].value;
    },

    getSummary() {
        const latest = metric => {
            const entries = this.history.filter(e => e.metric === metric);
            return entries.length ? entries[entries.length - 1].value : 0;
        };
        return {
            cpu: latest('cpu_usage').toFixed(1),
            memKB: latest('mem_usage_kb'),
            processCount: this.processCount,
            openWindows: latest('open_windows'),
            cpuTrend: this.getTrend('cpu_usage').toFixed(1),
        };
    }
};

window.AnalyticsEngine = AnalyticsEngine;
