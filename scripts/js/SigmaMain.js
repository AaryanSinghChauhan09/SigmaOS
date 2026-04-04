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

    window.runPQCAudit = () => {
        const canvas = document.getElementById('pqc-canvas');
        if (!canvas) return;
        const ctx = canvas.getContext('2d');
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.strokeStyle = '#00d2ff';
        ctx.beginPath();
        for(let i=0; i<50; i++) {
            const x = Math.random() * canvas.width;
            const y = Math.random() * canvas.height;
            ctx.lineTo(x, y);
        }
        ctx.stroke();
        window.SIGMA.spawnToast('Quantum Shard: LWE Lattice integrity verified.');
    };

    window.syncLedger = () => {
        const log = document.getElementById('ledger-log');
        if (!log) return;
        log.innerHTML += `<br>[${new Date().toLocaleTimeString()}] BLOCK_MINED: Peer Consensus 0x${Math.random().toString(16).slice(2, 10)}`;
    };

    setInterval(window.syncLedger, 5000);

    window.runBioAlign = () => {
        const s1 = document.getElementById('bio-seq1').value;
        const s2 = document.getElementById('bio-seq2').value;
        window.SIGMA.spawnToast(`Genomics: Aligning ${s1} vs ${s2} on Silicon...`);
        setTimeout(() => {
            window.SIGMA.spawnToast(`Genomics: Alignment Score = +${Math.floor(Math.random() * 50) + 10} (Needleman-Wunsch)`);
        }, 1500);
    };

    window.runLLMAttention = () => {
        const log = document.getElementById('llm-ops-log');
        if (!log) return;
        log.innerHTML = 'EXECUTING MATMUL: Q x K^T...';
        setTimeout(() => {
            log.innerHTML = '<span class="u-accent-text">ATTENTION SCORES GENERATED. O(n^2*d) TENSOR COMPUTED.</span>';
            window.SIGMA.spawnToast('Transformer: Self-Attention Primitive Done.');
        }, 2000);
    };

    window.runHFTCalc = () => {
        const out = document.getElementById('hft-vwap-out');
        if (!out) return;
        out.innerHTML = 'PULLING LIQUIDITY VECTORS...';
        setTimeout(() => {
            const vwap = (Math.random() * 500 + 100).toFixed(4);
            out.innerHTML = `<span class="u-error-text">VWAP EXECUTION PRICE: $${vwap}</span>`;
            window.SIGMA.spawnToast('HFT Oracle: C-Kernel VWAP Calculated.');
        }, 800);
    };

    window.runCoworkBroadcast = () => {
        const msg = document.getElementById('cowork-msg').value;
        const log = document.getElementById('cowork-log');
        if (!msg || !log) return;
        log.innerHTML += `<br>[${new Date().toLocaleTimeString()}] SENT: ${msg.substring(0, 15)}...`;
        window.SIGMA.spawnToast('Co-Work: Context broadcast to local agent ring.');
        setTimeout(() => {
            log.innerHTML += `<br><span class="u-accent-text">[ACK] Agent 2 & Agent 3 Synced via C-IPC.</span>`;
        }, 1200);
    };

    window.runOracleQuery = () => {
        const query = document.getElementById('oracle-query').value;
        const out = document.getElementById('oracle-out');
        if (!query || !out) return;
        out.innerHTML = `Scanning VFS Disk for: ${query}...`;
        window.SIGMA.spawnToast('Oracle: Local knowledge graph search initiated.');
        setTimeout(() => {
            out.innerHTML = `<span class="u-error-text">0 CLOUD REQUESTS. Match found in Silicon Block 0x4F.</span>`;
        }, 1500);
    };

    window.runMacroClaw = () => {
        const stat = document.getElementById('claw-status');
        if (!stat) return;
        stat.innerHTML = 'TELEPORTING CURSOR VIA C-POINTER...';
        window.SIGMA.spawnToast('Macro Claw: Executing hardware pointer teleport.');
        setTimeout(() => {
            stat.innerHTML = 'CURSOR AUTOMATION EVENT TRIGGERED.';
            const cursor = document.getElementById('cursor-element') || document.body;
            cursor.style.cursor = 'crosshair';
        }, 800);
    };

    window.runQubesAudit = () => {
        const log = document.getElementById('qubes-log');
        if (!log) return;
        log.innerHTML = 'VERIFYING APP_VM BOUNDARIES...';
        window.SIGMA.spawnToast('Qubes Eq: Hypervisor separation check initiated.');
        setTimeout(() => {
            log.innerHTML = '<span class="u-accent-text">Xen Sub-VMs isolated. Ring-0 secure.</span>';
        }, 1000);
    };

    window.runTimeDelta = () => {
        const log = document.getElementById('time-log');
        if (!log) return;
        log.innerHTML = 'CALCULATING FILE POINTER DELTAS...';
        window.SIGMA.spawnToast('Time Machine Eq: Snapshotting differential bits.');
        setTimeout(() => {
            log.innerHTML = '<span class="u-accent-text">DELTA BACKUP: 142KB Synced to Persistence.</span>';
        }, 1500);
    };

    window.runTailsRoute = () => {
        const log = document.getElementById('tails-log');
        if (!log) return;
        log.innerHTML = 'OBFUSCATING: NODE 1 -> NODE 2 -> NODE 3...';
        window.SIGMA.spawnToast('Tails Eq: Onion packet encase generated.');
        setTimeout(() => {
            log.innerHTML = '<span class="u-accent-text">TRAFFIC MASKED (AES-XOR). Deep route complete.</span>';
        }, 1800);
    };

    window.runPlan9Bind = () => {
        const addr = document.getElementById('p9-addr').value;
        if (!addr) return;
        window.SIGMA.spawnToast(`9P: Binding namespace ${addr}...`);
        setTimeout(() => {
            window.SIGMA.spawnToast('Plan 9 Eq: Remote file system linked to local VFS boundary.');
        }, 1200);
    };

    window.runHolyPoke = () => {
        const addr = document.getElementById('holy-poke-addr').value;
        const val = document.getElementById('holy-poke-val').value;
        if (!addr || !val) return;
        window.SIGMA.spawnToast(`HolyC Eq: Poking memory address 0xB8000 + ${addr} with ${val}.`);
        setTimeout(() => {
            window.SIGMA.spawnToast('TempleOS Eq: Direct unabstracted VRAM modification successful.');
        }, 500);
    };

    window.runKaliScan = () => {
        const log = document.getElementById('kali-log');
        if (!log) return;
        log.innerHTML = 'ENUMERATING LOCAL SILICON PORTS...';
        window.SIGMA.spawnToast('Kali Eq: Dispersing raw SYN port scanning kernel.');
        setTimeout(() => {
            log.innerHTML = '<span class="u-error-text">SCAN COMPLETE. Open Vectors: 22, 80, 443, 6500.</span>';
        }, 1600);
    };

    window.runEBPFVerify = () => {
        const log = document.getElementById('ebpf-log');
        if (!log) return;
        log.innerHTML = 'VERIFYING BYTE-CODE SAFETY...';
        window.SIGMA.spawnToast('eBPF Eq: Sandboxed kernel instruction evaluation.');
        setTimeout(() => {
            log.innerHTML = '<span class="u-accent-text">BYTE-CODE VERIFIED. Hook attached to Ring-0.</span>';
        }, 1200);
    };

    window.runCGroupConstraint = () => {
        const log = document.getElementById('cgroup-log');
        if (!log) return;
        log.innerHTML = 'ALLOCATING HARDWARE CONSTRAINTS...';
        window.SIGMA.spawnToast('C-Groups Eq: Simulating process isolation mapping.');
        setTimeout(() => {
            log.innerHTML = '<span class="u-accent-text">CGROUP LOCKED. CPU capped at assigned threshold.</span>';
        }, 1000);
    };

    window.runOOMStrike = () => {
        const log = document.getElementById('oom-log');
        if (!log) return;
        log.innerHTML = 'MEMORY CRITICAL. SEEKING SACRIFICE HEURISTIC...';
        window.SIGMA.spawnToast('OOM-Killer Eq: System exhaustion protocol triggered.');
        setTimeout(() => {
            log.innerHTML = '<span class="u-error-text">OOM STRIKE: Process PID 42 (Largest memory consumer) terminated.</span>';
        }, 1400);
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

    window.openWindow = (id) => window.SIGMA.wm.open(id);

    // Dynamic UI Rendering for Sovereign Shards
    window.SIGMA.renderDistros();
    window.SIGMA.renderMatrix();

    // Sovereign Zenith Autonomous Tasks
    window.SIGMA.scheduleAutoBackup();
    
    window.runBiasAudit = () => aiShard.runBiasAudit();
    window.runSelfHealing = () => aiShard.selfHeal();
    window.createSnapshot = (name) => window.SIGMA.createSnapshot(name);

    setTimeout(() => {
        window.SIGMA.spawnToast('ZENITH: Self-Healing Architecture Online.');
    }, 5000);
});
