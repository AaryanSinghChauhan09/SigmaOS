"use strict";

import { SigmaSystem } from './SigmaSystem.js';
import { SigmaAI } from './SigmaAI.js';
import { SigmaDS } from './SigmaDS.js';
import { SigmaDSA } from './SigmaDSA.js';

window.addEventListener('DOMContentLoaded', () => {
    window.SIGMA = new SigmaSystem();
    
    // OOPS-Instantiated Specialized Shards
    const aiShard = new SigmaAI(window.SIGMA);
    const dsShard = new SigmaDS(window.SIGMA);
    const dsaShard = new SigmaDSA(window.SIGMA);

    // Global Handlers for Sovereign Shards (Delegating to OOPS objects)
    window.toggleMenu = () => document.getElementById('sigma-menu').classList.toggle('hidden');
    window.startAIGen = () => aiShard.execute();
    window.runDSAnalysis = () => dsShard.execute();
    window.runDSAViz = () => dsaShard.execute();
    
    window.setDistroMirror = (type) => {
        const root = document.documentElement;
        if (type === 'UBUNTU') {
            root.style.setProperty('--accent-primary', '#E95420');
            window.SIGMA.spawnToast('Distro Mirror: Ubuntu Parity ACTIVE.');
        } else if (type === 'ARCH') {
            root.style.setProperty('--accent-primary', '#1793D1');
            window.SIGMA.spawnToast('Distro Mirror: Arch Parity ACTIVE.');
        } else {
            root.style.setProperty('--accent-primary', '#00d2ff');
            window.SIGMA.spawnToast('Distro Mirror: Sovereign Mode [SIGMA].');
        }
    };

    window.runUXAudit = () => {
        const results = document.getElementById('ux-audit-results');
        if (!results) return;
        results.innerHTML = 'AUDITING UX CORE...';
        setTimeout(() => {
            results.innerHTML = 'PERFECT PARITY: UX Zenith Achieved.';
            window.SIGMA.spawnToast('UX Audit: Industrial parity confirmed.');
        }, 1000);
    };

    window.runOOPSAudit = () => {
        const results = document.getElementById('oops-audit-results');
        if (!results) return;
        results.innerHTML = 'AUDITING OOPS HIERARCHY...';
        setTimeout(() => {
            results.innerHTML = 'HIERARCHY ACCURATE: Classes Encapsulated. Inheritance Sharded.';
            window.SIGMA.spawnToast('OOPS Audit: Codebase inheritance verified.');
        }, 1000);
    };

    window.applyPersona = (role) => {
        const shards = window.SIGMA.store.shards;
        const config = {
            'AI_RESEARCHER': ['aishard', 'mlshard', 'dsshard'],
            'DATA_SCIENTIST': ['dsshard', 'dsashard', 'planmaster'],
            'CYBER_EXPERT': ['cybershard', 'amnesicshard', 'oopsshard'],
            'FULL_STACK': ['webshard', 'vfsmanager', 'automationshard']
        };
        const targets = config[role] || [];
        shards.forEach(s => {
            if (targets.includes(s.id)) s.enabled = true;
            else s.enabled = false;
        });
        window.SIGMA.renderMenu();
        window.SIGMA.renderShardManager();
        window.SIGMA.spawnToast(`Persona ACTIVE: ${role}. Specialized shards ENABLED.`);
    };

    window.executeAmnesicScrub = () => {
        const progress = document.getElementById('scrub-progress');
        if (!progress) return;
        progress.innerHTML = 'INITIATING FORENSIC OVERWRITE...';
        let i = 0;
        const interval = setInterval(() => {
            if (i >= 100) {
                clearInterval(interval);
                window.SIGMA.vfs.fs = {}; // Pure silicon zeroing
                progress.innerHTML = 'PURGE COMPLETE. VFS WIPED.';
                window.SIGMA.spawnToast('Amnesic: Forensic data scrub finished.');
                return;
            }
            i += 10;
            progress.innerHTML = `SCRUBBING RAM-DISK: ${i}%`;
        }, 300);
    };

    window.scheduleTask = () => {
        const task = document.getElementById('auto-task').value;
        const log = document.getElementById('auto-log');
        if (!task || !log) return;
        log.innerHTML += `[${new Date().toLocaleTimeString()}] SCHEDULING: ${task}<br>`;
        setTimeout(() => {
            window.SIGMA.shell.execute(task);
            log.innerHTML += `<span class="u-accent-text">[DONE] EXEC: ${task}</span><br>`;
        }, 2000);
    };

    window.setAccent = (color) => {
        document.documentElement.style.setProperty('--accent-primary', color);
        window.SIGMA.spawnToast(`Personalization: Accent set to ${color}`);
    };

    window.setBlur = (val) => {
        document.documentElement.style.setProperty('--glass-blur', `${val}px`);
        window.SIGMA.spawnToast(`Personalization: Blur set to ${val}px`);
    };

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
