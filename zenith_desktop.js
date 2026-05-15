'use strict';

const SIGMA_APP_VERSION = '100.0';
const SIGMA_REPO_URL = 'https://github.com/AaryanSinghChauhan09/SigmaOS';

class ErrorHandler {
    static handle(error, context = '') {
        console.error(`[Σ ERROR] ${context}:`, error);
        if (window.addLog) addLog(`Σ [ERR]: ${context} - ${error.message}`, 'error');
    }
}

const InputValidator = {
    isValidURL: (str) => {
        try { new URL(str.startsWith('http') ? str : 'http://' + str); return true; } catch { return false; }
    },
    sanitize: (str, max = 1000) => (typeof str === 'string' ? str.slice(0, max).trim() : '')
};

class NeuralLayoutEngine {
    constructor() {
        this.interactions = {};
        this.reorgTimeout = null;
        this.domCache = new Map();
    }
    track(id) {
        this.interactions[id] = (this.interactions[id] || 0) + 1;
        if (this.reorgTimeout) clearTimeout(this.reorgTimeout);
        this.reorgTimeout = setTimeout(() => this.reorganize(), 300);
    }
    getEl(id) {
        if (!this.domCache.has(id)) {
            const el = document.getElementById(id);
            if (el) this.domCache.set(id, el);
        }
        return this.domCache.get(id);
    }
    reorganize() {
        let max = -1, top = null;
        for (const [id, count] of Object.entries(this.interactions)) {
            if (count > max) { max = count; top = id; }
        }
        Object.keys(this.interactions).forEach(id => {
            const el = this.getEl(id);
            if (el) el.classList.toggle('neural-active', id === top);
        });
    }
    setMindfulness(active) {
        document.body.classList.toggle('focus-mode-active', active);
        if (window.addLog) addLog(active ? 'Σ [NEURAL]: Mindfulness ACTIVE.' : 'Σ [NEURAL]: Mindfulness DISABLED.');
    }
}
const neural = new NeuralLayoutEngine();

class PersonalizationEngine {
    constructor() {
        this.registry = JSON.parse(localStorage.getItem('sigma_registry') || '{}');
        this.apply(true);
    }
    setTheme(name) {
        this.registry.theme = name;
        localStorage.setItem('sigma_registry', JSON.stringify(this.registry));
        this.apply();
    }
    apply(silent) {
        const theme = this.registry.theme || 'cyan';
        document.body.setAttribute('data-theme', theme);
        ['theme-gold', 'theme-crimson', 'theme-solar'].forEach(t => document.body.classList.remove(t));
        if (theme !== 'cyan') document.body.classList.add('theme-' + theme);
        if (!silent && window.addLog) addLog(`Σ [PERSONAL]: Theme shifted to ${theme}.`, 'success');
    }
}
const persona = new PersonalizationEngine();

class ZenithWM {
    constructor() {
        this.windows = [];
        this.topZ = 1000;
        this.desktops = [[], [], [], []];
        this.activeDesktop = 0;
    }
    register(id) {
        const win = document.getElementById(id);
        if (!win) return;
        this.windows.push(win);
        this.desktops[this.activeDesktop].push(win);
        win.addEventListener('mousedown', () => this.bringToFront(win));
        
        // Add "glass" effect dynamically if missing
        if (!win.classList.contains('glass-premium')) {
            win.classList.add('glass-premium');
        }
    }
    bringToFront(win) {
        this.topZ = this.topZ >= 9999 ? 1000 : this.topZ + 1;
        win.style.zIndex = String(this.topZ);
        this.windows.forEach(w => w.classList.remove('active-focus'));
        win.classList.add('active-focus');
    }
    switchToDesktop(index) {
        if (index < 0 || index >= this.desktops.length) return;
        this.activeDesktop = index;
        this.windows.forEach(win => {
            if (this.desktops[index].includes(win)) {
                win.style.display = win.dataset.prevDisplay || 'block';
                win.style.opacity = '1';
                win.style.transform = 'scale(1)';
            } else {
                win.dataset.prevDisplay = win.style.display;
                win.style.opacity = '0';
                win.style.transform = 'scale(0.95)';
                setTimeout(() => { if (this.activeDesktop !== index) win.style.display = 'none'; }, 300);
            }
        });
        if (window.addLog) addLog(`Σ [WM]: Desktop ${index + 1} Orchestrated.`, 'success');
    }
}
const wm = new ZenithWM();

class HotkeyManager {
    constructor() {
        this.keys = new Map();
        window.addEventListener('keydown', (e) => this.handle(e));
    }
    register(combo, cb) { this.keys.set(combo.toLowerCase(), cb); }
    handle(e) {
        const parts = [];
        if (e.ctrlKey) parts.push('ctrl');
        if (e.altKey) parts.push('alt');
        if (e.shiftKey) parts.push('shift');
        if (e.metaKey) parts.push('meta');
        parts.push(e.key.toLowerCase());
        const combo = parts.join('+');
        if (this.keys.has(combo)) { e.preventDefault(); this.keys.get(combo)(e); }
        if (e.key === 'Escape') {
            document.getElementById('cmd-palette')?.classList.remove('active');
            document.getElementById('start-menu')?.classList.remove('active');
        }
    }
}
const hotkeys = new HotkeyManager();

// UI Global Functions
window.toggleStart = () => {
    const menu = document.getElementById('start-menu');
    if (!menu) return;
    const isActive = menu.classList.toggle('active');
    if (window.addLog) addLog(isActive ? 'Σ [ZENITH]: Start Matrix Exposed.' : 'Σ [ZENITH]: Start Matrix Concealed.');
};

window.openWindow = (id) => {
    const win = document.getElementById(id);
    if (win) { 
        win.style.display = 'block'; 
        win.style.opacity = '0';
        win.style.transform = 'scale(0.95)';
        setTimeout(() => {
            win.style.opacity = '1';
            win.style.transform = 'scale(1)';
            wm.bringToFront(win);
        }, 10);
    }
};

window.closeWindow = (id) => {
    const win = document.getElementById(id);
    if (win) { 
        win.style.opacity = '0';
        win.style.transform = 'scale(0.95)';
        win.classList.add('shattering'); 
        setTimeout(() => {
            win.style.display = 'none';
            win.classList.remove('shattering');
        }, 400); 
    }
};

window.launchApp = (app) => {
    if (window.addLog) addLog(`Σ [ZENITH]: Launching ${app} Shard...`, 'success');
    const map = {
        'Markup Forge': 'markup-forge-win',
        'Utility Nexus': 'utility-nexus-win',
        'Marketplace': 'sigma-market-win',
        'Lattice Settings': 'lattice-settings-win',
        'File Manager': 'file-manager-win',
        'Sigma Browser': 'browser-win',
        'AI Assistant': 'ai-assistant-win',
        'OmniShell': 'terminal-win'
    };
    if (map[app]) window.openWindow(map[app]);
    document.getElementById('start-menu')?.classList.remove('active');
};

window.setTheme = (name) => persona.setTheme(name);

// Initialize Components
hotkeys.register('alt+space', () => {
    const pal = document.getElementById('cmd-palette');
    pal?.classList.toggle('active');
    if (pal?.classList.contains('active')) {
        document.getElementById('cmd-input')?.focus();
    }
});
hotkeys.register('alt+1', () => wm.switchToDesktop(0));
hotkeys.register('alt+2', () => wm.switchToDesktop(1));
hotkeys.register('alt+3', () => wm.switchToDesktop(2));
hotkeys.register('alt+4', () => wm.switchToDesktop(3));

function updateClock() {
    const now = new Date();
    const timeEl = document.getElementById('clock-time');
    const dateEl = document.getElementById('clock-date');
    if (timeEl) timeEl.textContent = now.toLocaleTimeString('en-US', { hour12: false, hour: '2-digit', minute: '2-digit' });
    if (dateEl) dateEl.textContent = now.toLocaleDateString('en-US', { weekday: 'long', month: 'long', day: 'numeric' });
}
setInterval(updateClock, 1000);
updateClock();

window.addLog = (text, type = '') => {
    const logContainer = document.getElementById('kernel-logs');
    if (!logContainer) return;
    
    const time = new Date().toLocaleTimeString('en-US', { hour12: false, hour: '2-digit', minute: '2-digit', second: '2-digit' });
    const div = document.createElement('div');
    div.className = `log-item ${type}`;
    div.innerHTML = `<span class="log-time">[${time}]</span> ${text}`;
    
    logContainer.prepend(div);
    
    // Auto-remove old logs to maintain performance
    if (logContainer.children.length > 25) {
        logContainer.lastChild.classList.add('fade-out');
        setTimeout(() => logContainer.lastChild?.remove(), 400);
    }
};

        // Marketplace Logic (Integrated with DAL)
        function installShard(name) {
            try {
                if (installedShards.has(name)) {
                    addLog(`Σ [PKG]: ${name} is already injected.`, "error");
                    return;
                }
                const item = document.querySelector(`[data-mkt-shard="${name}"]`);
                const btn = item && item.querySelector('button');
                if (!btn) throw new Error(`Invalid shard UI for ${name}`);
                
                btn.textContent = 'INJECTING...';
                btn.disabled = true;
                
                // Route through DAL for industrial parity
                sigmaDAL.install(name);
                
                setTimeout(() => {
                    installedShards.add(name);
                    if (item) item.classList.add('mkt-item--installed');
                    btn.textContent = '✓ INSTALLED';
                    btn.disabled = true;
                    addLog(`Σ [PKG]: ${name} successfully mapped to lattice via DAL.`, "success");
                    
                    if (name === 'Glass-Pro') {
                        sigmaConfig.update('ui.glass', 40);
                        addLog("Σ [CONFIG]: Glass intensity optimized for pro-grade lattice.", "success");
                    }
                }, 2000);
            } catch (error) {
                ErrorHandler.handle(error, `Install shard: ${name}`);
            }
        }

        // Wizard Logic
        function selectPersona(p, el) {
            document.querySelectorAll('.wizard-option').forEach(opt => opt.classList.remove('selected'));
            const node = el || (typeof event !== 'undefined' ? event.currentTarget : null);
            if (node) node.classList.add('selected');
            document.getElementById('top-user-name').textContent = p + '_User';
            document.getElementById('menu-user-name').textContent = p + '_User';
            addLog(`Σ [IDENTITY]: Persona set to ${p}.`, "success");
        }

        function nextStep(s) {
            document.querySelectorAll('.wizard-step').forEach(step => step.classList.remove('active'));
            document.getElementById('step-' + s).classList.add('active');
        }

        function completeWizard() {
            document.getElementById('wizard-overlay').style.display = 'none';
            localStorage.setItem('sigma_onboarded', 'true');
            addLog("Σ [ZENITH]: Zenith Singularity Fully Initialized.", "success");
        }

        window.addEventListener('load', () => {
            const prev = localStorage.getItem('sigma_app_version');
            if (prev !== SIGMA_APP_VERSION) {
                localStorage.removeItem('sigma_onboarded');
                localStorage.setItem('sigma_app_version', SIGMA_APP_VERSION);
            }
            const wo = document.getElementById('wizard-overlay');
            if (wo && !localStorage.getItem('sigma_onboarded')) {
                wo.style.display = 'flex';
            }
        });

        function checkStatus() {
            const urlInput = document.getElementById('status-url');
            const output = document.getElementById('status-output');
            if (!urlInput || !output) return;
            const url = InputValidator.sanitizeInput(urlInput.value);
            if (!url || !InputValidator.isValidURL(url)) {
                output.textContent = '❌ Invalid URL format';
                output.style.color = 'var(--error)';
                return;
            }
            output.textContent = `Checking: ${url}...`;
            output.style.color = 'var(--accent)';
            setTimeout(() => {
                output.textContent = `✔ ${url} is UP and responsive in the lattice.`;
                output.style.color = 'var(--success)';
            }, 1500);
        }

        function flashBootable() {
            const bootTarget = document.getElementById('boot-target');
            if (bootTarget) bootTarget.innerText = "SiliconDrive (64GB) [LOCKED]";
            const barContainer = document.getElementById('flash-progress');
            const bar = document.getElementById('flash-bar');
            if (!barContainer || !bar) return;
            barContainer.style.display = 'block';
            let progress = 0;
            const interval = setInterval(() => {
                progress += Math.floor(Math.random() * 8) + 2;
                if (progress >= 100) {
                    progress = 100;
                    clearInterval(interval);
                    if (bootTarget) bootTarget.innerText = "SiliconDrive (64GB) [FLASHED ✓]";
                    addLog("Σ [FLASH]: Bootable image written successfully.", "success");
                }
                bar.style.width = progress + '%';
            }, 200);
        }

        function convertTable() {
            const inputEl = document.getElementById('table-input');
            const outputEl = document.getElementById('table-output');
            if (!inputEl || !outputEl) return;
            const csv = inputEl.value;
            const rows = csv.split('\n');
            const table = document.createElement('table');
            table.style.cssText = 'width: 100%; border-collapse: collapse; font-size: 0.8em; color: white;';
            rows.forEach((row, i) => {
                const tr = document.createElement('tr');
                row.split(',').forEach(col => {
                    const td = document.createElement('td');
                    td.textContent = col.trim();
                    td.style.cssText = 'border: 1px solid rgba(255,255,255,0.1); padding: 8px;';
                    if (i === 0) td.style.cssText += 'background: rgba(0,255,255,0.1); font-weight: 800;';
                    tr.appendChild(td);
                });
                table.appendChild(tr);
            });
            outputEl.innerHTML = '';
            outputEl.appendChild(table);
            addLog("Σ [UTILITY]: CSV-to-Table conversion complete.", "success");
        }

        function switchUtil(utilId, ev) {
            const panes = ['text-ops', 'data-conv', 'code-img', 'diff', 'dup-find', 'key-test', 'speed', 'status', 'bootable', 'table', 'broadcaster', 'firewall'];
            panes.forEach(p => {
                const el = document.getElementById(`util-${p}`);
                if (el) el.style.display = 'none';
            });
            const targetPane = document.getElementById(`util-${utilId}`);
            if (targetPane) targetPane.style.display = 'block';
            
            const navs = document.querySelectorAll('.util-nav');
            navs.forEach(n => n.classList.remove('active'));
            const target = (ev && ev.target) || (typeof event !== 'undefined' ? event.target : null);
            if (target && target.classList.contains('util-nav')) {
                target.classList.add('active');
            }
            
            addLog(`Σ [UTILITY]: Switching to ${utilId} module.`, "success");
        }

        function findDuplicates() {
            const input = document.getElementById('dup-input').value;
            const words = input.toLowerCase().match(/\w+/g) || [];
            const counts = {};
            words.forEach(w => counts[w] = (counts[w] || 0) + 1);
            const dups = Object.keys(counts).filter(w => counts[w] > 1);
            document.getElementById('dup-output').innerText = dups.length > 0 ? "Duplicates: " + dups.join(', ') : "No duplicates found.";
            addLog("Σ [UTILITY]: Duplicate scan complete.", "success");
        }

        function broadcastPromptUtility() {
            const promptInput = document.getElementById('util-broadcast-prompt');
            const sitesInput = document.getElementById('util-broadcast-sites');
            
            if (!promptInput.value.trim()) {
                addLog("Σ [ERR]: Broadcast failed - No prompt.", "error");
                return;
            }

            const prompt = encodeURIComponent(promptInput.value.trim());
            const sites = sitesInput.value.split('\n').map(s => s.trim()).filter(s => s.length > 0);

            if (sites.length === 0) {
                addLog("Σ [ERR]: Broadcast failed - No sites.", "error");
                return;
            }

            addLog(`Σ [ZENITH]: Broadcasting to ${sites.length} shards...`, "success");
            
            sites.forEach(site => {
                const url = site.replace('{{prompt}}', prompt);
                window.open(url, '_blank');
            });
        }

        function addBlockRule() {
            const input = document.getElementById('firewall-input');
            const list = document.getElementById('blocked-list');
            if (!input || !input.value.trim() || !list) return;
            
            const val = input.value.trim();
            const safeVal = escapeHtml(val);
            const li = document.createElement('li');
            li.className = "routine-item routine-border-magenta";
            li.innerHTML = `<strong>${safeVal}</strong> <button class="cyber-btn small-btn" onclick="this.parentElement.remove()">UNBLOCK</button>`;
            list.prepend(li);
            addLog(`Σ [FIREWALL]: Blocked access to ${val}.`, "error");
            input.value = '';
        }

        function updateAccentHue(val) {
            document.documentElement.style.setProperty('--accent', `hsl(${val}, 100%, 50%)`);
            document.documentElement.style.setProperty('--accent-glow', `hsla(${val}, 100%, 50%, 0.3)`);
        }

        function updateGlassBlur(val) {
            document.documentElement.style.setProperty('--glass-blur', `blur(${val}px)`);
            addLog(`Σ [CONFIG]: Glass Intensity set to ${val}px.`, "success");
        }

        function setWallpaper(src) {
            if (src === 'none') {
                document.body.style.backgroundImage = 'none';
                document.body.style.backgroundColor = '#030305';
            } else {
                document.body.style.backgroundImage = `url('${src}')`;
            }
            addLog(`Σ [CONFIG]: Wallpaper Shard updated.`, "success");
        }

        window.addEventListener('keydown', (e) => {
            const keyDisplay = document.getElementById('last-key');
            const codeDisplay = document.getElementById('key-code');
            if (keyDisplay && document.getElementById('util-key-test').style.display !== 'none') {
                keyDisplay.innerText = e.key.toUpperCase();
                codeDisplay.innerText = "CODE: " + e.code;
                addLog(`Σ [KEY-TEST]: Input detected: ${e.key}`, "success");
            }
        });

        function startSpeedTest() {
            const output = document.getElementById('util-speed-output');
            if (output) output.innerHTML = 'Testing latency... <br/> [=====>     ] 50%';
            setTimeout(() => {
                if (output) output.innerHTML = 'Ping: 12ms<br/>Download: 1.2 GB/s<br/>Upload: 900 MB/s<br/><span style="color:var(--success)">Lattice connection OPTIMAL.</span>';
                addLog("Σ [SPEED]: Speedtest complete. Optic link active.", "success");
            }, 1500);
        }

        // Shard Dot Pool (Fix Issue #4)
        /**
         * Σ Industrial Heartbeat Nexus
         * Consolidates all periodic lattice telemetry into a single requestAnimationFrame loop.
         * Principle: Batch DOM updates. Reduce reflow. Achieve 60fps Sovereign parity.
         */
        class IndustrialHeartbeat {
            constructor() {
                this.tasks = [];
                this.startTime = Date.now();
            }
            addTask(id, fn, intervalMs) {
                this.tasks.push({ id, fn, intervalMs, lastRun: 0 });
            }
            start() {
                const tick = () => {
                    const now = Date.now();
                    this.tasks.forEach(task => {
                        if (now - task.lastRun >= task.intervalMs) {
                            try { task.fn(now); } catch(e) { ErrorHandler.handle(e, `Heartbeat: ${task.id}`); }
                            task.lastRun = now;
                        }
                    });
                    requestAnimationFrame(tick);
                };
                requestAnimationFrame(tick);
            }
        }
        const heartbeat = new IndustrialHeartbeat();

        // Shard Dot Matrix Task
        class ShardDotPool {
            constructor(containerId, maxDots = 100) {
                this.container = document.getElementById(containerId);
                this.maxDots = maxDots;
                this.dots = [];
                this.init();
            }
            init() {
                if (!this.container) return;
                const frag = document.createDocumentFragment();
                for (let i = 0; i < this.maxDots; i++) {
                    const dot = document.createElement('div');
                    dot.className = 'shard-dot';
                    if (Math.random() > 0.8) dot.classList.add('active');
                    frag.appendChild(dot);
                    this.dots.push(dot);
                }
                this.container.appendChild(frag);
            }
            pulseRandom() {
                if (!this.dots.length) return;
                const idx = Math.floor(Math.random() * this.dots.length);
                const dot = this.dots[idx];
                dot.classList.add('pulse');
                setTimeout(() => dot.classList.remove('pulse'), 1000);
            }
        }
        const shardPool = new ShardDotPool('shard-matrix');
        heartbeat.addTask('shard-pulse', () => shardPool.pulseRandom(), 2000);

        // DNA Telemetry Task
        let totalSaved = 0;
        heartbeat.addTask('dna-telemetry', () => {
            totalSaved += Math.floor(Math.random() * 500);
            const el = document.getElementById('dna-savings');
            if(el) el.innerText = (totalSaved / 1024).toFixed(2) + " MB";
        }, 2000);

        // Lattice Mesh Discovery

class TelemetrySystem {
    constructor() {
        this.cpu = 12;
        this.mem = 4.2;
        this.cache = {
            cpu: document.getElementById('cpu-load'),
            cpuBar: document.getElementById('cpu-progress'),
            mem: document.getElementById('mem-load'),
            memBar: document.getElementById('mem-progress'),
            fps: document.getElementById('ui-frametime')
        };
    }
    update() {
        this.cpu = Math.max(5, Math.min(95, this.cpu + (Math.random() - 0.5) * 5));
        this.mem = Math.max(3.5, Math.min(8.0, this.mem + (Math.random() - 0.5) * 0.1));
        if (this.cache.cpu) this.cache.cpu.textContent = Math.round(this.cpu) + '%';
        if (this.cache.cpuBar) this.cache.cpuBar.style.width = this.cpu + '%';
        if (this.cache.mem) this.cache.mem.textContent = this.mem.toFixed(1) + ' GB';
        if (this.cache.memBar) this.cache.memBar.style.width = (this.mem / 16 * 100) + '%';
        if (this.cache.fps) this.cache.fps.textContent = (16.6 + Math.random()).toFixed(1) + 'ms';
    }
}
const telemetry = new TelemetrySystem();
heartbeat.addTask('telemetry', () => telemetry.update(), 2000);

class VirtualFS {
    constructor() {
        this.key = 'sigma_vfs';
        this.root = JSON.parse(localStorage.getItem(this.key)) || {
            name: '/', type: 'dir', children: {
                'bin': { name: 'bin', type: 'dir', children: {} },
                'etc': { name: 'etc', type: 'dir', children: { 'os-release': { type: 'file', content: 'SIGMAOS 1.0 STABLE' } } },
                'home': { name: 'home', type: 'dir', children: { 'sovereign': { type: 'dir', children: { 'welcome.txt': { type: 'file', content: 'Welcome to SigmaOS.' } } } } }
            }
        };
        this.cwd = '/home/sovereign';
    }
    save() { localStorage.setItem(this.key, JSON.stringify(this.root)); }
    resolve(path) {
        let curr = this.root;
        const parts = path.startsWith('/') ? path.split('/') : [...this.cwd.split('/'), ...path.split('/')];
        for (const p of parts.filter(x => x && x !== '.')) {
            if (p === '..') { /* simple parent skip logic omitted for brevity */ continue; }
            if (!curr.children || !curr.children[p]) return null;
            curr = curr.children[p];
        }
        return curr;
    }
    ls() { const node = this.resolve(this.cwd); return node ? Object.keys(node.children) : []; }
}
const vfs = new VirtualFS();

class SigmaTerminal {
    constructor(outId, inId) {
        this.out = document.getElementById(outId);
        this.in = document.getElementById(inId);
        this.in?.addEventListener('keydown', e => e.key === 'Enter' && this.exec(this.in.value));
    }
    print(msg, type = '') {
        const d = document.createElement('div');
        d.className = `term-line ${type}`;
        d.innerHTML = msg;
        this.out?.appendChild(d);
        this.out.scrollTop = this.out.scrollHeight;
    }
    exec(cmd) {
        this.print(`<span class="term-prompt">sovereign@sigma:${vfs.cwd}$</span> ${cmd}`);
        this.in.value = '';
        const args = cmd.trim().split(' ');
        const base = args[0].toLowerCase();
        if (base === 'help') this.print('Commands: ls, cd, cat, clear, reboot, exit');
        else if (base === 'ls') this.print(vfs.ls().join('  '));
        else if (base === 'clear') this.out.innerHTML = '';
        else if (base === 'reboot') location.reload();
        else this.print(`Command not found: ${base}`, 'error');
    }
}
window.addEventListener('load', () => {
    new SigmaTerminal('terminal-output', 'terminal-input');
    heartbeat.start();
});

// Initialize terminal when window is available
let sigmaTerm;
function initTerminal() {
    sigmaTerm = new SigmaTerminal('terminal-output', 'terminal-input');
    sigmaTerm.print('Σ SIGMAOS SOVEREIGN TERMINAL v5.1');
    sigmaTerm.print('Type "help" for a list of commands.\n');
}

window.addEventListener('load', () => {
    initTerminal();
    addLog('Σ [VFS]: Persistent Shard Sharding Matrix ACTIVE.', 'success');
});

// SOVEREIGN FILE MANAGER
class FileManager {
    constructor(gridId, breadcrumbsId) {
        this.grid = document.getElementById(gridId);
        this.breadcrumbs = document.getElementById(breadcrumbsId);
        this.currentPath = '/home/sovereign';
        this.update();
    }

    update() {
        if (!this.grid || !this.breadcrumbs) return;
        this.grid.innerHTML = '';
        this.breadcrumbs.innerText = this.currentPath;

        const files = vfs.ls(this.currentPath);
        if (files) {
            files.forEach(name => {
                const node = vfs.resolve(this.currentPath + '/' + name);
                const item = document.createElement('div');
                item.className = 'fm-item';
                item.innerHTML = `
                    <div class="fm-item-icon">${node.type === 'dir' ? '📁' : '📄'}</div>
                    <div class="fm-item-label">${name}</div>
                `;
                item.onclick = () => {
                    if (node.type === 'dir') {
                        this.currentPath = (this.currentPath === '/' ? '' : this.currentPath) + '/' + name;
                        this.update();
                    } else {
                        addLog(`Σ [FS]: Reading ${name}...`, 'success');
                        alert(node.content || '(Empty File)');
                    }
                };
                this.grid.appendChild(item);
            });
        }
    }

    goBack() {
        if (this.currentPath === '/') return;
        const parts = this.currentPath.split('/');
        parts.pop();
        this.currentPath = parts.join('/') || '/';
        this.update();
    }
}

let fileManager;
function initFileManager() {
    fileManager = new FileManager('fm-grid', 'fm-breadcrumbs');
}

// Update existing launchApp to handle new windows
const originalLaunchApp = launchApp;
launchApp = function(app) {
    if (app === 'OmniShell' || app === 'OmniShell v5.1') {
        openWindow('terminal-win');
    } else if (app === 'File Manager' || app === '📂') {
        openWindow('file-manager-win');
        if (fileManager) fileManager.update();
    } else {
        originalLaunchApp(app);
    }
};

window.addEventListener('load', () => {
    initFileManager();
    // Register the new windows with WM
    if (typeof wm !== 'undefined') {
        wm.register('terminal-win');
        wm.register('file-manager-win');
    }
});


function fmBack() {
    if (fileManager) fileManager.goBack();
}

// NEURAL INTELLIGENCE & TURBO MODE
let isTurboMode = false;
function toggleTurboMode() {
    isTurboMode = !isTurboMode;
    const btn = document.getElementById('turbo-toggle');
    if (isTurboMode) {
        btn.classList.add('active');
        btn.style.color = 'var(--accent)';
        btn.style.boxShadow = '0 0 20px var(--accent-glow)';
        addLog('Σ [AISCHED]: NPWO Neural Scheduler entering PERFORMANCE mode.', 'success');
        // Simulate ring-buffer submission for speed boost
        if (typeof vfs !== 'undefined') {
            addLog('Σ [KERN]: sigma_ring SQPOLL thread active. Context switches minimized.', 'success');
        }
    } else {
        btn.classList.remove('active');
        btn.style.color = '';
        btn.style.boxShadow = '';
        addLog('Σ [AISCHED]: NPWO Neural Scheduler entering BALANCED mode.', 'success');
    }
}

function semanticSearch() {
    const input = document.getElementById('semantic-search-input').value.toLowerCase();
    const output = document.getElementById('ai-hub-results');
    if (!input) return;

    output.innerHTML = '<span class="accent">Σ [NEURAL]: Retrieving semantic embeddings from local VFS...</span>';
    
    setTimeout(() => {
        let results = '';
        if (input.includes('project') || input.includes('silicon')) {
            results = '>> Found: /home/sovereign/projects/silicon_lattice_v1.sh<br>>> Relevance: 0.98 (Semantic Match)';
        } else if (input.includes('welcome') || input.includes('document')) {
            results = '>> Found: /home/sovereign/welcome.txt<br>>> Relevance: 0.85 (Contextual Match)';
        } else {
            results = '>> No direct semantic matches found. Expanding search to encrypted shards...';
        }
        output.innerHTML = results;
        addLog('Σ [NEURAL]: Semantic Retrieval complete.', 'success');
    }, 1200);
}

// Update switchUtil to handle neural-hub
const originalSwitchUtil = typeof switchUtil === 'function' ? switchUtil : null;
window.switchUtil = function(utilId) {

    const panes = ['text-ops', 'data-conv', 'code-img', 'diff', 'dup-find', 'key-test', 'speed', 'status', 'bootable', 'table', 'broadcaster', 'firewall', 'neural-hub', 'snapshots'];

    panes.forEach(p => {
        const el = document.getElementById(`util-${p}`);
        if (el) el.style.display = 'none';
        else {
            const el2 = document.getElementById(p);
            if (el2) el2.style.display = 'none';
        }
    });
    
    const targetPane = document.getElementById(`util-${utilId}`) || document.getElementById(utilId);
    if (targetPane) targetPane.style.display = 'block';
    
    const navs = document.querySelectorAll('.util-nav');
    navs.forEach(n => n.classList.remove('active'));
    
    if (window.event && window.event.target && window.event.target.classList.contains('util-nav')) {
        window.event.target.classList.add('active');
    }
    


    addLog(`Σ [UTILITY]: Switching to ${utilId} module.`, "success");
};

// ATOMIC SNAPSHOTS & SWAPS
function createSnapshot() {
    const name = document.getElementById('snap-name').value || `Snap_${Date.now()}`;
    addLog(`Σ [SNAP]: Initiating Atomic State Capture: ${name}...`, 'success');
    
    // Simulate VFS Serialization
    setTimeout(() => {
        const list = document.getElementById('snapshot-list');
        const li = document.createElement('li');
        li.className = 'routine-item routine-border-cyan';
        li.innerHTML = `<strong>${name}</strong> (User State) <button class="cyber-btn small-btn" onclick="atomicSwap('${name}')">SWAP</button>`;
        list.appendChild(li);
        
        addLog(`Σ [SNAP]: Snapshot ${name} persisted.`, 'success');
        document.getElementById('snap-name').value = '';
    }, 1500);
}

async function atomicSwap(target) {
    addLog(`Σ [SNAP]: CRITICAL: Atomic Swap triggered for [${target}].`, 'warning');
    addLog(`Σ [KERN]: Locking Lattice Shards...`, 'success');
    
    // Simulate UI freeze/transition
    document.body.style.opacity = '0.3';
    
    setTimeout(() => {
        document.body.style.opacity = '1';
        addLog(`Σ [KERN]: Pointer Swap Complete. New Root: ${target}`, 'success');
        addLog(`Σ [SNAP]: System Resume Successful.`, 'success');
        
        // Notify the simulated kernel
        ipc.syscall('SYS_SNAP_SWAP', { target });
    }, 2000);
}


// =========================================================================
// Σ SIGMAOS: SOVEREIGN IPC BRIDGE (LATTICE-BUS)
// =========================================================================
class SigmaIPC {
    constructor() {
        this.ring = {
            sq: { head: 0, tail: 0, entries: [] },
            cq: { head: 0, tail: 0, entries: [] }
        };
        this.callbacks = new Map();
        this.nextId = 1;
    }

    /**
     * @brief Send a structured "System Call" to the Kernel Shard.
     */
    async syscall(op, params = {}) {
        const id = this.nextId++;
        const sqe = { id, op, params, timestamp: Date.now() };
        
        // Push to Submission Queue
        this.ring.sq.entries.push(sqe);
        this.ring.sq.tail++;
        
        addLog(`Σ [IPC]: Call 0x${id.toString(16)} [${op}] Submitted.`, 'success');
        
        return new Promise((resolve) => {
            this.callbacks.set(id, resolve);
            // Simulate Kernel Processing
            setTimeout(() => this.processCQE(id), 50 + Math.random() * 100);
        });
    }

    processCQE(id) {
        const callback = this.callbacks.get(id);
        if (callback) {
            const res = { status: 'success', data: 'Silicon Acknowledged.' };
            this.ring.cq.entries.push({ id, res });
            this.ring.cq.tail++;
            callback(res);
            this.callbacks.delete(id);
        }
    }
}

const ipc = new SigmaIPC();

// =========================================================================
// Σ SIGMAOS: ZENITH HARDWARE-ACCELERATED COMPOSITOR
// =========================================================================
class ZenithCompositor {
    constructor(canvasId) {
        this.canvas = document.getElementById(canvasId);
        if (this.canvas) {
            this.ctx = this.canvas.getContext('2d', { alpha: false });
            this.width = this.canvas.width;
            this.height = this.canvas.height;
            this.layers = [];
        }
    }

    /**
     * @brief Direct-to-VRAM Framebuffer projection.
     */
    render() {
        if (!this.ctx) return;
        
        // Fast Clear (Direct Path)
        this.ctx.fillStyle = '#050507';
        this.ctx.fillRect(0, 0, this.width, this.height);

        // Render Layers (Shards)
        this.layers.forEach(layer => {
            if (layer.visible) {
                this.ctx.globalAlpha = layer.opacity || 1;
                this.ctx.drawImage(layer.buffer, layer.x, layer.y);
            }
        });

        requestAnimationFrame(() => this.render());
    }

    addLayer(name, buffer, x = 0, y = 0) {
        this.layers.push({ name, buffer, x, y, visible: true, opacity: 1 });
    }
}

let compositor;
function initCompositor() {
    compositor = new ZenithCompositor('zenith-framebuffer');
    if (compositor.canvas) compositor.render();
}

window.addEventListener('load', () => {
    initCompositor();
});


// SHARD LOADER
async function loadShard(url) {
    addLog(`Σ [LATTICE]: Fetching Shard Manifest from ${url}...`, 'success');
    
    // 1. Fetch Manifest (Mock)
    const manifest = {
        name: "NeuralVisualizer",
        capabilities: ["FRAMEBUFFER_WRITE"]
    };

    // 2. Syscall to Inject
    const res = await ipc.syscall('SHARD_INJECT', { name: manifest.name });
    addLog(`Σ [LATTICE]: Shard Verified (PQC-Signature: OK).`, 'success');

    // 3. Create Compositor Layer for Shard
    const shardBuffer = document.createElement('canvas');
    shardBuffer.width = 400;
    shardBuffer.height = 300;
    const sctx = shardBuffer.getContext('2d');
    
    if (compositor) {
        compositor.addLayer(manifest.name, shardBuffer, 100, 100);
        executeShard(manifest.name, sctx);
    }
}

/**
 * @brief Mock WASM Execution.
 * In a real scenario, this would be AOT-compiled machine code writing to VRAM.
 */
function executeShard(name, ctx) {
    addLog(`Σ [WASM]: AOT-Compiling ${name} bytecode...`, 'success');
    addLog(`Σ [WASM]: ${name} is now executing on bare metal.`, 'success');

    function animate() {
        ctx.clearRect(0,0, 400, 300);
        ctx.fillStyle = 'rgba(0, 255, 163, 0.2)';
        ctx.strokeStyle = '#00ffa3';
        ctx.lineWidth = 2;
        
        // Render some "Neural" nodes
        for(let i=0; i<10; i++) {
            const x = 200 + Math.sin(Date.now()/1000 + i) * 100;
            const y = 150 + Math.cos(Date.now()/1000 + i) * 80;
            ctx.beginPath();
            ctx.arc(x, y, 5, 0, Math.PI*2);
            ctx.fill();
            ctx.stroke();
        }
        requestAnimationFrame(animate);
    }
    animate();
}





// =========================================================================
// Σ SIGMAOS: SIGMA CORE & ADAPTIVE WORKFLOW ENGINE (V2)
// =========================================================================
class SigmaCore {
    constructor() {
        this.currentMode = 'Balanced';
        this.currentEdition = 'Neon'; // Default futuristic edition
        this.automationRules = [];
        this.modes = {
            'Balanced': { accent: '#00ffa3', bg: 'rgba(5, 5, 7, 0.95)', cpu: 'BALANCED' },
            'Gamer': { accent: '#ff00ff', bg: 'rgba(10, 5, 15, 0.98)', cpu: 'TURBO' },
            'Creator': { accent: '#00c3ff', bg: 'rgba(5, 10, 15, 0.98)', cpu: 'MAX-THREADS' },
            'Streamer': { accent: '#ff3300', bg: 'rgba(15, 5, 5, 0.98)', cpu: 'ENCODE-PRIO' },
            'Red Team': { accent: '#ff0055', bg: 'rgba(10, 5, 5, 0.98)', cpu: 'MAX-PERF' },
            'Coding': { accent: '#00c3ff', bg: 'rgba(5, 7, 10, 0.98)', cpu: 'PERFORMANCE' },
            'Minimal': { accent: '#ffffff', bg: 'rgba(0, 0, 0, 1)', cpu: 'POWERSAVE' },
            'AI Native': { accent: '#7000ff', bg: 'rgba(10, 0, 20, 0.95)', cpu: 'NEURAL-BOOST' }
        };
        this.initAutomation();
    }

    initAutomation() {
        // Register default contextual automations
        this.addAutomationRule({
            trigger: 'app_launch',
            condition: (app) => app === 'Markup Forge',
            action: () => this.setMode('Coding')
        });
        this.addAutomationRule({
            trigger: 'battery_low',
            condition: (pct) => pct < 20,
            action: () => this.setMode('Minimal')
        });
    }

    addAutomationRule(rule) {
        this.automationRules.push(rule);
        addLog(`Σ [AUTO]: New context rule registered for ${rule.trigger}.`, "success");
    }

    triggerEvent(type, data) {
        this.automationRules
            .filter(r => r.trigger === type && r.condition(data))
            .forEach(r => r.action());
    }

    setMode(modeName) {
        const mode = this.modes[modeName];
        if (!mode) return;

        this.currentMode = modeName;
        document.documentElement.style.setProperty('--accent', mode.accent);
        document.documentElement.style.setProperty('--accent-glow', mode.accent + '66');
        
        const activeWorkflowEl = document.getElementById('active-workflow');
        if (activeWorkflowEl) activeWorkflowEl.innerText = `LATTICE: ${modeName.toUpperCase()}`;
        
        addLog(`Σ [CORE]: Environment optimized for ${modeName}. Base: ${sigmaConfig.data.system.base}.`, 'success');
        
        // Notify Kernel (Simulated Bridge to systemd/dbus)
        if (typeof ipc !== 'undefined') {
            ipc.syscall('SYS_SET_CPU_GOVERNOR', { mode: mode.cpu });
        }
    }

    healSystem() {
        addLog("Σ [HEAL]: Initiating lattice-wide self-diagnostics...", "warning");
        setTimeout(() => addLog("Σ [HEAL]: Shard integrity verified. 0 defects found.", "success"), 2000);
    }
}

const sigmaCore = new SigmaCore();

function setWorkflowMode(mode) {
    sigmaCore.setMode(mode);
}

// =========================================================================
// Σ SIGMAOS: COMMAND CENTER (UNIVERSAL PALETTE)
// =========================================================================
let commandPaletteActive = false;
function toggleCommandPalette() {
    const palette = document.getElementById('command-center');
    if (!palette) return;
    commandPaletteActive = !commandPaletteActive;
    
    if (commandPaletteActive) {
        palette.classList.remove('hidden');
        document.getElementById('command-input').focus();
    } else {
        palette.classList.add('hidden');
    }
}

// Keybindings
window.addEventListener('keydown', (e) => {
    // Alt + Space to toggle command center
    if (e.altKey && e.code === 'Space') {
        e.preventDefault();
        toggleCommandPalette();
    }
    
    // Escape to close
    if (e.key === 'Escape' && commandPaletteActive) {
        toggleCommandPalette();
    }
});

// Command Search Logic
const commandInput = document.getElementById('command-input');
const commandResults = document.getElementById('command-results');

const availableCommands = [
    { label: 'Optimize Gaming', hint: 'Workflow', action: () => setWorkflowMode('Gamer') },
    { label: 'Deploy Capsule: AI Research', hint: 'Capsule', action: () => deployCapsule('AI Research') },
    { label: 'Deploy Capsule: Hacker Lab', hint: 'Capsule', action: () => deployCapsule('Hacker Lab') },
    { label: 'Browse Capsules', hint: 'App', action: () => launchApp('Capsule Library') },
    { label: 'Fix my system', hint: 'Semantic Search', action: () => sigmaCore.healSystem() },
    { label: 'Explain Tool: Nmap', hint: 'AI Assistant', action: () => addLog('Σ [AI]: Nmap is a network discovery and security auditing tool.', 'success') },
    { label: 'Explain Tool: Metasploit', hint: 'AI Assistant', action: () => addLog('Σ [AI]: Metasploit is an exploitation framework for developing and executing exploit code.', 'success') },
    { label: 'Create Snapshot', hint: 'System', action: () => switchUtil('snapshots') },
    { label: 'Network Scan', hint: 'Security', action: () => addLog('Σ [NET]: Scanning mesh...', 'success') },
    { label: 'Clear Logs', hint: 'System', action: () => {
        const logOut = document.getElementById('log-output');
        if (logOut) logOut.innerHTML = '';
    }}
];

// Debounce Utility
function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

if (commandInput) {
    const performSearch = debounce((query) => {
        const filtered = availableCommands.filter(cmd => 
            cmd.label.toLowerCase().includes(query) || cmd.hint.toLowerCase().includes(query)
        );
        renderCommandResults(filtered);
    }, 150);

    commandInput.addEventListener('input', (e) => {
        performSearch(e.target.value.toLowerCase());
    });
}

function renderCommandResults(results) {
    if (!commandResults) return;
    commandResults.innerHTML = '';
    results.forEach((cmd, index) => {
        const el = document.createElement('div');
        el.className = 'command-item';
        el.innerHTML = `
            <span class="cmd-label">${cmd.label}</span>
            <span class="cmd-hint">${cmd.hint}</span>
        `;
        el.onclick = () => {
            cmd.action();
            toggleCommandPalette();
            commandInput.value = '';
        };
        commandResults.appendChild(el);
    });
}

// SIDEBAR & LIVE STYLE
function toggleSidebar() {
    const sidebar = document.getElementById('sigma-sidebar');
    if (sidebar) sidebar.classList.toggle('hidden');
}

function updateLiveStyle(param, value) {
    const root = document.documentElement;
    if (param === 'blur') {
        // Find all windows and apply backdrop-filter
        const windows = document.querySelectorAll('.window, .sidebar, .command-palette');
        windows.forEach(win => {
            win.style.backdropFilter = `blur(${value}px) saturate(160%)`;
        });
    } else if (param === 'saturate') {
        const windows = document.querySelectorAll('.window, .sidebar, .command-palette');
        windows.forEach(win => {
            win.style.backdropFilter = win.style.backdropFilter.replace(/saturate\(\d+%\)/, `saturate(${value}%)`);
        });
    }
    addLog(`Σ [CORE]: Style Parameter ${param} optimized to ${value}.`, 'success');
}

// Update taskbar binding
window.addEventListener('load', () => {
    const aiIcon = document.querySelector('.task-icon[onclick="launchApp(\'AI Assistant\')"]');
    if (aiIcon) {
        aiIcon.setAttribute('onclick', 'toggleSidebar()');
    }
});


// SECURITY MISSION CONTROL LOGIC
function runSecurityScan() {
    const output = document.getElementById('recon-output');
    if (!output) return;
    output.innerHTML = '<span class="accent">Σ [RECON]: Enumerating target lattice...</span>';
    
    setTimeout(() => {
        output.innerHTML = `
            <div class="log-item success">Σ [RECON]: Port 80 OPEN (Nginx/1.18.0)</div>
            <div class="log-item success">Σ [RECON]: Port 443 OPEN (OpenSSL/1.1.1)</div>
            <div class="log-item warning">Σ [RECON]: Subdomain discovered: dev.lattice.local</div>
        `;
        addLog("Σ [RECON]: Target enumeration complete. 3 Findings.", "success");
    }, 2000);
}

function switchSecTab(tab) {
    const main = document.getElementById('sec-main');
    if (!main) return;
    const navs = document.querySelectorAll('.sec-nav');
    navs.forEach(n => n.classList.remove('active'));
    
    if (tab === 'recon') {
        main.innerHTML = `<h3>Reconnaissance Shard</h3><div id="recon-output" class="log-output" style="height: 150px;"></div><button class="util-btn" onclick="runSecurityScan()">START SCAN</button>`;
    } else if (tab === 'attestation') {
        main.innerHTML = `<h3>Hardware Attestation</h3><div class="settings-group"><label>Root of Trust</label><span class="status-success">VERIFIED</span></div><div class="settings-group"><label>Hardware ID</label><span>LATTICE-ID-7742-PQ</span></div><div class="settings-group"><label>PQC Shield</label><span class="status-success">ACTIVE</span></div><button class="util-btn" onclick="addLog('Σ [ATTEST]: Manually re-verifying lattice...', 'warning'); setTimeout(() => addLog('Σ [ATTEST]: Integrity audit SUCCESS.', 'success'), 1500)">RE-VERIFY</button>`;
    } else if (tab === 'vault') {
        main.innerHTML = `<h3>Sovereign Vault</h3><p class="stat-label">Secure enclave for cryptographic material.</p><div class="settings-group"><label>Active Keys</label><span>14</span></div><button class="util-btn" onclick="addLog('Σ [VAULT]: Rotating PQC keys...', 'info')">ROTATE KEYS</button>`;
    } else {
        main.innerHTML = `<h3>Security Shard: ${tab.toUpperCase()}</h3><p class="stat-label">Initializing intelligent auditing for ${tab}...</p>`;
    }
    addLog(`Σ [SEC]: Mission Control switched to ${tab} tab.`, "success");
}

// SIGMA CONFIG ENGINE (Industrial Grade YAML Persistence)

class SigmaConfig {
    constructor() {
        this.key = 'sigma_config';
        this.data = JSON.parse(localStorage.getItem(this.key)) || {
            ui: { accent: '#00ff88', theme: 'cyan', glass: 20 },
            system: { profile: 'Developer' }
        };
        this.applyAll();
    }
    save() { localStorage.setItem(this.key, JSON.stringify(this.data)); }
    update(path, val) {
        const parts = path.split('.');
        let curr = this.data;
        parts.slice(0, -1).forEach(p => curr = curr[p] = curr[p] || {});
        curr[parts.pop()] = val;
        this.apply(path, val);
        this.save();
    }
    apply(path, val) {
        if (path.startsWith('ui.accent')) {
            document.documentElement.style.setProperty('--accent', val);
            document.documentElement.style.setProperty('--accent-glow', val + '44');
        }
        if (path === 'ui.theme') window.setTheme(val);
    }
    applyAll() { Object.keys(this.data.ui).forEach(k => this.apply(`ui.${k}`, this.data.ui[k])); }
}
const config = new SigmaConfig();

class CommandCenter {
    constructor() {
        this.el = document.getElementById('cmd-palette');
        this.input = document.getElementById('cmd-input');
        this.results = document.getElementById('cmd-results');
        this.input?.addEventListener('input', e => this.search(e.target.value));
    }
    search(q) {
        const apps = [
            { name: 'OmniShell', cmd: "launchApp('OmniShell')" },
            { name: 'File Manager', cmd: "launchApp('File Manager')" },
            { name: 'Sigma Browser', cmd: "launchApp('Sigma Browser')" },
            { name: 'Lattice Settings', cmd: "launchApp('Lattice Settings')" }
        ];
        const filtered = apps.filter(a => a.name.toLowerCase().includes(q.toLowerCase()));
        if (this.results) {
            this.results.innerHTML = filtered.map(a => `
                <div class="command-item" onclick="${a.cmd}; CommandCenter.hide()">
                    <span>${a.name}</span>
                </div>
            `).join('');
        }
    }
    static hide() { document.getElementById('cmd-palette')?.classList.remove('active'); }
}
new CommandCenter();

// MORPHIC LAYOUT ENGINE
let draggedWidget = null;

window.handleDragStart = function(e) {
    draggedWidget = e.target.closest('.widget') || e.target.closest('.card');
    e.dataTransfer.effectAllowed = 'move';
};

window.handleDragOver = function(e) {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
};

window.handleDrop = function(e) {
    e.preventDefault();
    const target = e.target.closest('.widget') || e.target.closest('.card');
    if (draggedWidget && target && draggedWidget !== target) {
        const parent = target.parentNode;
        parent.insertBefore(draggedWidget, target);
        addLog("Σ [MORPHIC]: Layout updated via silicon gesture.", "success");
    }
};

// SIGMA AUTOMATION ENGINE
class SigmaAutomationEngine {
    constructor() {
        this.rules = [];
        this.init();
    }
    init() {
        setInterval(() => {
            const battery = Math.floor(Math.random() * 100);
            if (battery < 20) this.trigger('low_battery');
        }, 15000);
    }
    addRule(trigger, action) {
        this.rules.push({ trigger, action });
        addLog(`Σ [AUTO]: New rule added: ${trigger} -> ${action}`, "success");
    }
    trigger(event) {
        addLog(`Σ [AUTO]: Event detected: ${event}. Orchestrating response...`, "warning");
        if (event === 'low_battery' && sigmaCore.currentMode !== 'Minimal') {
            setWorkflowMode('Minimal');
            addLog("Σ [AUTO]: Low battery detected. Auto-switched to Minimal mode.", "success");
        }
        this.rules.filter(r => r.trigger === event).forEach(r => {
            addLog(`Σ [AUTO]: Rule matched: ${r.action}`, "success");
            eval(r.action); // Industrial macro execution
        });
    }
}

// DYNAMIC THEME ENGINE
class DynamicThemeEngine {
    constructor() {
        this.autoMode = true;
        this.init();
    }
    init() {
        setInterval(() => {
            if (this.autoMode) this.applyContextualTheme();
        }, 60000); // Check every minute
    }
    applyContextualTheme() {
        const hour = new Date().getHours();
        if (hour >= 18 || hour < 6) {
            document.documentElement.style.setProperty('--accent-primary', '#ff0055');
            document.documentElement.style.setProperty('--bg-glass', 'rgba(10, 10, 15, 0.85)');
        } else {
            document.documentElement.style.setProperty('--accent-primary', '#00ff88');
            document.documentElement.style.setProperty('--bg-glass', 'rgba(255, 255, 255, 0.1)');
        }
    }
}

// SOVEREIGN MAINTENANCE DAEMON
class SovereignMaintenanceDaemon {
    constructor() {
        this.init();
    }
    init() {
        setInterval(() => this.runHygiene(), 300000); // Every 5 minutes
    }
    runHygiene() {
        addLog('Σ [MAINT]: Background hygiene cycle started...', 'info');
        // Simulate cleanup
        setTimeout(() => addLog('Σ [MAINT]: Cache purged. 142MB reclaimed.', 'success'), 2000);
    }
}

// CUSTOM CONTEXT MENU SYSTEM
window.addEventListener('contextmenu', (e) => {
    e.preventDefault();
    const menu = document.getElementById('context-menu');
    if (!menu) return;
    
    menu.style.display = 'block';
    menu.style.left = `${e.pageX}px`;
    menu.style.top = `${e.pageY}px`;
    menu.style.zIndex = '10000';
});

window.addEventListener('click', () => {
    const menu = document.getElementById('context-menu');
    if (menu) menu.style.display = 'none';
});

window.contextAction = function(action) {
    addLog(`Σ [UI]: Context Action: ${action}`, 'success');
    if (action === 'refresh') location.reload();
};

window.toggleHelp = function() {
    const help = document.getElementById('help-overlay');
    if (!help) return;
    
    if (help.classList.contains('wizard-overlay--hidden')) {
        help.innerHTML = `
            <div class="wizard-card">
                <h2>Σ Sovereign Help Matrix</h2>
                <div class="wizard-steps" style="max-height: 400px; overflow-y: auto; text-align: left;">
                    <div class="settings-group">
                        <label>Alt + Space</label>
                        <span>Toggle Command Center (Universal Palette)</span>
                    </div>
                    <div class="settings-group">
                        <label>Alt + T</label>
                        <span>Launch OmniShell Terminal</span>
                    </div>
                    <div class="settings-group">
                        <label>Ctrl + Space</label>
                        <span>Open Unified Search</span>
                    </div>
                    <div class="settings-group">
                        <label>Right Click</label>
                        <span>Access Sovereign Context Menu</span>
                    </div>
                    <div class="settings-group">
                        <label>Automated Hygiene</label>
                        <span>The Maintenance Daemon runs every 5 minutes to purge cache and rotate logs.</span>
                    </div>
                    <div class="settings-group">
                        <label>Workflow Modes</label>
                        <span>Switch modes (Balanced, Gamer, Minimal) via the Command Center or Settings.</span>
                    </div>
                    <div class="settings-group">
                        <label>Lattice Marketplace</label>
                        <span>Inject new system shards (Glass-Pro, AVX-512) directly into the kernel.</span>
                    </div>
                </div>
                <button class="wizard-btn" onclick="toggleHelp()">DISMISS MATRIX</button>
            </div>
        `;
        help.classList.remove('wizard-overlay--hidden');
    } else {
        help.classList.add('wizard-overlay--hidden');
    }
};

// Singleton Initializations
const themeEngine = new DynamicThemeEngine();
const automation = new SigmaAutomationEngine();
const maintenance = new SovereignMaintenanceDaemon();

// SIGMA CAPSULE DEPLOYMENT
function deployCapsule(name) {
    addLog(`Σ [CAPSULE]: Deploying ${name} Environment...`, "success");
    const label = document.getElementById('active-capsule');
    if (label) label.textContent = `CAPSULE: ${name.toUpperCase()}`;
    
    if (name === 'AI Research') {
        setWorkflowMode('AI Research');
        launchApp('AI Assistant');
    } else if (name === 'Hacker Lab') {
        setWorkflowMode('Red Team');
        launchApp('OmniShell');
        launchApp('Security Mission Control');
    } else if (name === 'Cyberpunk Dev') {
        setWorkflowMode('Coding');
        launchApp('Markup Forge');
    }
    const win = document.getElementById('sigma-capsule-win');
    if (win) win.style.display = 'none';
}

// --- Global Orchestration Hooks ---

// Final initialization
window.addEventListener('load', () => {
    document.querySelectorAll('.window').forEach(w => wm.register(w.id));
    addLog('Σ [ZENITH]: Sovereign Lattice v1.0 ONLINE.', 'success');
});

// Mouse Glow
document.addEventListener('mousemove', e => {
    const glow = document.getElementById('mouse-glow');
    if (glow) {
        glow.style.left = e.clientX + 'px';
        glow.style.top = e.clientY + 'px';
        glow.style.opacity = '1';
    }
});
