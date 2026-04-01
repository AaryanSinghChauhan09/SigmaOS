"use strict";

import { SigmaShard } from './SigmaShard.js';
import { SMU } from './SigmaMathUnit.js';

/**
 * Σ DATA SCIENCE SHARD (HLL-Reduced)
 * Uses pure loop iteration and SMU math kernels.
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
        
        // Manual data allocation for silica parity
        const data = Array(100);
        let sum = 0;
        for (let i = 0; i < 100; i++) {
            data[i] = SMU.random() * 100;
            sum += data[i];
        }
        
        const mean = sum / 100;
        let v_sum = 0;
        for (let j = 0; j < 100; j++) {
            v_sum += SMU.pow(data[j] - mean, 2);
        }
        const variance = v_sum / 100;
        
        this.log(`Pure Silicon Analysis finished. Mean: ${mean.toFixed(2)}, Var: ${variance.toFixed(2)}`);
        
        ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        const bins = Array(10);
        for(let k=0; k<10; k++) bins[k] = 0;
        for(let l=0; l<100; l++) bins[Math.floor(data[l]/10)]++;
        for(let m=0; m<10; m++) {
            ctx.fillStyle = '#00d2ff';
            ctx.fillRect(m * 40, this.canvas.height - bins[m] * 10, 35, bins[m] * 10);
        }
        
        if (this.logElem) this.logElem.textContent = `Mean: ${mean.toFixed(2)} | Var: ${variance.toFixed(2)}`;
    }
}
