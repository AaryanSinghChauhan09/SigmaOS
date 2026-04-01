"use strict";

import { SigmaVFS } from './SigmaVFS.js';
import { SigmaWM } from './SigmaWM.js';
import { SigmaShell } from './SigmaShell.js';
import { SigmaStore } from './SigmaStore.js';

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
        
        this.init();
    }

    init() {
        setInterval(() => {
            this.uptime++;
            this.updateMetrics();
        }, 1000);

        this.renderMenu();
        this.renderShardManager();
        this.initSpecializedShards();
        this.spawnToast('SIGMAOS ZENITH v160.0: SYSTEM READY');

        // Global key handlers
        document.onkeydown = (e) => {
            if (e.altKey && e.key === 'r') this.spawnToast('System Hard-Refresh Initiated...');
            if (e.key === 'Escape') this.wm.close('all');
        };
    }

    initSpecializedShards() {
        this.initDataChart();
        this.initDSChart();
        this.initSysAuditor();
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

    updateMetrics() {
        const cpuVal = document.getElementById('cpu-val');
        const memVal = document.getElementById('mem-val');
        if (cpuVal) cpuVal.textContent = Math.floor(Math.random() * 5 + 1) + '%';
        if (memVal) memVal.textContent = Math.floor(Math.random() * 10 + 24) + '%';
    }

    initDataChart() {
        const canvas = document.getElementById('data-chart');
        if (!canvas) return;
        const ctx = canvas.getContext('2d');
        let points = Array(20).fill(100);
        const render = () => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            ctx.strokeStyle = '#00d2ff';
            ctx.beginPath();
            ctx.moveTo(0, points[0]);
            points.forEach((p, i) => ctx.lineTo(i * 20, p));
            ctx.stroke();
            points.shift();
            points.push(Math.random() * 100 + 50);
            requestAnimationFrame(render);
        };
        render();
    }

    initDSChart() {
        const canvas = document.getElementById('ds-canvas');
        if (!canvas) return;
        const ctx = canvas.getContext('2d');
        const draw = () => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            ctx.strokeStyle = '#00d2ff';
            ctx.beginPath();
            for(let i=0; i<canvas.width; i++) {
                const y = Math.sin(i * 0.05 + Date.now() * 0.005) * 30 + 75;
                if(i===0) ctx.moveTo(i, y);
                else ctx.lineTo(i, y);
            }
            ctx.stroke();
            requestAnimationFrame(draw);
        };
        draw();
    }

    initSysAuditor() {
        const insecurePatterns = ['password', 'secret', 'token', 'key'];
        this.vfs_vulnerabilities = [];
        Object.keys(this.vfs.fs).forEach(path => {
            insecurePatterns.forEach(p => {
                if (path.toLowerCase().includes(p)) this.vfs_vulnerabilities.push(path);
            });
        });
    }

    renderMenu() {
        const grid = document.querySelector('.menu-items-grid');
        if (!grid) return;
        grid.innerHTML = '';
        this.store.shards.filter(s => s.enabled).forEach(s => {
            const card = document.createElement('div');
            card.className = 'menu-card';
            card.innerHTML = `<div class="u-font-size-lg">${s.icon}</div><div class="u-font-size-xxs u-margin-t-5">${s.name}</div>`;
            card.onclick = () => {
                this.wm.open(s.id);
                document.getElementById('sigma-menu').classList.add('hidden');
            };
            grid.appendChild(card);
        });
    }

    renderShardManager() {
        const list = document.getElementById('shard-manager-list');
        if (!list) return;
        list.innerHTML = '';
        this.store.shards.forEach(s => {
            const item = document.createElement('div');
            item.className = 'metric-card u-flex-between';
            item.innerHTML = `
                <div>
                    <div class="u-bold">${s.icon} ${s.name}</div>
                    <div class="u-font-size-xxs u-muted-text">${s.description}</div>
                </div>
                <label class="switch">
                    <input type="checkbox" ${s.enabled ? 'checked' : ''} onchange="SIGMA.store.toggle('${s.id}')">
                    <span class="slider"></span>
                </label>
            `;
            list.appendChild(item);
        });
    }
}
