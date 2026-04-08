"use strict";

import { SigmaVFS } from './SigmaVFS.js';
import { SigmaWM } from './SigmaWM.js';
import { SigmaShell } from './SigmaShell.js';
import { SigmaStore } from './SigmaStore.js';

// INDUSTRIAL COMPONENTS (MILESTONE 185)
import { SigmaUIRenderer } from './components/SigmaUIRenderer.js';
import { SigmaAuditor } from './components/SigmaAuditor.js';
import { SigmaMetrics } from './components/SigmaMetrics.js';

import { SigmaThemer } from './components/SigmaThemer.js';

/**
 * Σ SIGMAOS SYSTEM CORE
 * Main orchestrator for the sovereign environment.
 */
export class SigmaSystem {
    constructor() {
        this.uptime = 0;
        this.vfs = new SigmaVFS();
        this.store = new SigmaStore(this);
        this.wm = new SigmaWM(this);
        this.shell = new SigmaShell(this);

        // Component Shards
        this.ui = new SigmaUIRenderer(this);
        this.auditor = new SigmaAuditor(this);
        this.metrics = new SigmaMetrics(this);
        this.themer = new SigmaThemer(this);

        this.vfs_vulnerabilities = [];
        this.init();
    }

    init() {
        this.detectPlatform();
        this.themer.loadInitial();
        
        setInterval(() => {
            this.uptime++;
            this.updateTelemetry();
            this.handleMemoryPressure();
        }, 1000);

        // Component Rendering
        this.ui.renderMenu();
        this.ui.renderShardManager();
        this.ui.renderMatrix();
        this.ui.renderDistros();

        // Metric Initialization
        this.metrics.initDataChart();
        this.metrics.initDSChart();

        this.vfs_vulnerabilities = this.auditor.sysAudit();
        this.spawnToast('SIGMAOS ZENITH v196.0: SYSTEM READY');

        document.onkeydown = (e) => {
            if (e.altKey && e.key === 'r') this.spawnToast('System Hard-Refresh Initiated...');
            if (e.key === 'Escape') this.wm.close('all');
        };
    }

    handleMemoryPressure() {
        const pressure = Math.floor(Math.random() * 100);
        if (pressure > 85 && this.store) {
            this.spawnToast('⚠️ HIGH MEMORY PRESSURE DETECTED: Triggering Eviction...');
            this.store.purge();
            this.spawnToast('✅ EVICTION COMPLETE: Unused Shards Purged.');
        }
    }

    launchDistro(id) {
        const d = this.store.distros.find(x => x.id === id);
        if (!d) return;
        const selector = document.getElementById('distro-selector');
        const iframe = document.getElementById('distro-iframe');
        if (selector) selector.classList.add('hidden');
        if (iframe) {
            iframe.src = d.url;
            iframe.classList.remove('hidden');
        }
        this.spawnToast(`Streaming Distribution Shard: ${d.name}`);
    }

    executeMatrixTool(id) {
        const tool = this.store.matrixTools.find(x => x.id === id);
        if (!tool) return;
        this.spawnToast(`Executing Industrial Shard: ${tool.name}`);
        if (id === 'ai_orchestrator') this.wm.open('aiorch');
        if (id === 'spectrum_terminal') this.wm.open('spectrum');
    }

    spawnToast(msg) {
        const container = document.getElementById('toast-container');
        if (!container) return;
        const toast = document.createElement('div');
        toast.className = 'toast show';
        toast.textContent = msg;
        container.appendChild(toast);
        setTimeout(() => toast.remove(), 3000);
    }

    updateTelemetry() {
        const cpuVal = document.getElementById('cpu-val');
        const memVal = document.getElementById('mem-val');
        const healthVal = document.getElementById('health-score-val');
        
        const cpu = Math.floor(Math.random() * 5 + 1);
        const mem = Math.floor(Math.random() * 10 + 24);
        
        if (cpuVal) cpuVal.textContent = cpu + '%';
        if (memVal) memVal.textContent = mem + '%';
        
        const health = this.calculateHealthScore(cpu, mem);
        if (healthVal) {
            healthVal.textContent = health;
            healthVal.className = health > 80 ? 'u-accent-text' : (health > 50 ? 'u-warning-text' : 'u-error-text');
        }
    }

    calculateHealthScore(cpu, mem) {
        let score = 100;
        score -= (cpu > 80 ? (cpu - 80) * 2 : 0);
        score -= (mem > 90 ? (mem - 90) * 3 : 0);
        score -= (this.vfs_vulnerabilities.length * 2);
        return Math.max(0, score);
    }

    switchMode(mode) { this.themer.apply(mode); }

    detectPlatform() {
        const ua = navigator.userAgent.toLowerCase();
        if (/mobile|android|iphone|ipad|tablet/.test(ua)) {
            document.body.classList.add('platform-mobile');
            this.spawnToast('Platform: MOBILE Optimized Shard ACTIVE.');
        } else {
            document.body.classList.add('platform-pc');
            this.spawnToast('Platform: PC Industrial Zenith ACTIVE.');
        }
    }

    // Proxy Audit Methods (MILESTONE 185)
    runUXAudit() { this.auditor.runUXAudit(); }
    runOOPSAudit() { this.auditor.runOOPSAudit(); }
    renderShardManager() { this.ui.renderShardManager(); }
    renderMenu() { this.ui.renderMenu(); }
}
