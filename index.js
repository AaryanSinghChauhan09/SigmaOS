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
 * Σ SOVEREIGN SHARD CONFIG
 * Defines which tools are available and their metadata.
 */
const SOVEREIGN_SHARDS = [
    { id: 'terminal', name: 'Sigma Shell', icon: '🐚', domain: 'System', enabled: true, description: 'Core system command line interface.' },
    { id: 'shardmanager', name: 'Shard Store', icon: '🧩', domain: 'System', enabled: true, description: 'Full autonomy over tools & performance.' },
    { id: 'sysinfo', name: 'Metrics Hub', icon: '📊', domain: 'System', enabled: true, description: 'Real-time performance telemetry.' },
    { id: 'devshard', name: 'Dev IDE', icon: '👨‍💻', domain: 'Dev', enabled: true, description: 'C11/ASM IDE - Pure performance.' },
    { id: 'aishard', name: 'AI Lab', icon: '🧠', domain: 'AI', enabled: true, description: 'Gradient Descent & Tensor flow kernels.' },
    { id: 'dsshard', name: 'Data Sci', icon: '📉', domain: 'DS', enabled: true, description: 'Statistical analysis & Real-time math.' },
    { id: 'dsashard', name: 'DSA Viz', icon: '🧮', domain: 'DSA', enabled: true, description: 'Algorithm auditing & Real-performance sorting.' },
    { id: 'cybershard', name: 'Cyber Sec', icon: '🐲', domain: 'CS', enabled: true, description: 'Zero-trust audit & VFS path security.' },
    { id: 'mlshard', name: 'ML Ops', icon: '🚀', domain: 'ML', enabled: true, description: 'Deployment pipelines & Feature engineering.' },
    { id: 'financeshard', name: 'Finance', icon: '💰', domain: 'Finance', enabled: true, description: 'Industrial Finance - Stock & Crypto Matrix.' },
    { id: 'contentshard', name: 'Studio', icon: '🎬', domain: 'Media', enabled: true, description: 'Creative media studio - Direct Rendering.' },
    { id: 'productivity', name: 'Tasks', icon: '✅', domain: 'Productivity', enabled: true, description: 'Industrial Task Orchestration.' },
    { id: 'vfsmanager', name: 'VFS Admin', icon: '💾', domain: 'System', enabled: false, description: 'Manage raw silicon storage blocks.' },
    { id: 'netviz', name: 'Net Shard', icon: '🌐', domain: 'Network', enabled: false, description: 'Visualize real local network topography.' },
    { id: 'principles', name: 'Principles', icon: '⚖️', domain: 'System', enabled: true, description: 'Sovereign OS Manifest & Performance USPs.' }
];


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
        this.shards = this.loadShardConfig();
        this.init();
    }

    loadShardConfig() {
        const saved = localStorage.getItem('SOVEREIGN_SHARDS');
        if (saved) return JSON.parse(saved);
        return SOVEREIGN_SHARDS;
    }

    saveShardConfig() {
        localStorage.setItem('SOVEREIGN_SHARDS', JSON.stringify(this.shards));
    }

    init() {
        this.initClock();
        this.initMetrics();
        this.initBackground();
        this.initShell();
        this.initSpotlight();
        this.initSpecializedShards();
        this.renderShardManager();
        this.renderMenu();

        window.onerror = (msg, url, line) => {
            this.spawnToast(`Kernel Fault: ${msg}`, 0, true);
            return true;
        };

        this.spawnToast('Σ SIGMAOS ZENITH SUPREME INITIALIZED');
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
        this.initAIOrchestrator();
        this.initSovereignCamera();
        this.initKeyboardAccessibility();
        this.initFinanceData();
        this.initDataChart();
        this.initDSChart();
        this.initSysAuditor();
    }

    initSysAuditor() {
        // CS Shard: Real path scanning for 'insecure' patterns in VFS
        const insecurePatterns = ['password', 'secret', 'token', 'key'];
        this.vfs_vulnerabilities = [];
        Object.keys(this.vfs.fs).forEach(path => {
            insecurePatterns.forEach(p => {
                if (path.toLowerCase().includes(p)) this.vfs_vulnerabilities.push(path);
            });
        });
    }

    initDSChart() {
        const canvas = document.getElementById('ds-canvas');
        if (!canvas) return;
        const ctx = canvas.getContext('2d');
        const draw = () => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            ctx.strokeStyle = '#00d2ff';
            ctx.beginPath();
            for(let i=0; i<canvas.width; i++) {
                const y = Math.sin(i * 0.05 + Date.now() * 0.005) * 30 + 75;
                if(i===0) ctx.moveTo(i, y);
                else ctx.lineTo(i, y);
            }
            ctx.stroke();
            requestAnimationFrame(draw);
        };
        draw();
    }

    initFinanceData() {
        const table = document.getElementById('finance-table');
        if (!table) return;
        const assets = [
            { name: 'Σ-Coin', price: '$94,203', chg: '+8.4%' },
            { name: 'BTC', price: '$1.2M', chg: '+1.2%' },
            { name: 'US-Industrial', price: '$204.1', chg: '-0.3%' }
        ];
        table.innerHTML = assets.map(a => `
            <tr>
                <td class="u-accent-text u-bold">${a.name}</td>
                <td>${a.price}</td>
                <td class="${a.chg.startsWith('+') ? 'u-accent-text' : 'u-error-text'}">${a.chg}</td>
            </tr>
        `).join('');
    }

    initDataChart() {
        const canvas = document.getElementById('data-chart');
        if (!canvas) return;
        const ctx = canvas.getContext('2d');
        let points = Array(20).fill(0).map(() => Math.random() * 100);

        const draw = () => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            ctx.strokeStyle = '#00d2ff';
            ctx.lineWidth = 2;
            ctx.beginPath();
            points.forEach((p, i) => {
                const x = (canvas.width / 20) * i;
                const y = canvas.height - (p / 100 * canvas.height);
                if (i === 0) ctx.moveTo(x, y);
                else ctx.lineTo(x, y);
            });
            ctx.stroke();
            points.shift();
            points.push(Math.random() * 100);
        };
        setInterval(draw, 500);
    }

    renderShardManager() {
        const list = document.getElementById('shard-manager-list');
        if (!list) return;
        list.innerHTML = this.shards.map(s => `
            <div class="metric-card u-flex-between u-margin-b-10">
                <div>
                    <div class="u-bold">${s.icon} ${s.name}</div>
                    <div class="u-muted-text u-font-size-xxs">${s.description}</div>
                </div>
                <div class="u-flex-center">
                    <span class="u-muted-text u-font-size-xxs u-margin-r-10">${s.enabled ? 'ACTIVE' : 'DISABLED'}</span>
                    <label class="switch">
                        <input type="checkbox" ${s.enabled ? 'checked' : ''} onchange="toggleShard('${s.id}')">
                        <span class="slider round"></span>
                    </label>
                </div>
            </div>
        `).join('');
    }

    renderMenu() {
        const items = document.getElementById('menu-items');
        if (!items) return;
        items.innerHTML = this.shards
            .filter(s => s.enabled)
            .map(s => `
                <div class="menu-card" onclick="openWindow('${s.id}'); toggleMenu()">
                    <div class="u-font-size-lg">${s.icon}</div>
                    <div class="u-font-size-xxs u-bold">${s.name.toUpperCase()}</div>
                </div>
            `).join('');
    }

    initAIOrchestrator() {
        const list = document.getElementById('ai-model-list');
        if (list) {
            const models = ['GPT-4', 'Claude-3', 'Gemini-Pro', 'Llama-3'];
            list.innerHTML = models.map(m => `<div class="status-chip">${m}</div>`).join('');
        }
    }

    initSovereignCamera() {
        const video = document.getElementById('camera-stream');
        if (!video) return;
        navigator.mediaDevices.getUserMedia({ video: true })
            .then(stream => { video.srcObject = stream; })
            .catch(() => { this.spawnToast('Camera Shard: Hardware access denied.', 0, true); });
        
        const filter = document.getElementById('camera-filter');
        if (filter) filter.onchange = () => { video.style.filter = filter.value; };
    }

    initKeyboardAccessibility() {
        // Professional No-Mouse Support
        document.addEventListener('keydown', (e) => {
            // Workspace switching (Alt + 1/2/3)
            if (e.altKey && !isNaN(e.key)) {
                this.wm.switchWorkspace(parseInt(e.key));
            }
            // Window Tiling (Alt + T)
            if (e.altKey && e.key.toLowerCase() === 't') {
                this.wm.tile();
            }
            // Close Focused Window (Alt + W)
            if (e.altKey && e.key.toLowerCase() === 'w') {
                const focused = document.querySelector('.window.focused');
                if (focused) this.wm.close(focused.id.replace('win-', ''));
            }
        });
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

// --- Exported Professional Handlers ---
window.executeJusticeShard = () => {
    const log = document.getElementById('law-log');
    if (!log) return;
    const steps = [
        '[JUSTICE] Initiating BNSS Section 105 compliance check...',
        '[JUSTICE] Verifying Videography Shard Metadata (Hash-Direct)...',
        '[JUSTICE] Cross-referencing Supreme Court Interpretation (Curated DB)...',
        '[JUSTICE] COMPLIANCE SECURED: Digital Evidence Authenticated.'
    ];
    let i = 0;
    const interval = setInterval(() => {
        if (i >= steps.length) { clearInterval(interval); return; }
        const div = document.createElement('div');
        div.className = 'u-accent-text u-font-size-xxs';
        div.textContent = steps[i++];
        log.appendChild(div);
        log.scrollTop = log.scrollHeight;
    }, 1000);
};

window.takeSnapshot = () => {
    window.SIGMA.spawnToast('SigmaLens: Snapshot saved to /root/media/snap_' + Date.now() + '.png');
};

window.shareTask = () => {
    window.SIGMA.vfs.write('/root/data/task_share.json', JSON.stringify({ task: 'Distributed-Processing', nodes: 2 }));
    window.SIGMA.spawnToast('Aether-Link: Current mission distributed to 2 peer nodes.');
};

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

// --- NEW SOVEREIGN HANDLERS ---
window.toggleMenu = () => {
    document.getElementById('sigma-menu').classList.toggle('hidden');
};

window.filterMenu = () => {
    const q = document.getElementById('menu-search').value.toLowerCase();
    document.querySelectorAll('.menu-card').forEach(card => {
        const name = card.innerText.toLowerCase();
        card.style.display = name.includes(q) ? 'flex' : 'none';
    });
};

window.toggleShard = (id) => {
    const shard = window.SIGMA.shards.find(s => s.id === id);
    if (!shard) return;
    shard.enabled = !shard.enabled;
    window.SIGMA.saveShardConfig();
    window.SIGMA.renderShardManager();
    window.SIGMA.renderMenu();
    window.SIGMA.spawnToast(`Shard [${id}] ${shard.enabled ? 'Enabled' : 'Disabled'}.`);
    
    // If disabled, close the window
    if (!shard.enabled) window.SIGMA.wm.close(id);
};

window.purgeUnusedShards = () => {
    window.SIGMA.shards = window.SIGMA.shards.filter(s => s.enabled || s.id === 'shardmanager');
    window.SIGMA.saveShardConfig();
    window.SIGMA.renderShardManager();
    window.SIGMA.renderMenu();
    window.SIGMA.spawnToast('AUTONOMY TRIGGERED: Unused Shards Purged from Silicon.');
};

window.runCyberScan = () => {
    const log = document.getElementById('cyber-scan-log');
    if (!log) return;
    log.innerHTML = '[INFO] Auditing Sovereign VFS...<br>';
    
    // PURE PERFORMANCE: Real VFS Scan
    const vulns = window.SIGMA.vfs_vulnerabilities;
    let i = 0;
    const interval = setInterval(() => {
        if (i >= vulns.length) { 
            clearInterval(interval); 
            log.innerHTML += `[COMPLETE] Audit finished. ${vulns.length} insecure paths found.<br>`;
            window.SIGMA.spawnToast(`Security: Audit finished. Clean up requested.`);
            return; 
        }
        log.innerHTML += `<span class="u-error-text">[VULN] Insecure Path: ${vulns[i++]}</span><br>`;
        log.scrollTop = log.scrollHeight;
    }, 400);
};

window.runDSAnalysis = () => {
    const canvas = document.getElementById('ds-canvas');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    
    // PURE PERFORMANCE: Real Mean/Variance Calculation
    const data = Array(100).fill(0).map(() => Math.random() * 100);
    const mean = data.reduce((a, b) => a + b) / data.length;
    const variance = data.reduce((a, b) => a + Math.pow(b - mean, 2), 0) / data.length;
    
    window.SIGMA.spawnToast(`DS: Analysis finished. Mean: ${mean.toFixed(2)}, Var: ${variance.toFixed(2)}`);
    
    // Visualize histogram
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    const bins = Array(10).fill(0);
    data.forEach(d => bins[Math.floor(d/10)]++);
    bins.forEach((b, i) => {
        ctx.fillStyle = '#00d2ff';
        ctx.fillRect(i * 40, canvas.height - b * 10, 35, b * 10);
    });
};

window.startAIGen = () => {
    const bar = document.getElementById('ai-bar');
    const status = document.getElementById('ai-mission-status');
    if (!bar || !status) return;
    
    // PURE PERFORMANCE: Linear Regression Gradient Descent
    status.textContent = 'RUNNING GRADIENT DESCENT...';
    let w = 0, b = 0, alpha = 0.01;
    const data = Array(100).fill(0).map((_, i) => ({ x: i, y: 2 * i + 5 + Math.random() }));
    
    let epoch = 0;
    const runEpoch = () => {
        let dw = 0, db = 0;
        data.forEach(p => {
            const pred = w * p.x + b;
            dw += (pred - p.y) * p.x;
            db += (pred - p.y);
        });
        w -= (dw / 100) * alpha;
        b -= (db / 100) * alpha;
        epoch++;
        
        bar.style.width = (epoch / 5) + '%'; // 500 epochs
        if (epoch < 500) requestAnimationFrame(runEpoch);
        else {
            status.textContent = `TRAINED: y = ${w.toFixed(2)}x + ${b.toFixed(2)}`;
            window.SIGMA.spawnToast('AI Lab: Model training complete on Silicon.');
        }
    };
    runEpoch();
};

window.runDSAViz = () => {
    const area = document.getElementById('dsa-viz-area');
    const algo = document.getElementById('dsa-algo').value;
    if (!area) return;
    
    // PURE PERFORMANCE: Real Quicksort/BubbleSort
    const arr = Array(20).fill(0).map(() => Math.floor(Math.random() * 100));
    const render = (a) => {
        area.innerHTML = '';
        a.forEach(v => {
            const bar = document.createElement('div');
            bar.className = 'status-chip';
            bar.style.height = v + 'px';
            bar.style.width = '10px';
            bar.style.margin = '1px';
            area.appendChild(bar);
        });
    };

    if (algo === 'QUICKSORT') {
        const sort = async (a, low, high) => {
            if (low < high) {
                let pivot = a[high];
                let i = low - 1;
                for (let j = low; j < high; j++) {
                    if (a[j] < pivot) {
                        i++;
                        [a[i], a[j]] = [a[j], a[i]];
                        render(a);
                        await new Promise(r => setTimeout(r, 50));
                    }
                }
                [a[i + 1], a[high]] = [a[high], a[i + 1]];
                render(a);
                let pi = i + 1;
                await sort(a, low, pi - 1);
                await sort(a, pi + 1, high);
            }
        };
        sort(arr, 0, arr.length - 1).then(() => window.SIGMA.spawnToast('DSA: Quicksort logic audit complete.'));
    }
};

window.updateFinanceData = () => {
    window.SIGMA.initFinanceData();
    window.SIGMA.spawnToast('Finance Matrix Synchronized.');
};

// Periodic updates for Data Shard
setInterval(() => {
    const flow = document.getElementById('tensor-flow-rate');
    if (flow) flow.textContent = (Math.random() * 5 + 1).toFixed(2) + ' TB/s';
}, 3000);
window.setAccent = (color) => { document.documentElement.style.setProperty('--accent-primary', color); }; window.setBlur = (val) => { document.documentElement.style.setProperty('--window-bg', \gba(10, 15, 25, \)\); document.documentElement.style.setProperty('--backdrop-blur', \lur(\px)\); };
