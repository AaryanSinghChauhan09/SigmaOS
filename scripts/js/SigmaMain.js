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
    window.runCyberScan = () => {
        const log = document.getElementById('cyber-scan-log');
        if (!log) return;
        log.innerHTML = '[INFO] Auditing Sovereign VFS...<br>';
        const vulns = window.SIGMA.vfs_vulnerabilities;
        let i = 0;
        const interval = setInterval(() => {
            if (i >= vulns.length) { 
                clearInterval(interval); 
                log.innerHTML += `[COMPLETE] Audit finished. ${vulns.length} insecure paths found.`;
                window.SIGMA.spawnToast(`Security: Audit finished.`);
                return; 
            }
            log.innerHTML += `<span class="u-error-text">[VULN] Insecure Path: ${vulns[i++]}</span><br>`;
            log.scrollTop = log.scrollHeight;
        }, 400);
    };
    window.purgeUnusedShards = () => window.SIGMA.store.purge();
    window.openWindow = (id) => window.SIGMA.wm.open(id);
});
