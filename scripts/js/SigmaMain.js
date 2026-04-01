"use strict";

import { SigmaSystem } from './SigmaSystem.js';
import { SigmaAI } from './SigmaAI.js';
import { SigmaDS } from './SigmaDS.js';
import { SigmaDSA } from './SigmaDSA.js';

window.addEventListener('DOMContentLoaded', () => {
    window.SIGMA = new SigmaSystem();

    // Global Handlers for Sovereign Shards
    window.toggleMenu = () => document.getElementById('sigma-menu').classList.toggle('hidden');
    window.startAIGen = () => SigmaAI.startTraining(document.getElementById('ai-bar'), document.getElementById('ai-mission-status'), window.SIGMA.spawnToast);
    window.runDSAnalysis = () => SigmaDS.runAnalysis(document.getElementById('ds-canvas'), window.SIGMA.spawnToast);
    window.runDSAViz = () => SigmaDSA.runViz(document.getElementById('dsa-viz-area'), document.getElementById('dsa-algo').value, window.SIGMA.spawnToast);
    window.runUXAudit = () => {
        const results = document.getElementById('ux-audit-results');
        if (!results) return;
        results.innerHTML = 'AUDITING UX CORE...<br>';
        
        let score = 100;
        const issues = [];
        
        // Fitts's Law Check (Simple check for target sizes)
        document.querySelectorAll('.dock-item, .win-btn').forEach(el => {
            if (el.offsetWidth < 30 || el.offsetHeight < 30) {
                score -= 5;
                issues.push(`LOW TARGET AREA [${el.className}]`);
            }
        });

        // Jakob's Law Check (Familiarity)
        const windows = document.querySelectorAll('.window');
        windows.forEach(w => {
            if (!w.querySelector('.win-header')) {
                score -= 10;
                issues.push(`NON-STANDARD WINDOW: No Header`);
            }
        });

        setTimeout(() => {
            results.innerHTML = `SOVEREIGN SCORE: ${score}/100<br>`;
            if (issues.length > 0) results.innerHTML += `ISSUES: ${issues.join(', ')}`;
            else results.innerHTML += 'PERFECT PARITY: UX Zenith Achieved.';
            window.SIGMA.spawnToast(`UX Audit: Score ${score}. Industrial parity confirmed.`);
        }, 1000);
    };
    window.purgeUnusedShards = () => window.SIGMA.store.purge();
    window.openWindow = (id) => window.SIGMA.wm.open(id);
});
