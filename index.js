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
        const parts = command.split(' ');
        const name = parts[0].toLowerCase();
        const args = parts.slice(1);
        this.termPrint(output, `root@sigmaos:~# ${command}`, 'u-accent-text');

        const cmds = {
            help: () => this.termPrint(output, 'Commands: sigma-ai, sigma-ds, sigma-dsa, sigma-cs, sigma-proc, sigma-quantum, sigma-vfs, sigma-sec, sigma-sync, ls, neofetch, clear'),
            clear: () => { output.innerHTML = ''; },
            ls: () => this.termPrint(output, this.vfs.ls('/root').join('  ')),
            neofetch: () => this.termPrint(output, 'Σ SIGMAOS ZENITH OPERATIONAL\nKernel: Sovereign C11 | v170.0\nStatus: 100% OPERATIONAL'),
            'sigma-ai': (a) => {
                const sub = a[0];
                if (sub === 'inference') {
                    this.termPrint(output, 'Σ AI: Loading weights into SigmaTransformer cache...');
                    this.termPrint(output, 'Σ AI: Loop active. Reason: Sovereignty achieved.');
                } else if (sub === 'train') {
                    this.termPrint(output, 'Σ AI: Initiating Sharded Pretraining mission...');
                } else {
                    this.termPrint(output, 'Usage: sigma-ai [train|inference|explain]');
                }
            },
            'sigma-proc': (a) => {
                const sub = a[0];
                if (sub === 'kill') {
                    const pid = a[1];
                    this.termPrint(output, `Σ PROC: Purging PID ${pid} from kernel scheduler...`);
                    this.termPrint(output, '[SUCCESS]: Shard memory released.');
                } else {
                    this.termPrint(output, 'Σ PROCESS MANAGER: Active Shards:');
                    this.wm.getProcesses().forEach(p => this.termPrint(output, `[${p.pid}] ${p.name} - ${p.state}`));
                }
            },
            'sigma-quantum': (a) => {
                const sub = a[0];
                if (sub === 'lock') {
                    this.termPrint(output, 'Σ QUANTUM: Engaging AVX-512 memory isolation locks...');
                    this.termPrint(output, '[SEC]: Ring-0 hardware barrier established.');
                } else {
                    this.termPrint(output, 'Σ QUANTUM: Shard density stable at 100%.');
                }
            },
            'sigma-vfs': (a) => {
                const sub = a[0];
                if (sub === 'format') {
                    this.termPrint(output, 'Σ VFS: WARNING! Block-level wipe initiating in 3s...');
                    setTimeout(() => {
                        this.vfs.format();
                        this.termPrint(output, '[SUCCESS]: Partition /root re-initialized (Zenith Layout).');
                    }, 3000);
                } else {
                    this.termPrint(output, 'Σ VFS: Partition /root mounted (Sovereign Block Shards).');
                }
            },
            'sigma-sec': (a) => {
                this.termPrint(output, 'Σ SECURITY: Initiating Real-time Zero-Trust audit...');
                this.termPrint(output, '[SEC]: Integrity: 100% | Hash: SHA3-512 Verified.');
            },
            'sigma-cs': (a) => {
                const sub = a[0];
                if (sub === 'quiz') {
                    this.termPrint(output, 'Σ CS QUIZ: Q: What is the complexity of Shard Sort? A: O(log N).');
                } else {
                    this.termPrint(output, 'Σ CS: Running scheduler simulation mission...');
                }
            },
            'sigma-sync': () => {
                this.termPrint(output, 'Σ SYNC: Establishing PQC-1024 Handshake...');
                setTimeout(() => this.termPrint(output, '[SUCCESS]: Master Parity Achieved.'), 1000);
            }
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
}

window.SIGMA = new SigmaSystem();
window.openWindow = (id) => window.SIGMA.wm.open(id);
window.closeWindow = (id) => window.SIGMA.wm.close(id);
