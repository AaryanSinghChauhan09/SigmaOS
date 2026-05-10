// File: zenith_desktop/modules/telemetry.js
/**
 * SigmaOS TelemetrySystem
 * Manages system metrics collection and updates
 * @class
 */
export class TelemetrySystem {
    /**
     * Initialize telemetry system
     * @param {Object} config - Configuration options
     * @param {number} config.updateInterval - Metrics update interval in ms (default: 2000)
     * @param {number} config.maxMetrics - Maximum metrics to retain (default: 100)
     */
    constructor(config = {}) {
        this.smoothCpu = 12;
        this.smoothMem = 4.2;
        this.updateInterval = config.updateInterval || 2000;
        this.maxMetrics = config.maxMetrics || 100;
        this.metrics = [];
        this.animationFrameId = null;
        this.lastUpdateTime = 0;
        
        this.initDOMCache();
    }

    initDOMCache() {
        if (typeof document === 'undefined') return;
        this.dom = {
            cpuLoad: document.getElementById('cpu-load'),
            cpuProgress: document.getElementById('cpu-progress'),
            memLoad: document.getElementById('mem-load'),
            memProgress: document.getElementById('mem-progress'),
        };
    }

    start() {
        if (this.animationFrameId) return;
        this.lastUpdateTime = Date.now();
        this.loop();
    }

    stop() {
        if (this.animationFrameId) {
            cancelAnimationFrame(this.animationFrameId);
            this.animationFrameId = null;
        }
    }

    loop = () => {
        const now = Date.now();
        if (now - this.lastUpdateTime >= this.updateInterval) {
            this.update();
            this.lastUpdateTime = now;
        }
        this.animationFrameId = typeof requestAnimationFrame !== 'undefined' ? requestAnimationFrame(this.loop) : null;
    }

    update() {
        this.updateCPU();
        this.updateMemory();
        this.recordMetrics();
    }

    updateCPU() {
        this.smoothCpu = Math.max(5, Math.min(95, 
            this.smoothCpu + (Math.random() - 0.5) * 5
        ));
        const cpu = Math.round(this.smoothCpu);
        
        if (this.dom && this.dom.cpuLoad) {
            this.dom.cpuLoad.textContent = cpu + "%";
            this.dom.cpuProgress.style.width = cpu + "%";
        }
    }

    updateMemory() {
        this.smoothMem = Math.max(3.5, Math.min(8.0,
            this.smoothMem + (Math.random() - 0.5) * 0.08
        ));
        
        if (this.dom && this.dom.memLoad) {
            this.dom.memLoad.textContent = this.smoothMem.toFixed(1) + " GB";
        }
    }

    recordMetrics() {
        this.metrics.push({
            timestamp: Date.now(),
            cpu: Math.round(this.smoothCpu),
            mem: parseFloat(this.smoothMem.toFixed(1))
        });

        if (this.metrics.length > this.maxMetrics) {
            this.metrics.shift();
        }
    }

    getMetrics() {
        return [...this.metrics];
    }

    getAverageCPU() {
        if (!this.metrics.length) return 0;
        return (this.metrics.reduce((sum, m) => sum + m.cpu, 0) / this.metrics.length).toFixed(2);
    }
}

export default TelemetrySystem;
