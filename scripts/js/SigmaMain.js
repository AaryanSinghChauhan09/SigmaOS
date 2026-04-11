"use strict";

import { SigmaSystem } from './SigmaSystem.js';
import { SigmaAI } from './SigmaAI.js';
import { SigmaDS } from './SigmaDS.js';
import { SigmaDSA } from './SigmaDSA.js';
import { SigmaDistroHandlers } from './components/SigmaDistroHandlers.js';
import { SigmaSecurityHandlers } from './components/SigmaSecurityHandlers.js';

window.addEventListener('DOMContentLoaded', () => {
    window.SIGMA = new SigmaSystem();
    
    // OOPS-Instantiated Specialized Shards
    const aiShard = new SigmaAI(window.SIGMA);
    const dsShard = new SigmaDS(window.SIGMA);
    const dsaShard = new SigmaDSA(window.SIGMA);
    
    // Fix linter warnings for video[playsinline] by setting it programmatically
    const camera = document.getElementById('camera-stream');
    if (camera) camera.setAttribute('playsinline', '');

    // Global Handlers (Modularly Delegated)
    window.toggleMenu = () => document.getElementById('sigma-menu').classList.toggle('hidden');
    window.startAIGen = () => aiShard.execute();
    window.runDSAnalysis = () => dsShard.execute();
    window.runDSAViz = () => dsaShard.execute();
    
    // Distro Handlers
    window.setDistroMirror = (type) => SigmaDistroHandlers.setDistroMirror(type, window.SIGMA);
    window.applyPersona = (role) => SigmaDistroHandlers.applyPersona(role, window.SIGMA);

    // Security Handlers
    window.executeAmnesicScrub = () => SigmaSecurityHandlers.executeAmnesicScrub(window.SIGMA);
    window.runKaliScan = () => SigmaSecurityHandlers.runKaliScan(window.SIGMA);

    // UI & Personalization
    window.setAccent = (color) => {
        document.documentElement.style.setProperty('--accent-primary', color);
        window.SIGMA.spawnToast(`Personalization: Accent set to ${color}`);
    };

    window.setBlur = (val) => {
        document.documentElement.style.setProperty('--glass-blur', `${val}px`);
        window.SIGMA.spawnToast(`Personalization: Blur set to ${val}px`);
    };

    window.openWindow = (id) => window.SIGMA.wm.open(id);

    // Initialization
    window.SIGMA.renderDistros();
    window.SIGMA.renderMatrix();
    window.SIGMA.scheduleAutoBackup();
    
    setTimeout(() => {
        window.SIGMA.spawnToast('ZENITH: Self-Healing Architecture Online.');
    }, 5000);
});
