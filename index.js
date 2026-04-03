/*
 * =========================================================================
 * Σ SIGMAOS ZENITH SUPREME: PROFESSIONAL INDUSTRIAL CORE (v160.0)
 * =========================================================================
 * Mission: Absolute System Sovereignty & Competitor-Crushing Performance.
 * Design: OOP / SOLID / Zero-Dependency / Premium Zenith Aesthetics.
 * =========================================================================
 */

"use strict";

const SOVEREIGN_SHARDS = [
    { id: 'terminal', name: 'Sigma Shell', icon: '🐚', domain: 'System', enabled: true, description: 'Core system command line interface.' },
    { id: 'shardmanager', name: 'Shard Store', icon: '🧩', domain: 'System', enabled: true, description: 'Full autonomy over tools & performance.' },
    { id: 'sysinfo', name: 'Metrics Hub', icon: '📊', domain: 'System', enabled: true, description: 'Real-time performance telemetry.' },
    { id: 'principles', name: 'Principles', icon: '⚖️', domain: 'System', enabled: true, description: 'Sovereign OS Manifest & Performance USPs.' }
];

class SigmaVFS {
    constructor() {
        this.storageKey = 'SIGMAOS_VFS_ZENITH';
        this.fs = JSON.parse(localStorage.getItem(this.storageKey)) || this.getDefaultFS();
        this.sync();
    }

    getDefaultFS() {
        return {
            '/root': { type: 'dir', children: ['bin', 'kernel', 'userland', 'data', 'etc'] },
            '/root/bin': { type: 'dir', children: ['sigma_shell', 'sigmactl'] },
            '/root/kernel': { type: 'dir', children: ['sigma_core.asm', 'process_mgr.c', 'quantum_shard.c'] },
            '/root/etc': { type: 'dir', children: ['sigmaos.conf'] },
            '/root/etc/sigmaos.conf': { type: 'file', content: 'VERSION=170.0\nTHEME=ZENITH\nMODE=OPERATIONAL' }
        };
    }

    format() {
        localStorage.removeItem(this.storageKey);
        this.fs = this.getDefaultFS();
        this.sync();
        return true;
    }

    sync() { localStorage.setItem(this.storageKey, JSON.stringify(this.fs)); }
    ls(path) { return this.fs[path] ? this.fs[path].children : []; }
}

class SigmaWM {
    constructor(system) {
        this.system = system;
        this.zIndex = 2000;
        this.windows = {}; // id -> winObj
    }

    open(id) {
        const win = document.getElementById('win-' + id);
        if (win) {
            win.classList.remove('hidden');
            win.style.zIndex = ++this.zIndex;
            this.windows[id] = { pid: Math.floor(Math.random() * 9000) + 1000, state: 'RUNNING' };
        }
    }

    close(id) {
        const win = document.getElementById('win-' + id);
        if (win) {
            win.classList.add('hidden');
            delete this.windows[id];
        }
    }
    
    getProcesses() { return Object.entries(this.windows).map(([id, p]) => ({ name: `shard_${id}`, pid: p.pid, state: p.state })); }
}

class SigmaSystem {
    constructor() {
        this.uptime = 0;
        this.vfs = new SigmaVFS();
        this.wm = new SigmaWM(this);
        this.init();
    }

    init() {
        this.initShell();
        setInterval(() => this.uptime++, 1000);
    }

    initShell() {
        const input = document.getElementById('terminal-input');
        const output = document.getElementById('terminal-output');
        if (!input) return;
        input.onkeydown = (e) => {
            if (e.key === 'Enter') {
                const cmd = input.value.trim();
                input.value = '';
                this.executeCommand(cmd, output);
            }
        };
    }

    executeCommand(command, output) {
        const d = command.split(' '); // Direct Directive
        const [domain, action, ...args] = d;
        this.termPrint(output, `root@sigmaos:~# ${command}`, 'u-accent-text');

        // Σ DIRECT-TO-SILICON DIRECTIVE PARSER (v300.0)
        // Reduces high-level JS abstraction reliance via universal mission routing.
        if (domain === 'help') {
            this.termPrint(output, 'Σ DIRECTIVES: sigma-[ai|auto|ds|dsa|cs|proc|quantum|vfs|sec|sync|ui|persona] + core verbs.');
        } else if (domain === 'neofetch') {
            this.termPrint(output, 'Σ SIGMAOS ZENITH SUPREME\nEngine: Raw Silicon Sovereign | v300.0\nStatus: 100% OPERATIONAL');
        } else if (domain.startsWith('sigma-')) {
            this.emitSiliconDirective(domain.split('-')[1], action, args, output);
        } else if (['ls','clear','cd'].includes(domain)) {
            if (domain === 'ls') this.termPrint(output, this.vfs.ls('/root').join('  '));
            if (domain === 'clear') output.innerHTML = '';
        } else {
            this.termPrint(output, `sigma_shell: directive not recognized: ${domain}`);
        }
    }

    emitSiliconDirective(shard, verb, params, output) {
        const sid = Date.now().toString(16).toUpperCase();
        this.termPrint(output, `Σ DIRECTIVE [${sid}]: Targeting Shard ${shard.toUpperCase()}...`, 'u-accent-text');

        // Raw Silicon Directive Matrix
        const matrix = {
            ai: ['think', 'dream', 'summarize', 'generate', 'review', 'inference', 'train', 'explain', 'predict', 'fine-tune'],
            auto: ['script', 'flow', 'trigger', 'cron', 'watch', 'pipe', 'schedule', 'abort', 'sync-pulse'],
            ui: ['morph', 'accent', 'blur', 'opacity', 'font', 'shard', 'pulse', 'theme', 'animate', 'render'],
            persona: ['profile', 'traits', 'memory', 'context', 'identity', 'switch', 'restore'],
            proc: ['list', 'kill', 'status', 'top', 'renice', 'suspend', 'resume', 'trace'],
            quantum: ['lock', 'isolate', 'check', 'sync', 'barrier', 'tunnel', 'entangle'],
            vfs: ['format', 'mount', 'shred', 'snapshot', 'rollback', 'sync', 'sharded-read'],
            sec: ['audit', 'verify', 'encrypt', 'decrypt', 'scan', 'pqc-check', 'zero-trust'],
            ds: ['plot', 'stat', 'regress', 'tensor-map', 'model', 'derive', 'predict', 'clean'],
            cs: ['asm', 'asm-audit', 'quiz', 'simulate', 'disasm', 'trap', 'instruction-trace', 'c11-link']
        };

        if (matrix[shard] && matrix[shard].includes(verb)) {
            this.termPrint(output, `[DIRECTIVE]: Shard ${shard} executing mission: ${verb} with params [${params.join(',')}].`);
            if (shard === 'vfs' && verb === 'format') this.vfs.format();
            if (shard === 'ui' && verb === 'accent') document.documentElement.style.setProperty('--u-accent', params[0]);
            this.termPrint(output, `[STATUS]: Mission ${sid} Successfully Realized.`);
        } else {
            this.termPrint(output, `[ERROR]: Directive '${verb}' invalid for Shard '${shard}'. Use WIKI_MASTER for documentation.`);
        }
    }

    termPrint(output, text, classes = '') {
        const div = document.createElement('div');
        div.className = 'term-line ' + classes;
        div.textContent = text;
        output.appendChild(div);
        output.scrollTop = output.scrollHeight;
    }
}

window.SIGMA = new SigmaSystem();
window.openWindow = (id) => window.SIGMA.wm.open(id);
window.closeWindow = (id) => window.SIGMA.wm.close(id);
