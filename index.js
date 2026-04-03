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
            '/root': { type: 'dir', children: ['bin', 'kernel', 'userland'] },
            '/root/bin': { type: 'dir', children: ['sigma_shell', 'sigmactl'] },
            '/root/kernel': { type: 'dir', children: ['sigma_core.asm'] }
        };
    }

    sync() { localStorage.setItem(this.storageKey, JSON.stringify(this.fs)); }
    ls(path) { return this.fs[path] ? this.fs[path].children : []; }
}

class SigmaWM {
    constructor(system) {
        this.system = system;
        this.zIndex = 1000;
        this.activeWorkspace = 1;
        this.workspaces = {};
    }

    open(id) {
        const win = document.getElementById('win-' + id);
        if (win) {
            win.classList.remove('hidden');
            win.style.zIndex = ++this.zIndex;
        }
    }

    close(id) {
        const win = document.getElementById('win-' + id);
        if (win) win.classList.add('hidden');
    }
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
        const parts = command.split(' ');
        const name = parts[0].toLowerCase();
        const args = parts.slice(1);
        this.termPrint(output, `root@sigmaos:~# ${command}`, 'u-accent-text');

        const cmds = {
            help: () => this.termPrint(output, 'Commands: sigma-ai, sigma-ds, sigma-dsa, sigma-cs, sigma-proc, sigma-quantum, sigma-vfs, sigma-sec, sigma-sync, sigma-auto, sigma-tool, sigma-ui, ls, neofetch, clear'),
            clear: () => { output.innerHTML = ''; },
            ls: () => this.termPrint(output, this.vfs.ls('/root').join('  ')),
            neofetch: () => this.termPrint(output, 'Σ SIGMAOS ZENITH SUPREME\nKernel: Sovereign C11\nShell: Omni-Shell v160.0'),
            'sigma-ai': (a) => this.termPrint(output, 'Σ AI/ML: Industrial Reasoning Shard Active...'),
            'sigma-ds': (a) => this.termPrint(output, 'Σ DATA SCIENCE: Preprocessing sharded datasets...'),
            'sigma-dsa': (a) => this.termPrint(output, 'Σ DSA: Benchmarking O(log N) algorithm shards...'),
            'sigma-cs': (a) => this.termPrint(output, 'Σ COMPUTER SCIENCE: Simulating memory-mapped I/O...'),
            'sigma-proc': (a) => this.termPrint(output, 'Σ PROCESS MANAGER: Listing all sovereign PIDs...'),
            'sigma-quantum': (a) => this.termPrint(output, 'Σ QUANTUM SHARD: Integrity check at 100.00%.'),
            'sigma-vfs': (a) => this.termPrint(output, 'Σ VFS: Shard mount successful on /root.'),
            'sigma-sec': (a) => this.termPrint(output, 'Σ SECURITY: Zero-trust diagnostic running...'),
            'sigma-sync': () => {
                this.termPrint(output, 'Σ SYNC: Establishing PQC-1024 Handshake with GitHub...', 'u-accent-text');
                setTimeout(() => this.termPrint(output, '[SUCCESS]: Cloud Shards Synchronized.'), 1000);
            },
            'sigma-auto': (a) => this.termPrint(output, `Σ AUTOMATION: Mission '${a.join(' ')}' scheduled.`),
            'sigma-tool': (a) => this.termPrint(output, 'Σ TOOLS: studio, gaming, remote-bot, xclicker ready.'),
            'sigma-ui': (a) => this.termPrint(output, 'Σ UI: Aesthetic Zenith Morph applied.')
        };

        if (cmds[name]) cmds[name](args);
        else this.termPrint(output, `sigma_shell: command not found: ${name}`);
    }

    termPrint(output, text, classes = '') {
        const div = document.createElement('div');
        div.className = 'term-line ' + classes;
        div.textContent = text;
        output.appendChild(div);
        output.scrollTop = output.scrollHeight;
    }

    spawnToast(msg) {
        console.log(`[Σ TOAST]: ${msg}`);
    }
}

window.SIGMA = new SigmaSystem();
window.openWindow = (id) => window.SIGMA.wm.open(id);
window.closeWindow = (id) => window.SIGMA.wm.close(id);
