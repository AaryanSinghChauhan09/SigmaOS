
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
\n/** SigmaOS Zenith Desktop — https://github.com/AaryanSinghChauhan09/SigmaOS */
'use strict';

const SIGMA_APP_VERSION = '100.0';
const SIGMA_REPO_URL = 'https://github.com/AaryanSinghChauhan09/SigmaOS';

/* Σ Neural UI Layout Engine (NeuralWM) */
        class NeuralLayoutEngine {
            constructor() {
                this.interactions = {};
            }
            track(shardId) {
                this.interactions[shardId] = (this.interactions[shardId] || 0) + 1;
                this.reorganize();
            }
            reorganize() {
                const sorted = Object.entries(this.interactions).sort((a,b) => b[1] - a[1]);
                sorted.forEach(([id, count], i) => {
                    const el = document.getElementById(id);
                    if (el && i === 0) el.classList.add('neural-active');
                    else if (el) el.classList.remove('neural-active');
                });
            }
            setMindfulness(active) {
                document.body.classList.toggle('focus-mode-active', active);
                addLog(active ? 'Σ [NEURAL]: Mindfulness Mode ACTIVE. Filtering noise...' : 'Σ [NEURAL]: Mindfulness Mode DISABLED.');
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
            }
            register(id) {
                const win = document.getElementById(id);
                if (!win) return;
                this.windows.push(win);
                win.addEventListener('mousedown', () => this.bringToFront(win));
            }
            bringToFront(win) {
                this.topZ = this.topZ >= 9999 ? 1000 : this.topZ + 1;
                win.style.zIndex = String(this.topZ);
            }
            tileAll() {
                const width = window.innerWidth / this.windows.length;
                this.windows.forEach((win, i) => {
                    win.style.width = `${width - 20}px`;
                    win.style.left = `${i * width + 10}px`;
                    win.style.top = '100px';
                });
            }
        }
        const wm = new ZenithWM();

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
        const cmdPalette = document.getElementById('cmd-palette');
        const cmdInput = document.getElementById('cmd-input');

        window.addEventListener('keydown', (e) => {
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
        });

        if (cmdInput) {
            
function createDebounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => { clearTimeout(timeout); func(...args); };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}
\n        const filterCommands = createDebounce((q) => {\n            document.querySelectorAll('#cmd-results .command-item').forEach((el) => {\n                const t = el.textContent.toLowerCase();\n                el.style.display = !q || t.includes(q) ? '' : 'none';\n            });\n        }, 200);\n\n        cmdInput.addEventListener('input', (e) => {\n            const q = e.target.value.trim().toLowerCase();\n            filterCommands(q);\n        });\n/*
                const q = cmdInput.value.trim().toLowerCase();
                document.querySelectorAll('#cmd-results .command-item').forEach((el) => {
                    const t = el.textContent.toLowerCase();
                    el.style.display = !q || t.includes(q) ? '' : 'none';
                });
            });
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
        }\n\n        function flashBootable() {
            document.getElementById('boot-target').innerText = "SiliconDrive (64GB) [LOCKED]";
            const barContainer = document.getElementById('flash-progress');
            const bar = document.getElementById('flash-bar');
            barContainer.style.display = 'block';
            let progress = 0;
            const interval =         }

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
            const gauge = document.getElementById('speed-gauge');
            let speed = 0;
            const interval = 
            if (Math.random() > 0.99) {
                const faultIdx = Math.floor(Math.random() * 500);
                dots[faultIdx].classList.add('error');
                document.getElementById('shard-status').textContent = "Integrity: 99.8% (Fault Detected)";
                document.getElementById('shard-status').style.color = "var(--error)";

                setTimeout(() => {
                    dots[faultIdx].classList.remove('error');
                    dots[faultIdx].classList.add('healing');
                    document.getElementById('shard-status').textContent = "Integrity: 100.0% (Self-Healed)";
                    document.getElementById('shard-status').style.color = "var(--success)";
                    setTimeout(() => dots[faultIdx].classList.remove('healing'), 2000);
                }, 1500);
            }
        }, 2000);

        // Inject 999 Shards for 999+ Singularity
        const matrix = document.getElementById('shard-matrix');
        for (let i = 0; i < 999; i++) {
            const dot = document.createElement('div');
            dot.className = 'shard-dot';
            if (Math.random() > 0.95) dot.classList.add('active');
            matrix.appendChild(dot);
        }

        // DNA Telemetry Sync
        let totalSaved = 0;
        setInterval(() => {
            totalSaved += Math.floor(Math.random() * 500);
            document.getElementById('dna-savings').innerText = (totalSaved / 1024).toFixed(2) + " MB";
        }, 2000);

        // Lattice Mesh Discovery & Streaming
        let peers = 0;
        const meshView = document.getElementById('mesh-view');
        const meshSvg = document.getElementById('mesh-svg');
        const meshNodes = [];

        function discoverPeer() {
            peers++;
            document.getElementById('peer-count').innerText = peers;
            const node = document.createElement('div');
            node.className = 'mesh-node mesh-node-pulse';
            
            const x = Math.random() * 90;
            const y = Math.random() * 90;
            node.style.left = x + '%';
            node.style.top = y + '%';
            
            if (meshView) meshView.appendChild(node);
            meshNodes.push({x, y});

            // Simulate Data Stream to a random previous node
            if (peers > 1 && meshSvg) {
                const targetIdx = Math.floor(Math.random() * (peers - 1));
                const target = meshNodes[targetIdx];
                
                const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
                line.setAttribute('x1', x + '%');
                line.setAttribute('y1', y + '%');
                line.setAttribute('x2', target.x + '%');
                line.setAttribute('y2', target.y + '%');
                line.setAttribute('stroke', 'var(--accent)');
                line.setAttribute('stroke-width', '1');
                line.setAttribute('stroke-dasharray', '5,5');
                line.style.opacity = '0.5';
                
                // Animate stream
                let offset = 0;
                setInterval(() => {
                    offset -= 1;
                    line.setAttribute('stroke-dashoffset', offset);
                }, 50);

                meshSvg.appendChild(line);
            }
        }
        setInterval(() => { if (peers < 12) discoverPeer(); }, 5000);
        // Silicon-Direct Transpiler Telemetry
        let transpiledDrivers = 0;
        const driverElement = document.getElementById('transpiled-count');
        const isaElement = document.getElementById('silicon-isa');
        
        setInterval(() => {
            if(transpiledDrivers < 24) {
                transpiledDrivers += Math.floor(Math.random() * 3);
                if (driverElement) driverElement.innerText = transpiledDrivers;
            }
            if(Math.random() > 0.98) {
                isaElement.innerText = "ARM (AArch64)";
            } else if (Math.random() > 0.98) {
                isaElement.innerText = "RISC-V (RV64GC)";
            }
        }, 3000);
        // Neural Automator Telemetry
        const cognitiveTasks = ["Preemptive VRAM Caching", "Predictive Shard Loading", "Background Lattice Audit", "DNA Re-indexing", "Quantum Key Rotation"];
        const queueElement = document.getElementById('cognitive-queue');
        const currentTaskElement = document.getElementById('auto-current-task');
        const countElement = document.getElementById('auto-task-count');
        const autoProgress = document.getElementById('auto-progress');
        let activeTasks = 0;

        setInterval(() => {
            if (Math.random() > 0.6) {
                // Schedule new cognitive task
                const task = cognitiveTasks[Math.floor(Math.random() * cognitiveTasks.length)];
                const taskEl = document.createElement('div');
                taskEl.innerText = ">> " + task;
                taskEl.style.opacity = '0';
                taskEl.style.transition = 'opacity 0.5s';
                queueElement.prepend(taskEl);
                setTimeout(() => taskEl.style.opacity = '1', 50);
                
                activeTasks++;
                countElement.innerText = activeTasks + " Tasks Active";
                currentTaskElement.innerText = "Running: " + task;
                
                // Animate progress bar
                autoProgress.style.transition = 'width 2s linear';
                autoProgress.style.width = '100%';
                
                // Task completion
                setTimeout(() => {
                    activeTasks--;
                    countElement.innerText = activeTasks + " Tasks Active";
                    if (queueElement.contains(taskEl)) {
                        queueElement.removeChild(taskEl);
                    }
                    if (activeTasks === 0) {
                        currentTaskElement.innerText = "Next: Idle";
                    }
                    autoProgress.style.transition = 'none';
                    autoProgress.style.width = '0%';
                }, 2000);
            }
        }, 3500);

        // Orb Exchange Telemetry
        const orbList = ["NeuralVisualizer-v2", "QuantumSieve-PQC", "BioFS-DNA-Module", "CryptoLedger-X"];
        const orbStatus = document.getElementById('orb-active');
        const orbProgress = document.getElementById('orb-progress');
        
        setInterval(() => {
            if (Math.random() > 0.8) {
                const orb = orbList[Math.floor(Math.random() * orbList.length)];
                orbStatus.innerText = "Summoning: " + orb;
                orbProgress.style.transition = 'width 1.5s linear';
                orbProgress.style.width = '100%';
                
                setTimeout(() => {
                    orbStatus.innerText = "Idle";
                    orbProgress.style.transition = 'none';
                    orbProgress.style.width = '0%';
                }, 1600);
            }
        }, 4000);
        
        // Community Governance Telemetry
        let activeProposals = 0;
        const propElement = document.getElementById('gov-active');
        const voteElement = document.getElementById('gov-vote');
        const govProgress = document.getElementById('gov-progress');
        
        setInterval(() => {
            if(Math.random() > 0.7) {
                activeProposals = Math.floor(Math.random() * 12) + 1;
                propElement.innerText = activeProposals + " Active Proposals";
                
                if(Math.random() > 0.5) {
                    voteElement.innerText = "YEA (QKD-VERIFIED)";
                    voteElement.className = "status-success";
                    govProgress.style.width = (50 + Math.random() * 40) + '%';
                    govProgress.style.background = 'var(--success)';
                    govProgress.style.boxShadow = '0 0 10px var(--success)';
                } else {
                    voteElement.innerText = "NAY (QKD-VERIFIED)";
                    voteElement.className = "accent";
                    govProgress.style.width = (10 + Math.random() * 40) + '%';
                    govProgress.style.background = 'var(--error)';
                    govProgress.style.boxShadow = '0 0 10px var(--error)';
                }
            }
        }, 5000);

        // Trust Fabric Lockdown Mode
        setInterval(() => {
            if(Math.random() > 0.95) {
                // Trigger Lockdown
                document.body.classList.add('lockdown-mode');
                document.getElementById('qkd-status').innerText = "ANOMALY DETECTED";
                document.getElementById('qkd-status').style.color = "var(--error)";
                document.getElementById('orb-trust').innerText = "LOCKDOWN ACTIVE";
                document.getElementById('orb-trust').className = "accent";
                
                // Heal after 5 seconds
                setTimeout(() => {
                    document.body.classList.remove('lockdown-mode');
                    document.getElementById('qkd-status').innerText = "Entangled";
                    document.getElementById('qkd-status').style.color = "var(--text-light)";
                    document.getElementById('orb-trust').innerText = "QKD-VERIFIED";
                    document.getElementById('orb-trust').className = "status-success";
                }, 5000);
            }
        }, 8000);

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
    const input = document.getElementById('data-forge-input').value;
    const output = document.getElementById('data-forge-output');
    if(!input) return;
    
    output.innerHTML = '<span class=\"text-accent\">[SDP] Initializing 600-shard parallel map/reduce...</span><br>';
    setTimeout(() => {
        output.innerHTML += '<span class=\"text-blue\">[SDP] Data Sharding complete. Partitioning into 4096 blocks...</span><br>';
        setTimeout(() => {
            output.innerHTML += '<span class=\"text-success\">[SDP] Forge Complete. Latency: 0.12ms. Sovereignty Absolute.</span><br>';
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







\n
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
