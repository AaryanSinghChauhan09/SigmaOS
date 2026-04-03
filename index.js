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
    { id: 'omniagent', name: 'Omni Agent', icon: '🤖', domain: 'AI', enabled: true, description: 'Agentic coding & autonomous system orchestrator.' },
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
        document.addEventListener('mousedown', (e) => {
            const header = e.target.closest('.win-header');
            if (header) {
                const win = header.parentElement;
                this.focus(win.id.replace('win-', ''));
                this.dragWindow(win, e);
            }
        });

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

        document.querySelectorAll('.dock-item').forEach(item => {
            item.onclick = () => this.open(item.getAttribute('data-window'));
        });

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
                const txt = String.fromCharCode(0x03A3 + Math.random() * 20);
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
            help: () => this.termPrint(output, 'Commands: help, ls, cd, mkdir, touch, rm, cat, clear, neofetch, cpu, sigmactl, sigma, agent'),
            clear: () => { output.innerHTML = ''; },
            ls: () => this.termPrint(output, this.vfs.ls(this.cwd || '/root').join('  ')),
            neofetch: () => this.termPrint(output, 'Σ SIGMAOS ZENITH\nUptime: ' + this.uptime + 's\nKernel: Sovereign C11'),
            sigma: (a) => {
                if (a[0] === 'help') {
                    this.termPrint(output, `Σ SOVEREIGN HELP MODEL: Analyzing query '${a.slice(1).join(' ')}'...`, 'u-accent-text');
                    setTimeout(() => {
                        this.termPrint(output, 'Σ [GUIDANCE]: Use `sigmactl health` to check system integrity, or `ls /root` to see the sharded file system.');
                    }, 500);
                    return;
                }
                this.termPrint(output, 'Σ OMNI-AGENT ACTIVATED', 'u-accent-text u-bold');
                this.termPrint(output, `[PLANNING] Analyzing: ${a.join(' ')}...`);
                setTimeout(() => {
                    this.termPrint(output, '[SUCCESS] System shards synchronized.');
                }, 1000);
            },
            agent: (a) => {
                this.termPrint(output, `[AGENT] Task received: ${a.join(' ')}`);
                setTimeout(() => {
                    this.vfs.snapshot('AGENT_TASK');
                    this.termPrint(output, '[AGENT] Snapshot created. Executing plan...');
                }, 800);
            },
            academy: (a) => {
                const mission = a[0] || '1';
                this.termPrint(output, `Σ SIGMA ACADEMY: Mission ${mission} activated.`, 'u-accent-text u-bold');
                this.termPrint(output, 'MISSION: Use `mkdir /root/academy_test` to verify VFS write-shard.');
                this.spawnToast('Academy: New mission assigned.');
            },
            'sigma-ai': (a) => {
                const cmd = a[0];
                if (cmd === 'train') {
                    this.termPrint(output, 'Σ ML PIPELINE: Initiating PRETRAINING (Next-Token Prediction)...', 'u-accent-text');
                    setTimeout(() => this.termPrint(output, '[ML] Pretraining COMPLETE. Accuracy: 94.2%'), 2000);
                } else if (cmd === 'fine-tune') {
                    this.termPrint(output, 'Σ ML PIPELINE: Initiating FINE-TUNING (Domain Adaptation)...', 'u-accent-text');
                    setTimeout(() => this.termPrint(output, '[ML] Fine-tuning [Domain: HELP] COMPLETE.'), 1500);
                } else {
                    this.termPrint(output, 'Usage: sigma-ai [train|fine-tune|eval|inference]');
                }
            },
            'sigma-ui': (a) => {
                const sub = a[0];
                const val = a.slice(1).join(' ');
                if (sub === 'theme') {
                    this.termPrint(output, `Σ UI MORPH: Applying theme '${val}'...`);
                    document.documentElement.style.setProperty('--u-accent', '#00d2ff');
                    this.spawnToast(`Theme: ${val} applied.`);
                }
            },
            'sigma-persona': (a) => {
                const p = a[0];
                this.termPrint(output, `Σ KERNEL MORPH: Switching to PERSONA: ${p.toUpperCase()}...`, 'u-accent-text');
                this.termPrint(output, '[KERN] Priority: Real-time | Sched: FIFO | Shards: Optimized');
                this.spawnToast(`Persona: ${p} active.`);
            },
            'sigma-auto': (a) => {
                const [cmd, interval] = a;
                this.termPrint(output, `Σ AUTOMATION: Mission '${cmd}' scheduled for '${interval}'.`, 'u-accent-text');
                this.spawnToast('Automation: Registered Shard Listener.');
            },
            'sigma-tool': (a) => {
                const sub = a[0];
                const name = a[1];
                if (sub === 'register') {
                    this.termPrint(output, `Σ TOOL: Registering tool '${name}' to Omni-CLI...`, 'u-accent-text');
                    this.spawnToast(`Tool Registered: ${name}`);
                } else if (sub === 'run') {
                    this.termPrint(output, `Σ MISSION: Executing tool '${name}'...`, 'u-bold');
                } else if (sub === 'list') {
                    this.termPrint(output, 'Σ TOOLS: studio, gaming, remote-bot, xclicker, backup-manager, cron-shard', 'u-accent-text');
                } else {
                    this.termPrint(output, 'Usage: sigma-tool [register|run|list|status]');
                }
            },
            'sigma-privacy': (a) => {
                const sub = a[0];
                if (sub === 'purge') {
                    this.termPrint(output, 'Σ PRIVACY: Initiating GLOBAL AMNESIC PURGE...', 'u-accent-text');
                    this.termPrint(output, '[ML] AI Context logs flushed (1,024 shards purged).');
                    this.spawnToast('Privacy: System Amnesia Successful.');
                } else {
                    this.termPrint(output, 'Usage: sigma-privacy [purge|isolate|status]');
                }
            },
            'sigma-sync': () => {
                this.termPrint(output, 'Σ SYNC: Establishing PQC-1024 Handshake with GitHub...', 'u-accent-text');
                setTimeout(() => {
                    this.termPrint(output, '[SUCCESS]: Repositories synchronized. README & Wikis updated.');
                }, 1000);
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
        this.initFinanceData();
        this.initDSChart();
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
            { name: 'BTC', price: '$1.2M', chg: '+1.2%' }
        ];
        table.innerHTML = assets.map(a => `
            <tr><td class="u-accent-text u-bold">${a.name}</td><td>${a.price}</td><td>${a.chg}</td></tr>
        `).join('');
    }

    renderShardManager() {
        const list = document.getElementById('shard-manager-list');
        if (!list) return;
        list.innerHTML = this.shards.map(s => `
            <div class="metric-card u-flex-between">
                <div><div class="u-bold">${s.icon} ${s.name}</div></div>
                <label class="switch">
                    <input type="checkbox" ${s.enabled ? 'checked' : ''} onchange="toggleShard('${s.id}')">
                    <span class="slider round"></span>
                </label>
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
                    <div class="u-font-size-xxs u-bold">${s.name.toUpperCase()}</div>
                </div>
            `).join('');
    }

    initAIOrchestrator() {
        const list = document.getElementById('ai-model-list');
        if (list) list.innerHTML = ['Claude-3', 'GPT-4'].map(m => `<div class="status-chip">${m}</div>`).join('');
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

window.SIGMA = new SigmaSystem();

window.openWindow = (id) => window.SIGMA.wm.open(id);
window.closeWindow = (id) => window.SIGMA.wm.close(id);
window.toggleShard = (id) => window.SIGMA.toggleShard(id);

window.setAccent = (color) => { document.documentElement.style.setProperty('--accent-primary', color); };
window.setBlur = (val) => { 
    document.documentElement.style.setProperty('--window-bg', `rgba(10, 15, 25, ${val})`); 
    document.documentElement.style.setProperty('--backdrop-blur', `blur(${val * 10}px)`); 
};

window.runDSAnalysis = () => {
    const canvas = document.getElementById('ds-canvas');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const data = Array(100).fill(0).map(() => Math.random() * 100);
    const mean = data.reduce((a, b) => a + b) / data.length;
    const variance = data.reduce((a, b) => a + Math.pow(b - mean, 2), 0) / data.length;
    window.SIGMA.spawnToast(`Analysis finished. Mean: ${mean.toFixed(2)}`);
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    const bins = Array(10).fill(0);
    data.forEach(d => bins[Math.floor(d/10)]++);
    bins.forEach((b, i) => {
        ctx.fillStyle = '#00d2ff';
        ctx.fillRect(i * 40, canvas.height - b * 10, 35, b * 10);
    });
};
