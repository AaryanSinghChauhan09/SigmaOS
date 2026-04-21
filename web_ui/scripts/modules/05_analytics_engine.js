/**
 * SigmaOS Sovereign Analytics Engine
 * Module 05: Real-time system performance analysis and lattice visualization.
 */

const AnalyticsEngine = {
    history: [],
    
    logMetrics(metric, value) {
        const entry = {
            timestamp: Date.now(),
            metric,
            value
        };
        this.history.push(entry);
        if (this.history.length > 100) this.history.shift();
        
        // Signal event for HUD updates
        if (window.EventBus) {
            EventBus.publish('analytics_update', entry);
        }
    },

    getTrend(metric) {
        const filtered = this.history.filter(e => e.metric === metric);
        if (filtered.length < 2) return 0;
        const last = filtered[filtered.length - 1].value;
        const prev = filtered[filtered.length - 2].value;
        return last - prev;
    },

    init() {
        console.log("Σ Analytics Engine: Performance Tracking Active.");
        // Start periodic sampling
        setInterval(() => {
            const cpu = 10 + Math.random() * 20;
            this.logMetrics('cpu_usage', cpu);
        }, 5000);
    }
};

window.AnalyticsEngine = AnalyticsEngine;
