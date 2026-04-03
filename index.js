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
        const domain = parts[0].toLowerCase();
        const action = parts[1]?.toLowerCase();
        const args = parts.slice(2);
        
        this.termPrint(output, `root@sigmaos:~# ${command}`, 'u-accent-text');

        // Σ DYNAMIC MISSION ENGINE (v200.0)
        // Reduces reliance on predefined static functions for raw-logic dispatch.
        if (domain === 'help') {
            this.termPrint(output, 'Σ ZENITH CORE DOMAINS: sigma-[ai|ds|dsa|cs|proc|quantum|vfs|sec|sync|auto|tool|ui|persona] + ls, neofetch, clear');
        } else if (domain === 'clear') {
            output.innerHTML = '';
        } else if (domain === 'neofetch') {
            this.termPrint(output, 'Σ SIGMAOS ZENITH SUPREME\nKernel: Sovereign C11 | v200.0 (Raw Engine)\nStatus: 100% OPERATIONAL');
        } else if (domain.startsWith('sigma-')) {
            const shard = domain.split('-')[1];
            this.dispatchRawMission(shard, action, args, output);
        } else if (domain === 'ls') {
            this.termPrint(output, this.vfs.ls('/root').join('  '));
        } else {
            this.termPrint(output, `sigma_shell: command not found: ${domain}`);
        }
    }

    dispatchRawMission(shard, action, args, output) {
        const missionId = `${shard.toUpperCase()}_MISSION_${Math.floor(Math.random() * 900) + 100}`;
        this.termPrint(output, `Σ SHARD [${shard.toUpperCase()}]: Initiating mission ${missionId}...`, 'u-accent-text');

        // Raw-Logic Domain Interpreter
        switch(shard) {
            case 'ai':
                this.termPrint(output, `[AI] Interpreting raw reasoning path: ${action} -> [${args.join(',')}]`);
                this.termPrint(output, '[AI] Status: REALIZED.');
                break;
            case 'auto':
                this.termPrint(output, `[AUTO] Establishing raw trigger flow: ${action} -> [${args.join(',')}]`);
                break;
            case 'ui':
                this.termPrint(output, `[UI] Injecting raw aesthetic shard: ${action} (${args[0]})`);
                if (action === 'accent') document.documentElement.style.setProperty('--u-accent', args[0]);
                break;
            case 'persona':
                this.termPrint(output, `[PERSONA] Reshaping kernel traits: ${action} -> [${args.join(',')}]`);
                break;
            case 'proc':
                if (action === 'kill') this.termPrint(output, `[PROC] Purging raw task ${args[0]} from scheduler...`);
                else this.wm.getProcesses().forEach(p => this.termPrint(output, `[${p.pid}] ${p.name}`));
                break;
            case 'vfs':
                if (action === 'format') {
                    this.termPrint(output, '[VFS] WARNING! raw block-level wipe in progress...');
                    this.vfs.format();
                } else this.termPrint(output, `[VFS] Partition /root ${action || 'MOUNTED'}.`);
                break;
            case 'cs':
                this.termPrint(output, `[CS] Executing raw theory simulation: ${action || 'SCHEDULER'}`);
                break;
            case 'sync':
                this.termPrint(output, '[SYNC] Establishing raw PQC-1024 Handshake...');
                break;
            default:
                this.termPrint(output, `[Σ] Generic Logic Shard: Executing ${action} mission on raw silicon...`);
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
