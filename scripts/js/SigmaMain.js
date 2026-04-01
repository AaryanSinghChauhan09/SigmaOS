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
    window.setDistroMirror = (type) => {
        const root = document.documentElement;
        if (type === 'UBUNTU') {
            root.style.setProperty('--accent-primary', '#E95420');
            window.SIGMA.spawnToast('Distro Mirror: Ubuntu Parity ACTIVE. [apt] enabled.');
        } else if (type === 'ARCH') {
            root.style.setProperty('--accent-primary', '#1793D1');
            window.SIGMA.spawnToast('Distro Mirror: Arch Parity ACTIVE. [pacman] enabled.');
        } else {
            root.style.setProperty('--accent-primary', '#00d2ff');
            window.SIGMA.spawnToast('Distro Mirror: Sovereign Mode [SIGMA] - Industrial Zenith.');
        }
    };
    window.purgeUnusedShards = () => window.SIGMA.store.purge();
    window.openWindow = (id) => window.SIGMA.wm.open(id);
});
