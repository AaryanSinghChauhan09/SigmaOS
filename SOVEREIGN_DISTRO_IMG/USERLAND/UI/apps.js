/**
 * Σ SIGMA OS APPS SUITE v2.2
 * Business Logic for Sovereign Tools
 * Modularized for Enhanced Performance & Scalability
 */

export const Apps = {
    /** Text Cleaner Logic */
    cleanText() {
        const input = document.getElementById('tc_input').value;
        let out = input;

        if (document.getElementById('tc_trim').checked) out = out.split('\n').map(l => l.trim()).join('\n');
        if (document.getElementById('tc_breaks').checked) out = out.replace(/\n|\r/g, ' ');
        if (document.getElementById('tc_blank').checked) out = out.replace(/^\s*[\r\n]/gm, '');
        if (document.getElementById('tc_s_space').checked) out = out.replace(/ +(?= )/g, '');
        if (document.getElementById('tc_html').checked) out = out.replace(/<[^>]*>/g, '');
        if (document.getElementById('tc_url').checked) out = out.replace(/(https?:\/\/[^\s]+)/g, '');

        const custom = document.getElementById('tc_custom').value;
        if (custom) {
            const re = new RegExp(`[${custom}]`, 'g');
            out = out.replace(re, '');
        }

        document.getElementById('tc_output').value = out;
        SigmaKernel.notify("CLEANER: Logic successfully purged telemetry vectors.", "success");
    },

    /** Automation Hub Script Injection */
    injectScript() {
        const script = document.getElementById('auto_script').value;
        if (!script.trim()) return;

        const log = document.getElementById('auto-log');
        const entry = document.createElement('div');
        entry.textContent = `[${new Date().toLocaleTimeString()}] PARSING SCRIPT...`;
        log.prepend(entry);

        // Simple Automation Engine Parser
        const lines = script.split('\n');
        let delay = 500;

        lines.forEach(line => {
            setTimeout(() => {
                const step = document.createElement('div');
                if (line.startsWith('echo ')) {
                    step.textContent = `> ${line.substring(5)}`;
                } else if (line.startsWith('notify ')) {
                    SigmaKernel.notify(`SYS_EVENT: ${line.substring(7)}`, 'info');
                    step.textContent = `> Dispatching UI Notification`;
                } else if (line.startsWith('launch ')) {
                    const app = line.substring(7).trim();
                    if (window.UIEngine) UIEngine.launch(app);
                    step.textContent = `> Executing logical module: ${app}`;
                } else {
                    step.textContent = `> Exec_Shim: ${line}`;
                }
                log.prepend(step);
            }, delay);
            delay += 500;
        });

        setTimeout(() => {
            const finish = document.createElement('div');
            finish.textContent = `[AUTOMATION] Routine completed successfully.`;
            finish.style.color = "var(--accent)";
            log.prepend(finish);
            SigmaKernel.notify("AUTOMATION: Custom routine executed.", "success");
        }, delay + 200);

        document.getElementById('auto_script').value = '';
    },

    /** VFS Navigation */
    vfsNavigate(dir) {
        const display = document.getElementById('vfs-path-display');
        const files = document.getElementById('vfs-files');
        if (!display || !files) return;

        const truePath = dir === 'root' ? '/' : `/home/sigma/${dir}`;
        SigmaKernel.currentPath = truePath;
        display.textContent = `Viewing ${truePath}`;

        let items = [];
        if (window.SigmaKernel) {
            items = SigmaKernel.ls();
            if (items.length === 0 && dir !== 'root') items = ['log_dump.txt'];
        }

        files.innerHTML = items.map(f => `
            <div class="vfs-file" onclick="Apps.peekFile('${f}')" style="cursor: pointer; padding: 10px; border-radius: 4px; text-align: center;">
                <div class="font-24">📄</div>
                <div class="font-10 mt-5">${f}</div>
            </div>
        `).join('');
    },

    peekFile(filename) {
        const peek = document.getElementById('sovereign-peek');
        const content = document.getElementById('peek-content');
        if (!peek || !content) return;

        let data = "EMPTY_FILE_BUFFER";
        if (window.SigmaKernel) {
            data = SigmaKernel.readFile(SigmaKernel.currentPath + filename);
            if (data === "FILE_NOT_FOUND") {
                // Mock logic for demo
                if (filename === "readme.txt") data = "=== SOVEREIGN OS ===\nWelcome to SigmaOS v3.0\nFully Airgapped and Telemetry-Free.\n\nCompetitor USPs Absorbed:\n1. Apple Quick Look -> Sovereign Peek\n2. Windows Snapping -> Logical Tiling\n3. Spotlight -> Infinity Search\n4. Linux TTY -> Infinity Terminal";
                if (filename === "config.sys") data = "core_clock=999Mhz\ntelemetry=0\np2p_mesh=enabled\nui_engine=quantum";
                if (filename === "log_dump.txt") data = "[10:24:00] KERNEL BOOT\n[10:24:01] MODULES LOADED\n[10:24:05] VFS MOUNTED";
            }
        }

        content.innerText = data;
        peek.style.display = 'flex';
        SigmaKernel.notify(`Sovereign Peek engaged for ${filename}`, 'info');
    },

    closePeek() {
        const peek = document.getElementById('sovereign-peek');
        if (peek) peek.style.display = 'none';
    },

    /** Antigravity Cockpit & Quota Logic */
    agActiveAccount: null,
    agAccounts: [
        { id: 'primary', name: 'Primary Account', email: 'sovereign@sigma.os', quota: 85 },
        { id: 'vanguard', name: 'Vanguard Node', email: 'vanguard@sec.node', quota: 12 },
        { id: 'ghost', name: 'Ghost Identity', email: 'anonymous@p2p.mesh', quota: 0 }
    ],

    agInit() {
        this.agRefreshUI();
    },

    agRefreshUI() {
        const list = document.getElementById('ag-accounts-list');
        if (!list) return;
        list.innerHTML = this.agAccounts.map(acc => `
            <div class="ag-account ${this.agActiveAccount?.id === acc.id ? 'active' : ''}" onclick="Apps.agSelectAccount('${acc.id}')">
                ● ${acc.name}
            </div>
        `).join('');

        const cards = document.getElementById('ag-quota-cards');
        if (cards) {
            cards.innerHTML = this.agAccounts.map(acc => `
                <div class="dashboard-card mb-10">
                    <div class="sys-stat"><span>${acc.name}</span> <span>${acc.quota}%</span></div>
                    <div class="bar-bg"><div class="bar-fill" style="width:${acc.quota}%"></div></div>
                </div>
            `).join('');
        }
    },

    agSelectAccount(id) {
        this.agActiveAccount = this.agAccounts.find(a => a.id === id);
        document.getElementById('ag-active-badge').textContent = this.agActiveAccount.name;
        document.getElementById('ag-active-email').textContent = this.agActiveAccount.email;
        document.getElementById('ag-topbar-acct').textContent = `● ${this.agActiveAccount.id.toUpperCase()}`;
        this.agRefreshUI();
        SigmaKernel.notify(`ANTIGRAVITY: Account swap initiated. Token ${id.substring(0, 8)} active.`);
    },

    /** Neural Logic Analyzer */
    analyzerRunning: false,
    analyzerCanvas: null,
    analyzerCtx: null,

    analyzerToggle() {
        this.analyzerRunning = !this.analyzerRunning;
        const btn = document.getElementById('analyzer-toggle');
        btn.textContent = this.analyzerRunning ? 'STOP' : 'START';
        btn.className = this.analyzerRunning ? 'ag-btn ag-btn-danger p-2-5 font-10' : 'ag-btn p-2-5 font-10';

        if (this.analyzerRunning) {
            this.analyzerCanvas = document.getElementById('analyzer-canvas');
            this.analyzerCtx = this.analyzerCanvas.getContext('2d');
            this.analyzerLoop();
            SigmaKernel.notify("ANALYZER: Neural logic stream synchronized.", "success");
        }
    },

    analyzerLoop() {
        if (!this.analyzerRunning) return;
        const ctx = this.analyzerCtx;
        const w = this.analyzerCanvas.width;
        const h = this.analyzerCanvas.height;

        ctx.fillStyle = 'rgba(0, 20, 0, 0.1)';
        ctx.fillRect(0, 0, w, h);

        ctx.strokeStyle = getComputedStyle(document.documentElement).getPropertyValue('--accent');
        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.moveTo(0, h / 2);

        for (let x = 0; x < w; x += 5) {
            const y = h / 2 + Math.sin(x * 0.05 + Date.now() * 0.01) * 30 + (Math.random() - 0.5) * 10;
            ctx.lineTo(x, y);

            if (x % 100 === 0 && Math.random() > 0.98) {
                this.analyzerLog(`BIT_FLIP detected at memory nexus 0x${Math.floor(Math.random() * 0xFFFF).toString(16).toUpperCase()}`);
            }
        }
        ctx.stroke();

        requestAnimationFrame(() => this.analyzerLoop());
    },

    analyzerLog(msg) {
        const log = document.getElementById('analyzer-log');
        if (!log) return;
        const entry = document.createElement('div');
        entry.textContent = `[${new Date().toLocaleTimeString()}] ${msg}`;
        log.prepend(entry);
        if (log.children.length > 20) log.lastChild.remove();
    },

    analyzerReset() {
        const log = document.getElementById('analyzer-log');
        if (log) log.innerHTML = '<div class="text-dim">[INFO] Waiting for signal sync...</div>';
        SigmaKernel.notify("ANALYZER: Logic history purged.", "info");
    },

    /** Sovereign Vault */
    vaultGenerate() {
        const charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+";
        let pass = "";
        for (let i = 0; i < 24; i++) {
            pass += charset.charAt(Math.floor(Math.random() * charset.length));
        }
        const el = document.getElementById('vault-pass');
        if (el) el.value = pass;
        SigmaKernel.notify("VAULT: High-entropy logic sequence generated.", "success");
    }
};

// Global exports
window.Apps = Apps;
window.cleanText = () => Apps.cleanText();
window.copyCleanedText = () => {
    const txt = document.getElementById('tc_output');
    if (!txt) return;
    txt.select();
    document.execCommand('copy');
    SigmaKernel.notify("CLEANER: Results cached to system clipboard.", "success");
};
window.injectScript = () => Apps.injectScript();
window.vfsNavigate = (dir) => Apps.vfsNavigate(dir);
window.closePeek = () => Apps.closePeek();
window.agSwitchTab = (tab, btn) => {
    ['quota', 'cockpit', 'dispatch'].forEach(t => {
        const el = document.getElementById(`ag-tab-${t}`);
        if (el) el.classList.add('display-none');
    });
    const target = document.getElementById(`ag-tab-${tab}`);
    if (target) target.classList.remove('display-none');

    document.querySelectorAll('.ag-tab').forEach(t => t.classList.remove('active'));
    btn.classList.add('active');
};
window.calcInput = (key) => {
    const disp = document.getElementById('calc-display');
    if (key === '=') {
        try { disp.value = eval(disp.value); } catch (e) { disp.value = 'ERROR'; }
    } else if (key === 'C') {
        disp.value = '0';
    } else {
        if (disp.value === '0') disp.value = key;
        else disp.value += key;
    }
};

// Auto-init for some apps
setTimeout(() => Apps.agInit(), 500);
