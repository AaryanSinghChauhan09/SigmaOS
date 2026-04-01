"use strict";

import { SigmaShard } from './SigmaShard.js';

/**
 * Σ DATA SCIENCE SHARD (OOPS-Refactored)
 */
export class SigmaDS extends SigmaShard {
    constructor(system) {
        super('dsshard', 'Data Sci', system);
        this.canvas = document.getElementById('ds-canvas');
        this.logElem = document.getElementById('ds-stats-log');
    }

    /**
     * Polymorphic implementation of execute() for Stat Analysis.
     */
    execute() {
        if (!this.canvas) return;
        const ctx = this.canvas.getContext('2d');
        
        const data = Array(100).fill(0).map(() => Math.random() * 100);
        const mean = data.reduce((a, b) => a + b) / data.length;
        const variance = data.reduce((a, b) => a + Math.pow(b - mean, 2), 0) / data.length;
        
        this.log(`Analysis finished. Mean: ${mean.toFixed(2)}, Var: ${variance.toFixed(2)}`);
        
        ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        const bins = Array(10).fill(0);
        data.forEach(d => bins[Math.floor(d/10)]++);
        bins.forEach((b, i) => {
            ctx.fillStyle = '#00d2ff';
            ctx.fillRect(i * 40, this.canvas.height - b * 10, 35, b * 10);
        });
        
        if (this.logElem) this.logElem.textContent = `Mean: ${mean.toFixed(2)} | Var: ${variance.toFixed(2)}`;
    }
}
