/**
 * Σ SIGMA OS KERNEL v2.2
 * Core Logic Engine, Module Loader & Process Scheduler
 * Now featuring Dynamic Module Interop & Persistent VFS
 */

export const SigmaKernel = {
    version: '2.2.0-Sovereign-Quantum',
    uptime: 0,
    processes: [],
    modules: new Map(),
    vfs: {
        '/': { type: 'dir', children: ['bin', 'etc', 'home', 'var'], meta: { created: Date.now() } },
        '/bin': { type: 'dir', children: ['sh', 'ls', 'help', 'uptime', 'cat', 'ps', 'kill', 'clear'], meta: { created: Date.now() } },
        '/etc': { type: 'dir', children: ['system.conf', 'hosts', 'motd'], meta: { created: Date.now() } },
        '/home': { type: 'dir', children: ['sigma'], meta: { created: Date.now() } },
        '/home/sigma': { type: 'dir', children: ['documents', 'downloads', 'projects'], meta: { created: Date.now() } },
        '/home/sigma/documents': { type: 'dir', children: ['readme.txt'], meta: { created: Date.now() } },
        '/var': { type: 'dir', children: ['logs', 'locks'], meta: { created: Date.now() } },
        '/var/logs': { type: 'dir', children: ['boot.log', 'auth.log'], meta: { created: Date.now() } }
    },
    currentPath: '/home/sigma/',

    /** VFS API - High Performance File Interaction */
    ls(path = this.currentPath) {
        return this.vfs[path] ? this.vfs[path].children : [];
    },

    readFile(path) {
        // Simulated file read from VFS state
        return this.vfs[path] ? this.vfs[path].content : "FILE_NOT_FOUND";
    },

    writeFile(path, content) {
        if (!this.vfs[path]) {
            this.vfs[path] = { type: 'file', content: content, meta: { lastModified: Date.now() } };
            // Auto-update parent directory linkage logic would go here
        } else {
            this.vfs[path].content = content;
            this.vfs[path].meta.lastModified = Date.now();
        }
        this.notify(`VFS: Wrote to ${path}`, 'success');
    },

    async boot() {
        console.log("%cΣ ΣIGMA WEB OS KERNEL v2.2 BOOT SEQUENCE INITIATED", "color:#5AC8FA;font-weight:bold;font-size:16px;");
        this.uptime = Date.now();
        this.setupErrorHandling();
        this.initProcessTable();

        await this.loadCoreModules();
        this.enforceIsolationProtocols();

        this.notify("Kernel logical integrity verified.", "success");
        console.log("%cKERNEL BOOT SUCCESSFUL. READY FOR SOVEREIGN OPERATIONS.", "color:green;font-weight:bold;");
    },

    async loadCoreModules() {
        console.log("[KERNEL] Loading Core Logic Streams...");
        try {
            const ui = await import('./ui.js');
            const themes = await import('./themes.js');
            const shaders = await import('./shaders.js');
            const assistant = await import('./assistant.js');
            const telemetry = await import('./telemetry_shield.js');
            const sandbox = await import('./sandbox.js');
            const mesh = await import('./mesh.js');
            const infinity = await import('./infinity_search.js');
            const predictor = await import('./neural_predictor.js'); // [NEW ML CORE]
            const behavior = await import('./ml_behavior_engine.js');
            const recs = await import('./ml_recommendation_engine.js');

            this.modules.set('ui', ui.UIEngine);
            this.modules.set('themes', themes.ThemeEngine);
            this.modules.set('shaders', shaders.ShaderEngine);
            this.modules.set('assistant', assistant.AIAssistant);
            this.modules.set('telemetry', telemetry.TelemetryShield);
            this.modules.set('sandbox', sandbox.BrowserSandbox);
            this.modules.set('mesh', mesh.MeshNetwork);
            this.modules.set('infinity', infinity.InfinitySearch);
            this.modules.set('predictor', predictor.NeuralPredictor);
            this.modules.set('behavior', behavior.BehaviorEngine);
            this.modules.set('recs', recs.RecommendationEngine);

            // Initialize core services
            ui.UIEngine.init();
            themes.ThemeEngine.init();
            shaders.ShaderEngine.init();
            assistant.AIAssistant.init();
            mesh.MeshNetwork.init();
            infinity.InfinitySearch.init();
            predictor.NeuralPredictor.init();
            behavior.BehaviorEngine.init();
            recs.RecommendationEngine.init();

            // Hook recommendations into UIEngine.launch
            if (window.UIEngine) {
                const _origLaunch = window.UIEngine.launch.bind(window.UIEngine);
                window.UIEngine.launch = async function(id) {
                    const result = await _origLaunch(id);
                    recs.RecommendationEngine.renderSuggestionsHUD(id);
                    return result;
                };
            }

        } catch (error) {
            console.error("KERNEL_PANIC: Core logical components missing or corrupted.", error);
            this.notify("SIGMA_LOADER_FAULT: Module interop stream interrupted.", "error");
        }
    },

    setupErrorHandling() {
        window.onerror = (msg, src, lineno, colno, err) => {
            console.error("KERNEL EXCEPTION:", msg, err);
            this.notify(`KERNEL_TRAP: Process fault detected at ${src ? src.split('/').pop() : 'core'}:${lineno}. <br>Intercepted: ${msg.substring(0, 40)}...`, "error");
            return true;
        };

        window.onunhandledrejection = (event) => {
            console.error("ASYNC FAULT:", event.reason);
            this.notify(`ASYNC_INTERRUPT: Unhandled promise rejection via Mesh stream: ${event.reason}`, "error");
            event.preventDefault();
        };
    },

    notify(msg, type = 'info') {
        const errDiv = document.createElement('div');
        errDiv.className = `sovereign-error-toast toast-${type}`;

        let icon = 'ℹ️';
        if (type === 'error') icon = '⚠️';
        if (type === 'success') icon = '✅';

        errDiv.innerHTML = `${icon} <b style="letter-spacing:1px;">Σ SIGMA_OS</b><br>${msg}`;
        document.body.appendChild(errDiv);
        setTimeout(() => {
            errDiv.style.opacity = '0';
            errDiv.style.transform = 'translateX(100%)';
            setTimeout(() => errDiv.remove(), 500);
        }, 5000);
    },

    /** Deprecated - use notify() */
    notifyPanic(msg) { this.notify(msg, 'error'); },

    initProcessTable() {
        this.processes = [
            { id: 101, name: 'k_worker', cpu: 0, mem: 4.2 },
            { id: 102, name: 'mesh_svc', cpu: 1.2, mem: 12.5 },
            { id: 103, name: 'neural_bridge', cpu: 0.5, mem: 24.1 },
            { id: 104, name: 'ag_sync', cpu: 0.1, mem: 8.8 }
        ];

        // System HUD Management
        const hints = [
            "Use Ctrl+K for Global Discovery.",
            "Drag windows to edges for Snap tiling.",
            "Ask Assistant to 'Analyze this' for context.",
            "Check Mesh Network for node peering.",
            "Forensic Audit verifies logic integrity."
        ];
        let hintIndex = 0;

        setInterval(() => {
            // Process Updates
            this.processes.forEach(p => {
                p.cpu = (Math.random() * 2).toFixed(1);
                if (p.name === 'k_worker') p.cpu = (Math.random() * 0.5).toFixed(1);
            });
            if (window.updateDashPsTable) window.updateDashPsTable();

            // HUD Uptime & Entropy
            const upSecs = Math.floor((Date.now() - this.uptime) / 1000);
            const h = String(Math.floor(upSecs / 3600)).padStart(2, '0');
            const m = String(Math.floor((upSecs % 3600) / 60)).padStart(2, '0');
            const s = String(upSecs % 60).padStart(2, '0');
            const hudUp = document.getElementById('hud-uptime');
            if (hudUp) hudUp.textContent = `Uptime: ${h}:${m}:${s}`;

            // Entropy Meter
            const entPercent = (30 + Math.random() * 10).toFixed(1);
            const entFill = document.getElementById('entropy-fill');
            if (entFill) entFill.style.width = entPercent + '%';

            // Rotate Hints
            if (upSecs % 15 === 0) {
                hintIndex = (hintIndex + 1) % hints.length;
                const hh = document.getElementById('hud-hint');
                if (hh) {
                    hh.style.opacity = '0';
                    setTimeout(() => {
                        hh.textContent = hints[hintIndex];
                        hh.style.opacity = '1';
                    }, 400);
                }
            }
        }, 3000);

        // Kernel Watchdog - v3.0 Robustness
        setInterval(() => {
            const { AppRegistry, AppLoader } = window;
            if (!AppRegistry) return;

            AppRegistry.forEach(app => {
                if (app.status === 'error') {
                    console.warn(`[WATCHDOG] Detecting logic fault in component: ${app.id}. Attempting background recovery...`);
                    AppLoader.loadModule(app.id); // Re-attempt sync
                }
            });

            // Monitor UI Responsiveness (Simulated)
            if (Math.random() > 0.999) {
                this.notify("KERNEL: Detecting UI jitter. Optimizing stage manager for performance.", "info");
                if (window.UIEngine) window.UIEngine.updateStageManager();
            }

            // [NEW] Sovereign Auto-Suspend Automation
            if (window.UIEngine && window.UIEngine.openApps) {
                const active = window.UIEngine.activeWindow;
                window.UIEngine.openApps.forEach(id => {
                    const win = document.getElementById(`win-${id}`);
                    if (win && id !== active && !win.classList.contains('display-none')) {
                        // Sleep non-focused windows
                        win.style.filter = 'brightness(0.7) grayscale(0.5)';
                        win.style.transition = 'filter 0.5s';
                    } else if (win) {
                        // Wake up focused window
                        win.style.filter = 'brightness(1) grayscale(0)';
                    }
                });
            }

            // [NEW] ML Kernel Anomaly Detection Pipeline
            const predictor = this.modules.get('predictor');
            if (predictor) {
                const ml_analysis = predictor.detectAnomaly(this.processes);
                if (ml_analysis && ml_analysis.anomaly) {
                    this.notify(`[ML ENGINE] Predictive Heuristics detect anomalous CPU/RAM vector. Z-Score: ${ml_analysis.zScore.toFixed(2)}. Modulating Process Schedulers...`, ml_analysis.severity === 'CRITICAL' ? 'error' : 'info');
                    // AI Response: Randomly reduce CPU of non-essential processes to simulate ML autonomous response
                    this.processes.forEach(p => {
                        if (p.name !== 'k_worker') {
                            p.cpu = Math.max(0, parseFloat(p.cpu) - 2.0).toFixed(1);
                        }
                    });
                }
            }

            this.enforcePrivacyProtocols();
        }, 10000);
    },

    enforcePrivacyProtocols() {
        console.log("[SHIELD] Initiating Bloatware Audit...");
        const external = [];
        document.querySelectorAll('script, link[rel="stylesheet"]').forEach(el => {
            const src = el.src || el.href;
            if (src && !src.includes(window.location.origin) && !src.startsWith('data:')) {
                external.push(src);
                console.warn(`[SHIELD] 3rd Party Vector Detected: ${src}. Nullifying for Sovereign security.`);
                el.remove();
            }
        });

        if (external.length > 0) {
            this.notify(`Shield: ${external.length} 3rd party vectors nullified.`, 'success');
        } else {
            console.log("[SHIELD] Logic stream is 100% Sovereign (Pure).");
        }
    },

    enforceIsolationProtocols() {
        try {
            if (window.telemetry || window.ga || window.analytics) {
                this.notify("THIRD_PARTY_SHIM_DETECTED: Critical violation of Sovereign isolation protocols. Telemetry rejected.", 'error');
            }
        } catch (e) { }
    }
};

// Global exports for legacy event handlers in HTML
window.SigmaKernel = SigmaKernel;

// Immediate boot sequence
SigmaKernel.boot();
