
class ErrorHandler {
    static handle(error, context = '') {
        console.error(`[ERROR] ${context}:`, error);
        if (typeof addLog === 'function') {
            addLog(`Σ [ERR]: ${context} - ${error.message}`, 'error');
        }
    }

    static async safely(fn, context = 'Unknown') {
        try {
            return await fn();
        } catch (error) {
            this.handle(error, context);
            return null;
        }
    }
}

window.addEventListener('error', (event) => {
    ErrorHandler.handle(event.error, 'Uncaught exception');
});

window.addEventListener('unhandledrejection', (event) => {
    ErrorHandler.handle(event.reason, 'Unhandled Promise rejection');
});


const SecurityUtils = {
    sanitizeHTML(html) {
        const div = document.createElement('div');
        div.textContent = html;
        return div.innerHTML;
    }
};

const InputValidator = {
    isValidURL(str) {
        try { new URL(str.startsWith('http') ? str : 'http://' + str); return true; } catch { return false; }
    },
    sanitizeInput(str, maxLength = 1000) {
        if (typeof str !== 'string') return '';
        return str.slice(0, maxLength).trim();
    }
};
/** SigmaOS Zenith Desktop — https://github.com/AaryanSinghChauhan09/SigmaOS */
'use strict';

const SIGMA_APP_VERSION = '100.0';
const SIGMA_REPO_URL = 'https://github.com/AaryanSinghChauhan09/SigmaOS';

/* Σ Neural UI Layout Engine (NeuralWM) */
        class NeuralLayoutEngine {
            constructor() {
                this.interactions = {};
                this.reorgTimeout = null;
                this.domCache = new Map();
            }
            track(shardId) {
                this.interactions[shardId] = (this.interactions[shardId] || 0) + 1;
                this.scheduleReorganize();
            }
            scheduleReorganize() {
                if (this.reorgTimeout) clearTimeout(this.reorgTimeout);
                this.reorgTimeout = setTimeout(() => this.reorganize(), 300);
            }
            getEl(id) {
                if (!this.domCache.has(id)) {
                    this.domCache.set(id, document.getElementById(id));
                }
                return this.domCache.get(id);
            }
            reorganize() {
                // Determine top interaction directly without sorting the entire array if we only care about the top 1
                let maxCount = -1;
                let topId = null;
                for (const [id, count] of Object.entries(this.interactions)) {
                    if (count > maxCount) {
                        maxCount = count;
                        topId = id;
                    }
                }
                for (const id of Object.keys(this.interactions)) {
                    const el = this.getEl(id);
                    if (!el) continue;
                    if (id === topId) el.classList.add('neural-active');
                    else el.classList.remove('neural-active');
                }
            }
            setMindfulness(active) {
                document.body.classList.toggle('focus-mode-active', active);
                if (typeof addLog === 'function') addLog(active ? 'Σ [NEURAL]: Mindfulness Mode ACTIVE. Filtering noise...' : 'Σ [NEURAL]: Mindfulness Mode DISABLED.');
            }
        }
        const neural = new NeuralLayoutEngine();

        /* Σ Personalization & Theme Orchestration */
        class PersonalizationEngine {
            constructor() {
                this.registry = JSON.parse(localStorage.getItem('sigma_registry') || '{}');
                this.apply(true);
            }
            setTheme(name) {
                this.registry.theme = name;
                this.save();
                this.apply();
            }
            save() {
                localStorage.setItem('sigma_registry', JSON.stringify(this.registry));
            }
            apply(silent) {
                const theme = this.registry.theme || 'cyan';
                document.body.setAttribute('data-theme', theme);
                if (!silent && typeof addLog === 'function') {
                    addLog(`Σ [PERSONAL]: Shifting lattice hue to ${theme}...`, 'success');
                }
            }
        }
        const persona = new PersonalizationEngine();

        function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

function minimizeWindow(id) {
            const win = document.getElementById(id);
            if (win) win.style.display = 'none';
        }
        function closeWindow(id) {
            const win = document.getElementById(id);
            if (!win) return;
            win.classList.remove('shattering');
            win.style.display = 'none';
            if (typeof addLog === 'function') {
                addLog(`Σ [WM]: Shard ${id} minimized to lattice stack.`, 'success');
            }
        }

        /* Σ Zenith Window Manager (ZenithWM) Logic */
        class ZenithWM {
            constructor() {
                this.windows = [];
                this.topZ = 1000;
                this.desktops = [[], [], [], []]; // 4 Virtual Desktops
                this.activeDesktop = 0;
            }
            register(id) {
                const win = document.getElementById(id);
                if (!win) return;
                this.windows.push(win);
                this.desktops[this.activeDesktop].push(win);
                win.addEventListener('mousedown', () => this.bringToFront(win));
            }
            bringToFront(win) {
                this.topZ = this.topZ >= 9999 ? 1000 : this.topZ + 1;
                win.style.zIndex = String(this.topZ);
            }
            switchToDesktop(index) {
                if (index < 0 || index >= this.desktops.length) return;
                this.activeDesktop = index;
                this.windows.forEach(win => {
                    if (this.desktops[index].includes(win)) {
                        win.style.display = win.dataset.prevDisplay || 'block';
                    } else {
                        win.dataset.prevDisplay = win.style.display;
                        win.style.display = 'none';
                    }
                });
                addLog(`Σ [WM]: Switched to Virtual Desktop ${index + 1}.`, 'success');
                updateDesktopUI(index);
            }
            tileAll() {
                const visibleWindows = this.windows.filter(w => w.style.display !== 'none');
                const width = window.innerWidth / visibleWindows.length;
                visibleWindows.forEach((win, i) => {
                    win.style.width = `${width - 20}px`;
                    win.style.left = `${i * width + 10}px`;
                    win.style.top = '100px';
                });
            }
        }
        const wm = new ZenithWM();

        function updateDesktopUI(index) {
            document.querySelectorAll('.desktop-indicator').forEach((el, i) => {
                el.classList.toggle('active', i === index);
            });
        }


        const installedShards = new Set();
        let notifCount = 0;
        let ctxMenuEl;
        let notifPanelEl;
        let batteryPanelEl;
        let smoothCpu = 12;
        let smoothMem = 4.2;

        function pushNotification(msg) {
            notifCount++;
            const badge = document.getElementById('notif-badge');
            if (badge) {
                badge.textContent = notifCount > 9 ? '9+' : String(notifCount);
                badge.style.display = 'inline';
            }
            const list = document.getElementById('notif-list');
            if (list) {
                const row = document.createElement('div');
                row.className = 'notif-item';
                row.textContent = new Date().toLocaleTimeString() + ' — ' + msg;
                list.prepend(row);
            }
        }

        function toggleNotifications(open) {
            if (!notifPanelEl) notifPanelEl = document.getElementById('notif-panel');
            if (!notifPanelEl) return;
            const want = open === true || open === false ? open : !notifPanelEl.classList.contains('active');
            notifPanelEl.classList.toggle('active', want);
        }

        function clearAllNotifications() {
            notifCount = 0;
            const badge = document.getElementById('notif-badge');
            if (badge) { badge.textContent = '0'; badge.style.display = 'none'; }
            const list = document.getElementById('notif-list');
            if (list) list.innerHTML = '';
        }

        function toggleBatteryPanel(open) {
            if (!batteryPanelEl) batteryPanelEl = document.getElementById('battery-panel');
            if (!batteryPanelEl) return;
            const want = open === true || open === false ? open : !batteryPanelEl.classList.contains('active');
            batteryPanelEl.classList.toggle('active', want);
        }

        function setPowerMode(mode) {
            const label = document.getElementById('power-mode-label');
            if (label) label.textContent = mode;
            addLog(`Σ [POWER]: Profile set to ${mode}.`, 'success');
            toggleBatteryPanel(false);
        }

        function hideContextMenu() {
            if (!ctxMenuEl) ctxMenuEl = document.getElementById('context-menu');
            if (ctxMenuEl) ctxMenuEl.classList.remove('active');
        }

        function showContextMenu(clientX, clientY) {
            if (!ctxMenuEl) ctxMenuEl = document.getElementById('context-menu');
            if (!ctxMenuEl) return;
            ctxMenuEl.classList.add('active');
            const pad = 8;
            let x = clientX;
            let y = clientY;
            const r = ctxMenuEl.getBoundingClientRect();
            const mw = r.width || 200;
            const mh = r.height || 160;
            if (x + mw > window.innerWidth - pad) x = window.innerWidth - mw - pad;
            if (y + mh > window.innerHeight - pad) y = window.innerHeight - mh - pad;
            ctxMenuEl.style.left = x + 'px';
            ctxMenuEl.style.top = y + 'px';
        }

        function ctxAction(act) {
            hideContextMenu();
            if (act === 'reload') location.reload();
            else if (act === 'palette') {
                document.getElementById('cmd-palette').classList.add('active');
                document.getElementById('cmd-input').focus();
            } else if (act === 'settings') launchApp('Lattice Settings');
            else if (act === 'repo') window.open(SIGMA_REPO_URL, '_blank', 'noopener');
            else if (act === 'widgets') neural.setMindfulness(!document.body.classList.contains('focus-mode-active'));
            else addLog(`Σ [CTX]: ${act} queued on sovereign lattice.`, 'success');
        }

        // Update Clock
        function updateClock() {
            const now = new Date();
            const time = now.toLocaleTimeString('en-US', { hour12: false, hour: '2-digit', minute: '2-digit' });
            const date = now.toLocaleDateString('en-US', { weekday: 'long', month: 'long', day: 'numeric' });
            document.getElementById('clock-time').textContent = time;
            document.getElementById('clock-date').textContent = date;
        }
        setInterval(updateClock, 1000);
        updateClock();

        // Command Palette Logic
        /* Σ Hotkey Orchestrator */
        class HotkeyManager {
            constructor() {
                this.hotkeys = new Map();
                window.addEventListener('keydown', (e) => this.handle(e));
            }
            register(combo, callback) {
                this.hotkeys.set(combo.toLowerCase(), callback);
            }
            handle(e) {
                const parts = [];
                if (e.ctrlKey) parts.push('ctrl');
                if (e.altKey) parts.push('alt');
                if (e.shiftKey) parts.push('shift');
                if (e.metaKey) parts.push('meta');
                parts.push(e.key.toLowerCase());
                
                const combo = parts.join('+');
                if (this.hotkeys.has(combo)) {
                    e.preventDefault();
                    this.hotkeys.get(combo)(e);
                }

                // Legacy palette handling
                const tag = (e.target && e.target.tagName) || '';
                const isOtherField = ['INPUT', 'TEXTAREA', 'SELECT'].includes(tag) && e.target && e.target.id !== 'cmd-input';
                const paletteHotkey = (e.metaKey || e.ctrlKey) && (e.key === 'k' || e.code === 'Space');
                if (paletteHotkey && !isOtherField) {
                    e.preventDefault();
                    cmdPalette.classList.toggle('active');
                    if (cmdPalette.classList.contains('active') && cmdInput) cmdInput.focus();
                }
                if (e.key === 'Escape') {
                    cmdPalette.classList.remove('active');
                    hideContextMenu();
                    toggleNotifications(false);
                    toggleBatteryPanel(false);
                }
            }
        }
        const hotkeys = new HotkeyManager();

        // Register default hotkeys
        hotkeys.register('alt+1', () => wm.switchToDesktop(0));
        hotkeys.register('alt+2', () => wm.switchToDesktop(1));
        hotkeys.register('alt+3', () => wm.switchToDesktop(2));
        hotkeys.register('alt+4', () => wm.switchToDesktop(3));
        hotkeys.register('ctrl+alt+t', () => launchApp('Markup Forge'));
        hotkeys.register('ctrl+shift+l', () => addLog('Σ [USER]: User triggered manual audit.', 'success'));


        if (cmdInput) {
            
function createDebounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => { clearTimeout(timeout); func(...args); };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}
        const filterCommands = createDebounce((q) => {
            document.querySelectorAll('#cmd-results .command-item').forEach((el) => {
                const t = el.textContent.toLowerCase();
                el.style.display = !q || t.includes(q) ? '' : 'none';
            });
        }, 200);

        const cmdInput = document.getElementById('cmd-input');
        if (cmdInput) {
            cmdInput.addEventListener('input', (e) => {
                const q = e.target.value.trim().toLowerCase();
                filterCommands(q);
            });
        }
    }

        function setTheme(name) {
            ['theme-gold', 'theme-crimson', 'theme-solar'].forEach((c) => document.body.classList.remove(c));
            if (name && name !== 'cyan') document.body.classList.add('theme-' + name);
            persona.registry.theme = name;
            persona.save();
            persona.apply(true);
            document.body.setAttribute('data-theme', name || 'cyan');
            const status = (name || 'cyan').toUpperCase();
            addLog(`Σ [CONFIG]: Shard Theme switched to ${status}.`, 'success');
            const personaLabel = document.getElementById('persona-theme');
            if (personaLabel) personaLabel.textContent = `Theme: ${status}`;
            cmdPalette.classList.remove('active');
        }

        function openWindow(id) {
            const win = document.getElementById(id);
            if (!win) return;
            win.style.display = 'block';
            wm.bringToFront(win);
        }

        function toggleUtilityNexus() {
            openWindow('utility-nexus-win');
            addLog('Σ [AUTO]: Morphic Automation routed to Utility Nexus.', 'success');
        }

        // SVG Graph Logic
        const cpuGraph = document.getElementById('cpu-graph');
        const points = [40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40];

        function updateGraph(val) {
            points.shift();
            points.push(40 - (val / 100 * 40));
            const path = points.map((p, i) => `${i * 30},${p}`).join(' ');
            cpuGraph.setAttribute('points', path);
        }

        // Simulate Kernel Logs
        const logContainer = document.getElementById('kernel-logs');
        const logs = [
            { t: "Σ [BOOT]: Sovereign Lattice Online.", c: "" },
            { t: "Σ [SECURE]: PQC Shard Initialized (Lattice-V5).", c: "success" },
            { t: "Σ [AUTO]: Neural Personalization active.", c: "" },
            { t: "Σ [NET]: Cloud Maestro projected to EU-WEST-1.", c: "" },
            { t: "Σ [KERN]: Memory Slab Shard mapped (64MB).", c: "success" },
            { t: "Σ [AUDIT]: CPU Silicon Discovery: 16 Zenith Cores.", c: "" },
            { t: "Σ [VGA]: Direct Framebuffer Projector Online.", c: "success" },
            { t: "Σ [LATTICE]: Industrial 600-Shard Modularization COMPLETE.", c: "success" },
            { t: "Σ [ZENITH]: Command Palette (Ctrl+K / Ctrl+Space) ready.", c: "success" }
        ];

        function addLog(text, type) {
            const div = document.createElement('div');
            div.className = `log-item ${type}`;
            div.textContent = text;
            logContainer.prepend(div);
            if (logContainer.children.length > 15) logContainer.lastChild.remove();
        }

        let logIdx = 0;
        function cycleLogs() {
            if (logIdx < logs.length) {
                addLog(logs[logIdx].t, logs[logIdx].c);
                logIdx++;
                setTimeout(cycleLogs, 800);
            }
        }
        cycleLogs();

        // Simulate Hardware Stats
        // UI Interactions
        function toggleStart() {
            const menu = document.getElementById('start-menu');
            menu.classList.toggle('active');
            if (menu.classList.contains('active')) {
                addLog("Σ [ZENITH]: Start Menu Shard ACTIVE.", "success");
            }
        }

        function launchApp(app) {
            addLog(`Σ [ZENITH]: Launching ${app} Shard...`, "success");
            const map = {
                'Markup Forge': 'markup-forge-win',
                'Utility Nexus': 'utility-nexus-win',
                'Marketplace': 'marketplace-win',
                'Customization Panel': 'customization-win',
                'Lattice Settings': 'lattice-settings-win',
                'File Manager': 'file-manager-win',
                'Sigma Browser': 'browser-win',
                'AI Assistant': 'ai-assistant-win',
            };
            const id = map[app];
            if (id) {
                openWindow(id);
            } else {
                addLog(`Σ [ZENITH]: Shard "${app}" is not mapped on this lattice.`, "error");
            }
            const menu = document.getElementById('start-menu');
            if (menu && menu.classList.contains('active')) toggleStart();
        }

        // Marketplace Logic
        function installShard(name) {
            try {
                if (installedShards.has(name)) {
                    addLog(`Σ [PKG]: ${name} is already injected.`, "error");
                    return;
                }
                const item = document.querySelector(`[data-mkt-shard="${name}"]`);
                const btn = item && item.querySelector('button');
                if (!btn) {
                    throw new Error(`Invalid shard UI for ${name}`);
                }
                btn.textContent = 'INJECTING...';
                btn.disabled = true;
                addLog(`Σ [PKG]: Fetching ${name} bundle...`, "success");
                setTimeout(() => {
                    try {
                        installedShards.add(name);
                        if (item) item.classList.add('mkt-item--installed');
                        btn.textContent = '✓ INSTALLED';
                        btn.disabled = true;
                        addLog(`Σ [PKG]: ${name} injected successfully.`, "success");
                        pushNotification(`Pkg: ${name} installed`);
                        if (name === 'Glass-Pro') {
                            document.body.style.backdropFilter = 'blur(40px)';
                            addLog("Σ [CONFIG]: AVX-512 Shard Preempted for Glass-Pro.", "success");
                        }
                    } catch (e) {
                        ErrorHandler.handle(e, 'Shard installation completion');
                        btn.textContent = 'FAILED';
                    }
                }, 1000);
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
        let peers = 0;
        const meshView = document.getElementById('mesh-view');
        const meshSvg = document.getElementById('mesh-svg');
        const meshNodes = [];
        const MAX_LINES = 50; 
        const lines = [];

        function discoverPeer() {
            if (peers >= 12) return;
            peers++;
            const pc = document.getElementById('peer-count');
            if(pc) pc.innerText = peers;
            const node = document.createElement('div');
            node.className = 'mesh-node mesh-node-pulse';
            const x = Math.random() * 90;
            const y = Math.random() * 90;
            node.style.left = x + '%';
            node.style.top = y + '%';
            if (meshView) meshView.appendChild(node);
            meshNodes.push({x, y});

            if (peers > 1 && meshSvg) {
                const targetIdx = Math.floor(Math.random() * (peers - 1));
                const target = meshNodes[targetIdx];
                const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
                line.setAttribute('x1', x + '%'); line.setAttribute('y1', y + '%');
                line.setAttribute('x2', target.x + '%'); line.setAttribute('y2', target.y + '%');
                line.setAttribute('stroke', 'var(--accent)');
                line.setAttribute('stroke-width', '1');
                line.setAttribute('stroke-dasharray', '5,5');
                line.style.opacity = '0.5';
                line.style.animation = 'dash 2s linear infinite';
                meshSvg.appendChild(line);
                lines.push(line);
                if (lines.length > MAX_LINES) {
                    const oldLine = lines.shift();
                    if (oldLine.parentNode) oldLine.parentNode.removeChild(oldLine);
                }
            }
        }
        heartbeat.addTask('mesh-discovery', discoverPeer, 5000);

        // Transpiler & ISA Telemetry
        let transpiledDrivers = 0;
        heartbeat.addTask('transpiler-telemetry', () => {
            if(transpiledDrivers < 24) {
                transpiledDrivers += Math.floor(Math.random() * 3);
                const el = document.getElementById('transpiled-count');
                if (el) el.innerText = transpiledDrivers;
            }
            const isaEl = document.getElementById('silicon-isa');
            if(isaEl && Math.random() > 0.95) {
                isaEl.innerText = Math.random() > 0.5 ? "ARM (AArch64)" : "RISC-V (RV64GC)";
            }
        }, 3000);

        // Neural Automator
        const cognitiveTasks = ["Preemptive VRAM Caching", "Predictive Shard Loading", "Background Lattice Audit", "DNA Re-indexing", "Quantum Key Rotation"];
        let activeTasks = 0;
        heartbeat.addTask('neural-automator', () => {
            if (Math.random() > 0.6) {
                const task = cognitiveTasks[Math.floor(Math.random() * cognitiveTasks.length)];
                const queueEl = document.getElementById('cognitive-queue');
                const taskEl = document.createElement('div');
                taskEl.innerText = ">> " + task;
                taskEl.style.opacity = '0';
                taskEl.style.transition = 'opacity 0.5s';
                if (queueEl) queueEl.prepend(taskEl);
                setTimeout(() => taskEl.style.opacity = '1', 50);
                activeTasks++;
                const countEl = document.getElementById('auto-task-count');
                const currentEl = document.getElementById('auto-current-task');
                if (countEl) countEl.innerText = activeTasks + " Tasks Active";
                if (currentEl) currentEl.innerText = "Running: " + task;
                const autoProgress = document.getElementById('auto-progress');
                if (autoProgress) {
                    autoProgress.style.transition = 'width 2s linear';
                    autoProgress.style.width = '100%';
                }
                setTimeout(() => {
                    activeTasks--;
                    if (countEl) countEl.innerText = activeTasks + " Tasks Active";
                    if (taskEl.parentNode) taskEl.parentNode.removeChild(taskEl);
                    if (activeTasks === 0 && currentEl) currentEl.innerText = "Next: Idle";
                    if (autoProgress) { autoProgress.style.transition = 'none'; autoProgress.style.width = '0%'; }
                }, 2000);
            }
        }, 3500);

        // Orb Exchange
        const orbList = ["NeuralVisualizer-v2", "QuantumSieve-PQC", "BioFS-DNA-Module", "CryptoLedger-X"];
        heartbeat.addTask('orb-exchange', () => {
            if (Math.random() > 0.8) {
                const orb = orbList[Math.floor(Math.random() * orbList.length)];
                const statusEl = document.getElementById('orb-active');
                const progressEl = document.getElementById('orb-progress');
                if (statusEl) statusEl.innerText = "Summoning: " + orb;
                if (progressEl) {
                    progressEl.style.transition = 'width 1.5s linear';
                    progressEl.style.width = '100%';
                }
                setTimeout(() => {
                    if (statusEl) statusEl.innerText = "Idle";
                    if (progressEl) { progressEl.style.transition = 'none'; progressEl.style.width = '0%'; }
                }, 1600);
            }
        }, 4000);

        // Community Governance
        heartbeat.addTask('governance-telemetry', () => {
            if(Math.random() > 0.7) {
                const activeProposals = Math.floor(Math.random() * 12) + 1;
                const propEl = document.getElementById('gov-active');
                const voteEl = document.getElementById('gov-vote');
                const govProgress = document.getElementById('gov-progress');
                if (propEl) propEl.innerText = activeProposals + " Active Proposals";
                if (voteEl && govProgress) {
                    if(Math.random() > 0.5) {
                        voteEl.innerText = "YEA (QKD-VERIFIED)";
                        voteEl.className = "status-success";
                        govProgress.style.width = (50 + Math.random() * 40) + '%';
                        govProgress.style.background = 'var(--success)';
                    } else {
                        voteEl.innerText = "NAY (QKD-VERIFIED)";
                        voteEl.className = "accent";
                        govProgress.style.width = (10 + Math.random() * 40) + '%';
                        govProgress.style.background = 'var(--error)';
                    }
                }
            }
        }, 5000);

        // Trust Fabric Lockdown
        heartbeat.addTask('trust-fabric-audit', () => {
            if(Math.random() > 0.98) {
                document.body.classList.add('lockdown-mode');
                const qkdStatus = document.getElementById('qkd-status');
                const orbTrust = document.getElementById('orb-trust');
                if (qkdStatus) { qkdStatus.innerText = "ANOMALY DETECTED"; qkdStatus.style.color = "var(--error)"; }
                if (orbTrust) { orbTrust.innerText = "LOCKDOWN ACTIVE"; orbTrust.className = "accent"; }
                setTimeout(() => {
                    document.body.classList.remove('lockdown-mode');
                    if (qkdStatus) { qkdStatus.innerText = "Entangled"; qkdStatus.style.color = ""; }
                    if (orbTrust) { orbTrust.innerText = "QKD-VERIFIED"; orbTrust.className = "status-success"; }
                }, 5000);
            }
        }, 10000);

        heartbeat.start();

        const SigmaBrowser = {
            stack: ['sigma://lattice'],
            ptr: 0,
            pages: {
                'sigma://lattice': '<!DOCTYPE html><html><body style="background:#0a0a0f;color:#7dd3fc;font-family:system-ui;padding:2rem"><h1>σ Lattice Home</h1><p>Simulated sigma:// page.</p><a href="https://github.com/AaryanSinghChauhan09/SigmaOS" style="color:#fbbf24">GitHub — SigmaOS</a></body></html>',
                'sigma://kernel': '<!DOCTYPE html><html><body style="background:#0f0a14;color:#fbcfe8;font-family:system-ui;padding:2rem"><h1>Kernel ring</h1><p>Shard orchestration map.</p></body></html>',
                'sigma://mesh': '<!DOCTYPE html><html><body style="background:#0a140f;color:#86efac;font-family:system-ui;padding:2rem"><h1>P2P Mesh</h1><p>Peers synced.</p></body></html>',
            },
        };

        function sigmaBrowseGo() {
            const input = document.getElementById('browser-url');
            const frame = document.getElementById('browser-frame');
            if (!input || !frame) return;
            let u = (input.value || '').trim();
            if (!u.startsWith('sigma://')) u = 'sigma://lattice';
            const html = SigmaBrowser.pages[u] || SigmaBrowser.pages['sigma://lattice'];
            SigmaBrowser.stack = SigmaBrowser.stack.slice(0, SigmaBrowser.ptr + 1);
            SigmaBrowser.stack.push(u);
            SigmaBrowser.ptr = SigmaBrowser.stack.length - 1;
            frame.srcdoc = html;
            addLog(`Σ [WEB]: Navigated to ${u}`, 'success');
        }

        function sigmaBrowseBack() {
            const frame = document.getElementById('browser-frame');
            const input = document.getElementById('browser-url');
            if (SigmaBrowser.ptr > 0) SigmaBrowser.ptr--;
            const u = SigmaBrowser.stack[SigmaBrowser.ptr];
            if (input) input.value = u;
            if (frame) frame.srcdoc = SigmaBrowser.pages[u] || SigmaBrowser.pages['sigma://lattice'];
        }

        function sigmaBrowseForward() {
            const frame = document.getElementById('browser-frame');
            const input = document.getElementById('browser-url');
            if (SigmaBrowser.ptr < SigmaBrowser.stack.length - 1) SigmaBrowser.ptr++;
            const u = SigmaBrowser.stack[SigmaBrowser.ptr];
            if (input) input.value = u;
            if (frame) frame.srcdoc = SigmaBrowser.pages[u] || SigmaBrowser.pages['sigma://lattice'];
        }

        const AI_LINES = [
            'Σ Lattice acknowledges your intent.',
            'Routing cognitive shards… deterministic envelope sealed.',
            'No outbound bridge configured — reply simulated locally.',
            'Entropy within nominal bounds. Proceed.',
            'Recommend verifying shard hashes against GitHub main.',
        ];

        function sendAiChat() {
            const inp = document.getElementById('ai-chat-input');
            const log = document.getElementById('ai-chat-log');
            if (!inp || !log) return;
            const q = inp.value.trim();
            if (!q) return;
            const userLine = document.createElement('div');
            userLine.className = 'ai-chat-line ai-chat-line--user';
            userLine.textContent = q;
            log.appendChild(userLine);
            inp.value = '';
            const botLine = document.createElement('div');
            botLine.className = 'ai-chat-line ai-chat-line--bot';
            botLine.textContent = AI_LINES[Math.floor(Math.random() * AI_LINES.length)];
            log.appendChild(botLine);
            log.scrollTop = log.scrollHeight;
        }

        let batCharge = 87;
        setInterval(() => {
            batCharge = Math.max(18, Math.min(100, batCharge + (Math.random() - 0.52)));
            const fill = document.getElementById('battery-fill');
            const pct = document.getElementById('battery-pct');
            if (fill) fill.style.width = Math.round(batCharge) + '%';
            if (pct) pct.textContent = Math.round(batCharge) + '%';
        }, 4000);

        (function initZenithInteractions() {
            const desk = document.getElementById('zenith-desktop');
            if (desk) {
                desk.addEventListener('contextmenu', (e) => {
                    e.preventDefault();
                    showContextMenu(e.clientX, e.clientY);
                });
            }
            document.addEventListener('click', (e) => {
                if (e.target.closest('#context-menu')) return;
                hideContextMenu();
            });
        })();
const glow = document.getElementById('mouse-glow');
document.addEventListener('mousemove', (e) => {
    if(glow) {
        glow.style.left = e.clientX + 'px';
        glow.style.top = e.clientY + 'px';
        glow.style.opacity = '1';
    }
});
document.addEventListener('mouseleave', () => { if(glow) glow.style.opacity = '0'; });


function processData() {
    const input = document.getElementById('data-forge-input')?.value;
    const output = document.getElementById('data-forge-output');
    if(!input || !output) return;
    
    output.innerHTML = '<span class=\"text-accent\">[SDP] Initializing 600-shard parallel map/reduce...</span><br>';
    setTimeout(() => {
        output.innerHTML += '<span class=\"text-blue\">[SDP] Data Sharding complete. Partitioning into 4096 blocks...</span><br>';
        setTimeout(() => {
            output.innerHTML += '<span class=\"text-success\">[SDP] Forge Complete. Latency Absolute.</span><br>';
            output.innerHTML += '<pre style=\"color: #fff; margin-top: 10px;\">' + btoa(input).substring(0, 100) + '... [ENCODED]</pre>';
        }, 800);
    }, 500);
}

function executeCommand(cmd) {
    const args = cmd.split(' ');
    const command = args[0].toLowerCase();
    
    if (command === 'launch' || command === 'open') {
        const app = args.slice(1).join(' ');
        launchApp(app);
        return `[ZENITH] Launching ${app}...`;
    } else if (command === 'theme') {
        const themeName = args[1];
        setTheme(themeName);
        return `[ZENITH] Theme updated to ${themeName}.`;
    } else if (command === 'diag') {
        return `[DIAG] Silicon Health: 100%. All shards operational.`;
    }
    return `[ZENITH] Unknown command: ${command}`;
}

function evaluateLatticeRun(query) {
    if (query.startsWith('=')) {
        const expr = query.substring(1).trim();
        if (/^[0-9\+\-\*\/\(\)\. ]+$/.test(expr)) {
            try {
                const res = new Function('return ' + expr)();
                return `[Σ-RUN] Result: ${res}`;
            } catch(e) {
                return `[Σ-RUN] Math Error.`;
            }
        } else {
            return `[Σ-RUN] Access Denied: Unsafe characters detected.`;
        }
    }
    return null;
}








// REFACTORED TELEMETRY SYSTEM
class TelemetrySystem {
    constructor() {
        this.smoothCpu = 12;
        this.smoothMem = 4.2;
        this.batCharge = 87;
        this.peers = 0;
        this.animationFrameId = null;
        this.lastUpdateTime = 0;
        this.updateInterval = 2000;
        
        this.domCache = {
            cpuLoad: document.getElementById('cpu-load'),
            cpuProgress: document.getElementById('cpu-progress'),
            memLoad: document.getElementById('mem-load'),
            memProgress: document.getElementById('mem-progress'),
            vfsNodes: document.getElementById('vfs-nodes'),
            vfsProgress: document.getElementById('vfs-progress'),
            uiFrametime: document.getElementById('ui-frametime')
        };
    }
    start() {
        this.lastUpdateTime = Date.now();
        this.loop();
    }
    stop() {
        if (this.animationFrameId) cancelAnimationFrame(this.animationFrameId);
    }
    loop = () => {
        const now = Date.now();
        if (now - this.lastUpdateTime >= this.updateInterval) {
            this.updateAllMetrics();
            this.lastUpdateTime = now;
        }
        this.animationFrameId = requestAnimationFrame(this.loop);
    }
    updateAllMetrics() {
        this.updateCPUMetrics();
        this.updateMemoryMetrics();
        this.updateVFSMounts();
        this.updateUI();
    }
    updateCPUMetrics() {
        this.smoothCpu = Math.max(5, Math.min(95, this.smoothCpu + (Math.random() - 0.5) * 5));
        const cpu = Math.round(this.smoothCpu);
        if (this.domCache.cpuLoad) this.domCache.cpuLoad.textContent = cpu + "%";
        if (this.domCache.cpuProgress) this.domCache.cpuProgress.style.width = cpu + "%";
        if (typeof updateGraph === 'function') updateGraph(cpu);
    }
    updateMemoryMetrics() {
        this.smoothMem = Math.max(3.5, Math.min(8.0, this.smoothMem + (Math.random() - 0.5) * 0.08));
        if (this.domCache.memLoad) this.domCache.memLoad.textContent = this.smoothMem.toFixed(1) + " GB";
        if (this.domCache.memProgress) this.domCache.memProgress.style.width = (35 + this.smoothCpu * 0.45) + "%";
    }
    updateVFSMounts() {
        if (Math.random() > 0.96 && this.domCache.vfsNodes) {
            const nodes = Math.floor(Math.random() * 5) + 10;
            this.domCache.vfsNodes.textContent = `${nodes} Nodes`;
        }
        if (this.domCache.vfsProgress) this.domCache.vfsProgress.style.width = (75 + Math.random() * 10) + "%";
    }
    updateUI() {
        if (this.domCache.uiFrametime) this.domCache.uiFrametime.textContent = (8.0 + Math.random() * 0.5).toFixed(1) + 'ms';
    }
}
const telemetry = new TelemetrySystem();
window.addEventListener('load', () => telemetry.start());
window.addEventListener('beforeunload', () => telemetry.stop());

// =========================================================================
// Σ SIGMAOS: VIRTUAL FILE SYSTEM (VFS) ZENITH
// =========================================================================
class VirtualFS {
    constructor() {
        this.storageKey = 'sigma_vfs_zenith';
        this.root = JSON.parse(localStorage.getItem(this.storageKey)) || {
            name: '/',
            type: 'dir',
            children: {
                'bin': { name: 'bin', type: 'dir', children: {} },
                'etc': { name: 'etc', type: 'dir', children: {
                    'os-release': { name: 'os-release', type: 'file', content: 'NAME="SigmaOS"\nVERSION="100.0-ZENITH"\nCODENAME="Singularity"' }
                }},
                'home': { name: 'home', type: 'dir', children: {
                    'sovereign': { name: 'sovereign', type: 'dir', children: {
                        'welcome.txt': { name: 'welcome.txt', type: 'file', content: 'Welcome to SigmaOS Sovereign Zenith.\nYour files are now persistent.' },
                        'projects': { name: 'projects', type: 'dir', children: {} }
                    }}
                }},
                'tmp': { name: 'tmp', type: 'dir', children: {} }
            }
        };
        this.currentPath = '/home/sovereign';
    }

    save() {
        localStorage.setItem(this.storageKey, JSON.stringify(this.root));
    }

    resolve(path) {
        if (!path) return null;
        let parts = path.startsWith('/') ? path.split('/') : [...this.currentPath.split('/'), ...path.split('/')];
        parts = parts.filter(p => p !== '' && p !== '.');
        
        // Handle ..
        let resolvedParts = [];
        for (let p of parts) {
            if (p === '..') resolvedParts.pop();
            else resolvedParts.push(p);
        }

        let curr = this.root;
        for (let p of resolvedParts) {
            if (curr.type !== 'dir' || !curr.children[p]) return null;
            curr = curr.children[p];
        }
        return curr;
    }

    mkdir(path) {
        const parts = path.split('/');
        const name = parts.pop();
        const parentPath = parts.join('/') || this.currentPath;
        const parent = this.resolve(parentPath);
        if (parent && parent.type === 'dir' && !parent.children[name]) {
            parent.children[name] = { name: name, type: 'dir', children: {} };
            this.save();
            return true;
        }
        return false;
    }

    writeFile(path, content) {
        const parts = path.split('/');
        const name = parts.pop();
        const parentPath = parts.join('/') || this.currentPath;
        const parent = this.resolve(parentPath);
        if (parent && parent.type === 'dir') {
            parent.children[name] = { name: name, type: 'file', content: content };
            this.save();
            return true;
        }
        return false;
    }

    readFile(path) {
        const node = this.resolve(path);
        return (node && node.type === 'file') ? node.content : null;
    }

    ls(path = '.') {
        const node = this.resolve(path);
        if (node && node.type === 'dir') {
            return Object.keys(node.children);
        }
        return null;
    }
}

const vfs = new VirtualFS();

// SIGMA TERMINAL ENGINE
class SigmaTerminal {
    constructor(outputId, inputId) {
        this.output = document.getElementById(outputId);
        this.input = document.getElementById(inputId);
        this.history = [];
        this.historyPtr = -1;
        
        if (this.input) {
            this.input.addEventListener('keydown', (e) => {
                if (e.key === 'Enter') {
                    const cmd = this.input.value;
                    this.execute(cmd);
                    this.input.value = '';
                }
            });
        }
    }

    print(text, type = '') {
        if (!this.output) return;
        const div = document.createElement('div');
        div.className = `term-line ${type}`;
        div.innerHTML = text.replace(/\n/g, '<br>');
        this.output.appendChild(div);
        this.output.scrollTop = this.output.scrollHeight;
    }

    execute(line) {
        this.print(`<span class="term-prompt">sovereign@sigma:${vfs.currentPath}$</span> ${line}`);
        const parts = line.trim().split(/\s+/);
        const cmd = parts[0].toLowerCase();
        const args = parts.slice(1);

        switch(cmd) {
            case 'help':
                this.print('Available Commands: ls, cd, pwd, cat, mkdir, touch, rm, clear, sc, info, reboot');
                break;
            case 'ls':
                const files = vfs.ls(args[0] || '.');
                if (files) this.print(files.join('  '));
                else this.print('ls: cannot access directory', 'error');
                break;
            case 'pwd':
                this.print(vfs.currentPath);
                break;
            case 'cd':
                const newDir = args[0] || '/home/sovereign';
                const resolved = vfs.resolve(newDir);
                if (resolved && resolved.type === 'dir') {
                    if (newDir.startsWith('/')) vfs.currentPath = newDir;
                    else {
                        // Very basic relative path handling
                        if (newDir === '..') {
                            const p = vfs.currentPath.split('/');
                            p.pop();
                            vfs.currentPath = p.join('/') || '/';
                        } else {
                            vfs.currentPath = (vfs.currentPath === '/' ? '' : vfs.currentPath) + '/' + newDir;
                        }
                    }
                } else this.print(`cd: no such directory: ${newDir}`, 'error');
                break;
            case 'cat':
                const content = vfs.readFile(args[0]);
                if (content !== null) this.print(content);
                else this.print(`cat: ${args[0]}: No such file`, 'error');
                break;
            case 'mkdir':
                if (vfs.mkdir(args[0])) this.print(`Directory created: ${args[0]}`);
                else this.print(`mkdir: failed to create ${args[0]}`, 'error');
                break;
            case 'touch':
                if (vfs.writeFile(args[0], '')) this.print(`File created: ${args[0]}`);
                else this.print(`touch: failed to create ${args[0]}`, 'error');
                break;
            case 'clear':
                if (this.output) this.output.innerHTML = '';
                break;
            case 'sc': // Sigma Compiler (Mock Toolchain)
                this.print('Σ [TOOLCHAIN]: Porting Clang/LLVM to lattice...');
                this.print(`Σ [TOOLCHAIN]: Compiling ${args[0]} -> /bin/${args[0].replace('.c', '')}`);
                setTimeout(() => this.print('Σ [TOOLCHAIN]: Success. Binary injected.', 'success'), 1000);
                break;

            case 'reboot':
                this.print('Σ [KERN]: Soft reboot initiated...');
                setTimeout(() => location.reload(), 1000);
                break;
            case 'browse':
                const url = args[0] || 'sigma://lattice';
                launchApp('Sigma Browser');
                const browserInput = document.getElementById('browser-url');
                if (browserInput) {
                    browserInput.value = url;
                    if (typeof sigmaBrowseGo === 'function') sigmaBrowseGo();
                }
                this.print(`Σ [WEB]: Redirecting to ${url}...`);
                break;
            case 'shard':
                this.print('Σ [LATTICE]: Listing active shards...');
                this.print('>> Shard-0x8F (Zenith-UI): ACTIVE');
                this.print('>> Shard-0x92 (VFS-Core): ACTIVE');
                this.print('>> Shard-0x44 (PQC-Shield): ACTIVE');
                break;
            case 'info':
                this.print('Σ SIGMAOS: SOVEREIGN ZENITH v100.0');
                this.print('Host: Sigma-Lattice-01');
                this.print('Uptime: 12ms (Singularity Scale)');
                this.print('Memory: 4.2 GB / 16.0 GB');
                this.print('FS: Persistent (localStorage)');
                break;

            default:
                if (line.trim() !== '') this.print(`sigma: command not found: ${cmd}`, 'error');
        }
    }
}

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
// Σ SIGMAOS: SIGMA CORE & ADAPTIVE WORKFLOW ENGINE
// =========================================================================
class SigmaCore {
    constructor() {
        this.currentMode = 'Balanced';
        this.modes = {



            'Balanced': { accent: '#00ffa3', bg: 'rgba(5, 5, 7, 0.95)', cpu: 'BALANCED' },
            'Gamer': { accent: '#ff00ff', bg: 'rgba(10, 5, 15, 0.98)', cpu: 'TURBO' },
            'Creator': { accent: '#00c3ff', bg: 'rgba(5, 10, 15, 0.98)', cpu: 'MAX-THREADS' },
            'Streamer': { accent: '#ff3300', bg: 'rgba(15, 5, 5, 0.98)', cpu: 'ENCODE-PRIO' },
            'Red Team': { accent: '#ff0055', bg: 'rgba(10, 5, 5, 0.98)', cpu: 'MAX-PERF' },
            'Coding': { accent: '#00c3ff', bg: 'rgba(5, 7, 10, 0.98)', cpu: 'PERFORMANCE' },
            'Minimal': { accent: '#ffffff', bg: 'rgba(0, 0, 0, 1)', cpu: 'POWERSAVE' }



        };
    }

    setMode(modeName) {
        const mode = this.modes[modeName];
        if (!mode) return;

        this.currentMode = modeName;
        document.documentElement.style.setProperty('--accent', mode.accent);
        document.documentElement.style.setProperty('--accent-glow', mode.accent + '66');
        const activeWorkflowEl = document.getElementById('active-workflow');
        if (activeWorkflowEl) activeWorkflowEl.innerText = `MODAL: ${modeName.toUpperCase()}`;
        
        addLog(`Σ [CORE]: Workflow Engine optimized for ${modeName}. Mode: ${mode.cpu}.`, 'success');
        
        // Notify Kernel (Simulated)
        if (typeof ipc !== 'undefined') {
            ipc.syscall('SYS_SET_CPU_GOVERNOR', { mode: mode.cpu });
        }
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

if (commandInput) {
    commandInput.addEventListener('input', (e) => {
        const query = e.target.value.toLowerCase();
        const filtered = availableCommands.filter(cmd => 
            cmd.label.toLowerCase().includes(query) || cmd.hint.toLowerCase().includes(query)
        );
        
        renderCommandResults(filtered);
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
    
    // Simple mock tab switching
    main.innerHTML = `<h3>Security Shard: ${tab.toUpperCase()}</h3><p class="stat-label">Initializing intelligent auditing for ${tab}...</p>`;
    addLog(`Σ [SEC]: Mission Control switched to ${tab} tab.`, "success");
}

// Update start menu launch

// SIGMA CONFIG ENGINE
class SigmaConfig {
    constructor() {
        this.data = {
            desktop: { opacity: 0.95, blur: 20 },
            ai: { enabled: true, autoOptimize: true },
            services: { boot: 'FAST' }
        };
    }

    update(path, value) {
        addLog(`Σ [CONFIG]: Updating ${path} to ${value}...`, "success");
        if (path === 'desktop.opacity') {
            document.body.style.backgroundColor = `rgba(3, 3, 5, ${value/100})`;
        }
    }
}

// PERFORMANCE HUD LOGIC
function toggleOverlay() {
    const hud = document.getElementById('performance-hud');
    if (hud) {
        hud.classList.toggle('hidden');
        addLog(`Σ [HUD]: Overlay ${hud.classList.contains('hidden') ? 'OFF' : 'ON'}.`, "success");
    }
}

// Update HUD Telemetry
setInterval(() => {
    const cpu = document.getElementById('hud-cpu');
    const mem = document.getElementById('hud-mem');
    if (cpu) cpu.textContent = (Math.random() * 20 + 10).toFixed(1) + "%";
    if (mem) mem.textContent = (Math.random() * 0.5 + 4.1).toFixed(1) + "GB";
}, 2000);

// Unified Search Logic
window.performSearch = function(query) {
    const results = document.getElementById('command-results');
    if (!results) return;
    
    const data = [
        { name: 'OmniShell', type: 'app', action: "launchApp('OmniShell')" },
        { name: 'File Manager', type: 'app', action: "launchApp('File Manager')" },
        { name: 'Sigma Browser', type: 'app', action: "launchApp('Sigma Browser')" },
        { name: 'System Installer', type: 'app', action: "launchApp('System Installer')" },
        { name: 'Lattice Settings', type: 'setting', action: "launchApp('Utility Nexus')" },
        { name: 'Theme: Neon Cyan', type: 'command', action: "setTheme('cyan')" },
        { name: 'Theme: Solar Gold', type: 'command', action: "setTheme('gold')" },
        { name: 'Theme: Crimson Shard', type: 'command', action: "setTheme('crimson')" },
        { name: 'NPWO Turbo Mode', type: 'toggle', action: "toggleTurboMode()" }
    ];

    const filtered = data.filter(item => item.name.toLowerCase().includes(query.toLowerCase()));
    
    results.innerHTML = filtered.map(item => `
        <div class="command-item" onclick="${item.action}; document.getElementById('cmd-palette').classList.remove('active')">
            <span>${item.name}</span>
            <kbd>${item.type.toUpperCase()}</kbd>
        </div>
    `).join('');
};

document.getElementById('cmd-input')?.addEventListener('input', (e) => performSearch(e.target.value));

// Sigma Settings Hub Tab Logic
window.switchSettings = function(tab) {
    const main = document.getElementById('settings-main');
    if (!main) return;
    document.querySelectorAll('.settings-nav').forEach(n => {
        n.classList.remove('active');
        if (n.textContent.toLowerCase().includes(tab)) n.classList.add('active');
    });

    if (tab === 'services') {
        main.innerHTML = `<h3>Service Manager</h3><div class="settings-group"><label>Lattice Orchestrator</label><span class="status-success">RUNNING</span></div><div class="settings-group"><label>PQC Cryptography</label><span class="status-success">HARDENED</span></div><div class="settings-group"><label>Aether Network Shard</label><button class="util-btn" onclick="addLog('Σ [SVC]: Restarting Aether Shard...', 'warning')">RESTART</button></div>`;
    } else if (tab === 'config') {
        main.innerHTML = `<h3>Config Engine (YAML)</h3><textarea class="util-input" style="height: 150px; font-family: monospace;">system:\n  kernel: sovereign-v100\n  security: lattice-pqc\n  ui: zenith-fluid</textarea><button class="util-btn" onclick="addLog('Σ [CONFIG]: YAML Manifest deployed.', 'success')">DEPLOY CONFIG</button>`;
    } else if (tab === 'modules') {
        main.innerHTML = `<h3>Dynamic Module Manager</h3><p class="stat-label">Hot-swap drivers and UI shards without rebooting.</p><div class="settings-group"><label>GPU Driver v2.1</label><button class="util-btn" onclick="addLog('Σ [MODULE]: Hot-swapping GPU Driver...', 'success')">RELOAD</button></div><div class="settings-group"><label>Aether Network Shard</label><button class="util-btn" onclick="addLog('Σ [MODULE]: Shard reloaded.', 'success')">RELOAD</button></div>`;
    } else if (tab === 'benchmarks') {
        main.innerHTML = `<h3>Benchmark Laboratory</h3><p class="stat-label">Measure silicon-native performance and lattice throughput.</p><div class="settings-group"><label>Core Latency</label><span>0.04μs</span></div><div class="settings-group"><label>Lattice Sync Rate</label><span>8.2 GB/s</span></div><button class="util-btn" onclick="addLog('Σ [BENCH]: Running Stress Test...', 'warning')">START STRESS TEST</button>`;
    } else if (tab === 'accessibility') {
        main.innerHTML = `<h3>Accessibility</h3><p class="stat-label">Inclusive design for the Sovereign Lattice.</p><div class="settings-group"><label>Screen Reader</label><button class="util-btn" onclick="addLog('Σ [UI]: Screen Reader ENABLED.', 'success')">ENABLE</button></div><div class="settings-group"><label>High Contrast</label><button class="util-btn" onclick="addLog('Σ [UI]: High Contrast mode active.', 'success')">ENABLE</button></div>`;
    } else if (tab === 'healing') {
        main.innerHTML = `<h3>Auto-Healing</h3><p class="stat-label">AI-driven diagnostics and lattice repair.</p><div class="settings-group"><label>Health: 100%</label><button class="util-btn" onclick="sigmaCore.healSystem()">START AUDIT</button></div>`;

    } else {
        main.innerHTML = `<h3>${tab.toUpperCase()}</h3><p class="stat-label">Integrating ${tab} services...</p>`;
    }
    addLog(`Σ [SETTINGS]: Switched to ${tab}.`, "success");
};

// MORPHIC LAYOUT ENGINE
let draggedWidget = null;

window.handleDragStart = function(e) {
    draggedWidget = e.target.closest('.card');
    e.dataTransfer.effectAllowed = 'move';
};

window.handleDragOver = function(e) {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
};

window.handleDrop = function(e) {
    e.preventDefault();
    const target = e.target.closest('.card');
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

const themeEngine = new DynamicThemeEngine();
const automation = new SigmaAutomationEngine();

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

// Master launchApp Controller
const baseLaunchApp = typeof launchApp === 'function' ? launchApp : (a) => console.log("Launching: " + a);
window.launchApp = function(app) {
    if (app === 'Security Mission Control') {
        const win = document.getElementById('security-mission-control-win');
        if (win) {
            win.style.display = 'block';
            win.style.zIndex = '3000';
        }
    } else if (app === 'Sigma Settings') {
        const win = document.getElementById('sigma-settings-win');
        if (win) {
            win.style.display = 'block';
            win.style.zIndex = '3000';
        }
    } else if (app === 'System Installer') {
        addLog('Σ [INSTALLER]: Bootstrapping Sovereign Onboarding Wizard...', 'success');
        setTimeout(() => addLog('Σ [INSTALLER]: Deployment target: /dev/lattice_nvme0n1', 'info'), 1000);
    } else if (app === 'Capsule Library') {
        const win = document.getElementById('sigma-capsule-win');
        if (win) {
            win.style.display = 'block';
            win.style.zIndex = '3000';
        }
    } else {
        baseLaunchApp(app);
    }
};

// Singleton Initializations
if (typeof sigmaConfig === 'undefined') window.sigmaConfig = new SigmaConfig();
if (typeof sigmaCore === 'undefined') window.sigmaCore = new SigmaCore();
if (typeof automationEngine === 'undefined') window.automationEngine = new SigmaAutomationEngine();


// SOVEREIGN MAINTENANCE DAEMON
class SovereignMaintenanceDaemon {
    constructor() {
        this.init();
    }
    init() {
        setInterval(() => this.runHygiene(), 300000); // Every 5 minutes
    }
    runHygiene() {
        addLog('S [MAINT]: Background hygiene cycle started...', 'info');
        // Simulate cleanup
        setTimeout(() => addLog('S [MAINT]: Cache purged. 142MB reclaimed.', 'success'), 2000);
    }
}
const maintenance = new SovereignMaintenanceDaemon();

// CUSTOM CONTEXT MENU SYSTEM
window.addEventListener('contextmenu', (e) => {
    e.preventDefault();
    const menu = document.getElementById('context-menu');
    if (!menu) return;
    
    menu.style.display = 'block';
    menu.style.left = \\px\;
    menu.style.top = \\px\;
    menu.style.zIndex = '10000';
});

window.addEventListener('click', () => {
    const menu = document.getElementById('context-menu');
    if (menu) menu.style.display = 'none';
});

window.contextAction = function(action) {
    addLog(\S [UI]: Context Action: \\, 'success');
    if (action === 'refresh') location.reload();
};

window.toggleHelp = function() {
    const help = document.getElementById('help-overlay');
    if (help) help.classList.toggle('wizard-overlay--hidden');
};
