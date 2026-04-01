"use strict";

/**
 * Σ DATA SCIENCE SHARD
 * Statistical analysis and visualization kernels.
 */
export class SigmaDS {
    static runAnalysis(canvas, spawnToast) {
        if (!canvas) return;
        const ctx = canvas.getContext('2d');
        
        const data = Array(100).fill(0).map(() => Math.random() * 100);
        const mean = data.reduce((a, b) => a + b) / data.length;
        const variance = data.reduce((a, b) => a + Math.pow(b - mean, 2), 0) / data.length;
        
        spawnToast(`DS: Analysis finished. Mean: ${mean.toFixed(2)}, Var: ${variance.toFixed(2)}`);
        
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        const bins = Array(10).fill(0);
        data.forEach(d => bins[Math.floor(d/10)]++);
        bins.forEach((b, i) => {
            ctx.fillStyle = '#00d2ff';
            ctx.fillRect(i * 40, canvas.height - b * 10, 35, b * 10);
        });
        
        const log = document.getElementById('ds-stats-log');
        if (log) log.textContent = `Mean: ${mean.toFixed(2)} | Var: ${variance.toFixed(2)}`;
    }
}
