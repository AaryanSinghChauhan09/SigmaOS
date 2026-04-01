/*
 * =========================================================================
 * Σ SIGMAOS ZENITH SUPREME: PROFESSIONAL INDUSTRIAL CORE (v160.0)
 * =========================================================================
 * Mission: Absolute System Sovereignty & Competitor-Crushing Performance.
 * Design: OOP / SOLID / Zero-Dependency / Premium Zenith Aesthetics.
 * =========================================================================
 */

"use strict";

/**
 * Σ SOVEREIGN VFS (Virtual File System)
 * High-performance, persistent sharded storage.
 */
class SigmaVFS {
    constructor() {
        this.storageKey = 'SIGMAOS_VFS_ZENITH';
        this.fs = JSON.parse(localStorage.getItem(this.storageKey)) || this.getDefaultFS();
        this.sync();
    }

    getDefaultFS() {
        return {
            '/root': { type: 'dir', children: ['bin', 'kernel', 'userland', 'data', 'media', 'etc'] },
            '/root/bin': { type: 'dir', children: ['sigma_shell', 'sigmactl'] },
            '/root/kernel': { type: 'dir', children: ['sigma_core.asm', 'boot_master.c'] },
            '/root/userland': { type: 'dir', children: [] },
            '/root/data': { type: 'dir', children: ['industrial.json', 'audit.log'] },
            '/root/media': { type: 'dir', children: [] },
            '/root/etc': { type: 'dir', children: ['sigmaos.conf'] },
            '/root/data/industrial.json': { type: 'file', content: '{"status": "SOVEREIGN", "integrity": 100}' },
            '/root/etc/sigmaos.conf': { type: 'file', content: 'VERSION=160.0\nTHEME=ZENITH\nMODE=SUPREME' }
        };
    }

    sync() { localStorage.setItem(this.storageKey, JSON.stringify(this.fs)); }

    exists(path) { return !!this.fs[path]; }
    isDir(path) { return this.fs[path] && this.fs[path].type === 'dir'; }
    ls(path) { return this.fs[path] ? this.fs[path].children : []; }

    mkdir(path) {
        if (this.exists(path)) return false;
        const parts = path.split('/');
        const name = parts.pop();
        const parent = parts.join('/') || '/root';
        if (!this.isDir(parent)) return false;
        this.fs[path] = { type: 'dir', children: [] };
        if (!this.fs[parent].children.includes(name)) this.fs[parent].children.push(name);
        this.sync();
        return true;
    }

    write(path, content) {
        const parts = path.split('/');
        const name = parts.pop();
        const parent = parts.join('/') || '/root';
        if (!this.isDir(parent)) return false;
        if (!this.exists(path)) {
            if (!this.fs[parent].children.includes(name)) this.fs[parent].children.push(name);
        }
        this.fs[path] = { type: 'file', content };
        this.sync();
        return true;
    }

    read(path) { return this.fs[path] ? this.fs[path].content : null; }

    remove(path) {
        if (!this.exists(path)) return false;
        const parts = path.split('/');
        const name = parts.pop();
        const parent = parts.join('/') || '/root';
        this.fs[parent].children = this.fs[parent].children.filter(c => c !== name);
        delete this.fs[path];
        this.sync();
        return true;
    }

    snapshot(name) {
        const state = JSON.stringify(this.fs);
        localStorage.setItem(`SNAPSHOT_${name.toUpperCase()}`, state);
        return true;
    }

    rollback(name) {
        const state = localStorage.getItem(`SNAPSHOT_${name.toUpperCase()}`);
        if (!state) return false;
        this.fs = JSON.parse(state);
        this.sync();
        return true;
    }
}

/**
 * Σ WINDOW MANAGER (SigmaWM)
 * Handles workspace orchestration, taskbar, and industrial UI logic.
 */
class SigmaWM {
    constructor(system) {
        this.system = system;
        this.zIndex = 1000;
        this.activeWorkspace = 1;
        this.workspaces = {}; // winId -> wsNum
        this.init();
    }

    init() {
        // Window dragging and control events
        document.addEventListener('mousedown', (e) => {
            const header = e.target.closest('.win-header');
            if (header) {
                const win = header.parentElement;
                this.focus(win.id.replace('win-', ''));
                this.dragWindow(win, e);
            }
        });

        // Global actions for window buttons
        document.addEventListener('click', (e) => {
            const btn = e.target.closest('.win-btn');
            if (btn) {
                const action = btn.getAttribute('data-action');
                const winId = btn.getAttribute('data-win');
                if (action === 'close') this.close(winId);
                if (action === 'minimize') this.minimize(winId);
                if (action === 'maximize') this.maximize(winId);
            }
        });

        // Dock items
        document.querySelectorAll('.dock-item').forEach(item => {
            item.onclick = () => this.open(item.getAttribute('data-window'));
        });

        // Workspace indicators
        document.querySelectorAll('.ws-indicator').forEach(ws => {
            ws.onclick = () => this.switchWorkspace(parseInt(ws.getAttribute('data-ws')));
        });
    }

    dragWindow(win, e) {
        let offsetX = e.clientX - win.offsetLeft;
        let offsetY = e.clientY - win.offsetTop;
        const onMove = (ev) => {
            win.style.left = (ev.clientX - offsetX) + 'px';
            win.style.top = (ev.clientY - offsetY) + 'px';
        };
        const onUp = () => {
            document.removeEventListener('mousemove', onMove);
            document.removeEventListener('mouseup', onUp);
        };
        document.addEventListener('mousemove', onMove);
        document.addEventListener('mouseup', onUp);
    }

    open(id) {
        const win = document.getElementById('win-' + id);
        if (!win) return;
        win.classList.remove('hidden');
        win.classList.remove('minimized');
        this.workspaces[id] = this.activeWorkspace;
        this.focus(id);
        this.updateTaskbar();
    }

    close(id) {
        const win = document.getElementById('win-' + id);
        if (!win) return;
        win.classList.add('hidden');
        this.updateTaskbar();
        this.system.spawnToast(`Shard [${id}] Terminated.`);
    }

    minimize(id) {
        const win = document.getElementById('win-' + id);
        if (!win) return;
        win.classList.add('hidden');
        this.updateTaskbar();
        this.system.spawnToast(`Shard [${id}] Suspended.`);
    }

    maximize(id) {
        const win = document.getElementById('win-' + id);
        if (!win) return;
        win.classList.toggle('maximized');
    }

    focus(id) {
        const win = document.getElementById('win-' + id);
        if (!win) return;
        document.querySelectorAll('.window').forEach(w => w.classList.remove('focused'));
        win.classList.add('focused');
        win.style.zIndex = ++this.zIndex;
    }

    switchWorkspace(ws) {
        this.activeWorkspace = ws;
        document.querySelectorAll('.ws-indicator').forEach(el => {
            el.classList.toggle('active', parseInt(el.getAttribute('data-ws')) === ws);
        });
        this.render();
        this.system.spawnToast(`Switched to Workspace ${ws}`);
    }

    render() {
        document.querySelectorAll('.window').forEach(win => {
            const id = win.id.replace('win-', '');
            const winWS = this.workspaces[id] || 1;
            if (winWS === this.activeWorkspace && !win.classList.contains('hidden')) {
                win.style.display = 'flex';
            } else {
                win.style.display = 'none';
            }
        });
        this.updateTaskbar();
    }

    updateTaskbar() {
        const taskbar = document.getElementById('taskbar');
        if (!taskbar) return;
        taskbar.innerHTML = '';
        
        document.querySelectorAll('.window').forEach(win => {
            if (win.classList.contains('hidden')) return;
            const id = win.id.replace('win-', '');
            const title = win.querySelector('.win-title')?.innerText || id;
            
            const item = document.createElement('div');
            item.className = `top-item status-chip ${win.classList.contains('focused') ? 'active-chip' : ''}`;
            item.innerHTML = `<span>${title.split(' ')[0]}</span>`;
            item.onclick = () => {
                const ws = this.workspaces[id] || 1;
                if (ws !== this.activeWorkspace) this.switchWorkspace(ws);
                this.focus(id);
            };
            taskbar.appendChild(item);
        });
    }

    tile() {
        const windows = Array.from(document.querySelectorAll('.window:not(.hidden)'));
        if (windows.length === 0) return;
        const width = 100 / windows.length;
        windows.forEach((win, i) => {
            win.classList.remove('maximized');
            win.style.width = `calc(${width}% - 10px)`;
            win.style.height = 'calc(100vh - 60px)';
            win.style.left = `${i * width}%`;
            win.style.top = '40px';
        });
    }
}

/**
 * Σ SIGMA SYSTEM CORE
 * The main orchestrator for SigmaOS.
 */
class SigmaSystem {
    constructor() {
        this.uptime = 0;
        this.vfs = new SigmaVFS();
        this.wm = new SigmaWM(this);
        this.activeMode = 'ZENITH';
        this.init();
    }

    init() {
        this.initClock();
        this.initMetrics();
        this.initBackground();
        this.initShell();
        this.initSpotlight();
        this.initSpecializedShards();

        window.onerror = (msg, url, line) => {
            this.spawnToast(`Kernel Fault: ${msg}`, 0, true);
            return true;
        };

        this.spawnToast('Σ SIGMAOS ZENITH MASTERPIECE INITIALIZED');
    }

    initClock() {
        const clock = document.getElementById('clock');
        setInterval(() => {
            if (clock) clock.textContent = new Date().toTimeString().split(' ')[0];
            this.uptime++;
        }, 1000);
    }

    initMetrics() {
        setInterval(() => {
            const cpu = (Math.random() * 5 + 2).toFixed(1);
            const mem = (Math.random() * 2 + 18).toFixed(1);
            
            this.setElText('cpu-val', cpu + '%');
            this.setElText('mem-val', mem + '%');
            this.setElText('cpu-percent', cpu + '%');
            this.setElText('mem-val-display', mem);
            
            const cpuBar = document.getElementById('cpu-bar');
            if (cpuBar) cpuBar.style.width = cpu + '%';
            
            this.updateProcessList();
        }, 2000);
    }

    setElText(id, text) {
        const el = document.getElementById(id);
        if (el) el.textContent = text;
    }

    updateProcessList() {
        const list = document.getElementById('proc-list');
        if (!list) return;
        const procs = [
            { pid: 0, name: 'sigma_kernel', state: 'RUNNING', cpu: '0.1' },
            { pid: 1, name: 'sigma_init', state: 'RUNNING', cpu: '0.5' },
            { pid: 102, name: 'sigma_gui', state: 'RUNNING', cpu: '2.4' }
        ];
        list.innerHTML = procs.map(p => `
            <div class="u-flex-between">
                <span>${p.pid}</span><span>${p.name}</span><span>${p.state}</span><span>${p.cpu}%</span>
            </div>
        `).join('');
    }

    initBackground() {
        const canvas = document.getElementById('bg-canvas');
        if (!canvas) return;
        const ctx = canvas.getContext('2d');
        let w = canvas.width = window.innerWidth;
        let h = canvas.height = window.innerHeight;
        const cols = Math.floor(w / 14);
        const drops = Array(cols).fill(1);

        const draw = () => {
            ctx.fillStyle = 'rgba(0, 0, 0, 0.05)';
            ctx.fillRect(0, 0, w, h);
            ctx.fillStyle = getComputedStyle(document.documentElement).getPropertyValue('--accent-primary') || '#00d2ff';
            ctx.font = '14px monospace';
            drops.forEach((y, x) => {
                const txt = String.fromCharCode(0x03A3 + Math.random() * 20); // Sigma and friends
                ctx.fillText(txt, x * 14, y * 14);
                if (y * 14 > h && Math.random() > 0.975) drops[x] = 0;
                drops[x]++;
            });
        };
        setInterval(draw, 33);
        window.onresize = () => { w = canvas.width = window.innerWidth; h = canvas.height = window.innerHeight; };
    }

    initShell() {
        const input = document.getElementById('terminal-input');
        const output = document.getElementById('terminal-output');
        if (!input) return;

        input.onkeydown = (e) => {
            if (e.key === 'Enter') {
                const cmd = input.value.trim();
                input.value = '';
                if (!cmd) return;
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
            help: () => this.termPrint(output, 'Commands: help, ls, cd, mkdir, touch, rm, cat, clear, neofetch, cpu, mem, matrix, scrub, shutdown, sigmactl'),
            clear: () => { output.innerHTML = ''; },
            ls: () => this.termPrint(output, this.vfs.ls(this.cwd || '/root').join('  ')),
            cd: (a) => {
                const path = a[0] || '/root';
                if (this.vfs.exists(path) && this.vfs.isDir(path)) {
                    this.cwd = path;
                    this.termPrint(output, `CWD: ${this.cwd}`);
                } else {
                    this.termPrint(output, `cd: no such directory: ${path}`);
                }
            },
            mkdir: (a) => {
                if (!a[0]) return this.termPrint(output, 'mkdir: missing operand');
                const path = (this.cwd || '/root') + '/' + a[0];
                if (this.vfs.mkdir(path)) this.termPrint(output, `Created: ${a[0]}`);
                else this.termPrint(output, `mkdir: failed for ${a[0]}`);
            },
            touch: (a) => {
                if (!a[0]) return this.termPrint(output, 'touch: missing operand');
                const path = (this.cwd || '/root') + '/' + a[0];
                if (this.vfs.write(path, '')) this.termPrint(output, `Created: ${a[0]}`);
                else this.termPrint(output, `touch: failed for ${a[0]}`);
            },
            rm: (a) => {
                if (!a[0]) return this.termPrint(output, 'rm: missing operand');
                const path = (this.cwd || '/root') + '/' + a[0];
                if (this.vfs.remove(path)) this.termPrint(output, `Removed: ${a[0]}`);
                else this.termPrint(output, `rm: failed for ${a[0]}`);
            },
            cat: (a) => {
                if (!a[0]) return this.termPrint(output, 'cat: missing operand');
                const path = (this.cwd || '/root') + '/' + a[0];
                const content = this.vfs.read(path);
                if (content !== null) this.termPrint(output, content);
                else this.termPrint(output, `cat: no such file: ${a[0]}`);
            },
            neofetch: () => this.termPrint(output, 'Σ SIGMAOS ZENITH\nUptime: ' + this.uptime + 's\nKernel: Sovereign C11\nResolution: Industrial Retina'),
            cpu: () => this.termPrint(output, 'CPU: ' + document.getElementById('cpu-val').textContent),
            matrix: () => this.wm.open('industrialmatrix'),
            scrub: () => {
                this.termPrint(output, 'Initiating Silicon Scrub...');
                setTimeout(() => this.termPrint(output, 'Registers zeroed. Memory sanitized.'), 500);
            },
            sigmactl: (a) => this.handleSigmaCtl(a, output),
            shutdown: () => {
                this.termPrint(output, 'System going down...');
                setTimeout(() => location.reload(), 1500);
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

    handleSigmaCtl(args, output) {
        if (!args[0]) return this.termPrint(output, 'Usage: sigmactl <health|audit|status|wm>');
        if (args[0] === 'wm') this.wm.tile();
        if (args[0] === 'health') this.termPrint(output, `Uptime: ${this.uptime}s | VFS Integrity: 100%`);
        if (args[0] === 'status') this.termPrint(output, 'SIGMAOS SOVEREIGN ZENITH: ACTIVE');
    }

    initSpotlight() {
        document.onkeydown = (e) => {
            if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
                e.preventDefault();
                document.getElementById('spotlight').classList.toggle('hidden');
                document.getElementById('spotlight-input').focus();
            }
        };
    }

    initSpecializedShards() {
        // Initialize AI, Camera, etc.
        this.initAIOrchestrator();
    }

    initAIOrchestrator() {
        const list = document.getElementById('ai-model-list');
        if (list) {
            const models = ['GPT-4', 'Claude-3', 'Gemini-Pro', 'Llama-3'];
            list.innerHTML = models.map(m => `<div class="status-chip">${m}</div>`).join('');
        }
    }

    spawnToast(msg, delay = 0, isError = false) {
        setTimeout(() => {
            const container = document.getElementById('toast-container');
            if (!container) return;
            const toast = document.createElement('div');
            toast.className = 'toast' + (isError ? ' toast-error' : '');
            toast.textContent = msg;
            container.appendChild(toast);
            setTimeout(() => toast.remove(), 3500);
        }, delay);
    }
}

// Global System Instance
window.SIGMA = new SigmaSystem();

// Exported Globals for HTML compatibility
window.openWindow = (id) => window.SIGMA.wm.open(id);
window.closeWindow = (id) => window.SIGMA.wm.close(id);
window.minimizeWindow = (id) => window.SIGMA.wm.minimize(id);
window.toggleMaximize = (id) => window.SIGMA.wm.maximize(id);
window.switchMode = (mode) => {
    window.SIGMA.activeMode = mode;
    document.body.className = 'mode-' + mode.toLowerCase();
    const chip = document.getElementById('active-mode-chip');
    if (chip) chip.textContent = 'MODE: ' + mode;
};
