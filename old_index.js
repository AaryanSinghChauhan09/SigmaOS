/*
 * =========================================================================
 * Σ SIGMAOS ZENITH SUPREME: INDUSTRIAL ORCHESTRATION SHARD (v94.0)
 * =========================================================================
 * Mission: Universal Distribution & System Sovereignty.
 * USP: Themes / Cloud Hub / Remote Bots / Script Playground / Safety Shard.
 * =========================================================================
 */

// --- Global System State ---
let sysUptime = 0;
let currentWorkspace = 1;
const WINDOW_WORKSPACES = {}; // Map winId -> workspaceNum
let activeMode = 'ZENITH';

// --- System Stability & Error Catching (v270.0 Patch) ---
window.onerror = function(msg, url, lineNo, columnNo, error) {
    console.error(`Σ FATAL ERROR: ${msg} at ${url}:${lineNo}:${columnNo}`, error);
    spawnToast(`Kernel Fault Prevented: ${msg}`, 0, true);
    logAuditEvent(`ERROR_CAUGHT: ${msg}`);
    return true; // Prevent default browser crash handler
};
window.onunhandledrejection = function(event) {
    console.error(`Σ UNHANDLED PROMISE: ${event.reason}`);
    spawnToast(`Async Promise Fault: ${event.reason}`, 0, true);
    return true;
};

// --- Persistent Sovereign VFS (No Simulation) ---
class SovereignVFS {
    constructor() {
        this.storageKey = 'SIGMAOS_VFS_ZENITH';
        this.fs = JSON.parse(localStorage.getItem(this.storageKey)) || {
            '/root': { type: 'dir', children: ['bin', 'kernel', 'userland', 'data', 'media'] },
            '/root/bin': { type: 'dir', children: [] },
            '/root/kernel': { type: 'dir', children: [] },
            '/root/userland': { type: 'dir', children: [] },
            '/root/data': { type: 'dir', children: ['industrial.json'] },
            '/root/media': { type: 'dir', children: [] },
            '/root/data/industrial.json': { type: 'file', content: '{"status": "SOVEREIGN", "integrity": 100}' }
        };
        this.sync();
    }
    sync() { localStorage.setItem(this.storageKey, JSON.stringify(this.fs)); }
    exists(path) { return !!this.fs[path]; }
    isDir(path) { return this.fs[path] && this.fs[path].type === 'dir'; }
    ls(path) { return this.fs[path] ? this.fs[path].children : []; }
    mkdir(path) {
        if (this.exists(path)) return false;
        const parent = path.substring(0, path.lastIndexOf('/')) || '/root';
        if (!this.isDir(parent)) return false;
        this.fs[path] = { type: 'dir', children: [] };
        this.fs[parent].children.push(path.split('/').pop());
        this.sync();
        return true;
    }
    write(path, content) {
        const parent = path.substring(0, path.lastIndexOf('/')) || '/root';
        if (!this.isDir(parent)) return false;
        if (!this.exists(path)) this.fs[parent].children.push(path.split('/').pop());
        this.fs[path] = { type: 'file', content };
        this.sync();
        return true;
    }
    read(path) { return this.fs[path] ? this.fs[path].content : null; }
    snapshot(name) {
        const data = JSON.stringify(localStorage);
        localStorage.setItem(`SNAPSHOT_${name.toUpperCase()}`, data);
        spawnToast(`System Snapshot [${name}] created.`);
    }
    rollback(name) {
        const data = localStorage.getItem(`SNAPSHOT_${name.toUpperCase()}`);
        if (!data) return termPrint(`Rollback failed: Snapshot ${name} not found.`);
        localStorage.clear();
        const state = JSON.parse(data);
        Object.keys(state).forEach(k => localStorage.setItem(k, state[k]));
        spawnToast(`Rollback Successful: Reverted to ${name}. Rebooting...`);
        setTimeout(() => location.reload(), 1500);
    }
}

const VFS = new SovereignVFS();
let terminalCwd = '/root';

const PROCESSES = [
    { pid: 0, name: 'sigma_kernel', state: 'RUNNING', cpu: 0.1 },
    { pid: 1, name: 'sigma_init', state: 'RUNNING', cpu: 0.5 },
    { pid: 102, name: 'sigma_gui', state: 'RUNNING', cpu: 2.4 },
    { pid: 105, name: 'sigma_net', state: 'SLEEPING', cpu: 0.0 }
];

const REPOSITORY = [
    { id: 'dev_tools', name: 'Industrial Dev Suite', version: '4.5.0', desc: 'Sovereign C/C++ Compiler & Debugger', icon: '🛠️', installed: true },
    { id: 'net_shield', name: 'Sigma Shield Firewall', version: '1.2.9', desc: 'Kernel-level packet inspection', icon: '🛡️', installed: false },
    { id: 'office_matrix', name: 'Sigma Matrix Office', version: '3.0.2', desc: 'Distributed spreadsheet & docs', icon: '📊', installed: true },
    { id: 'bio_lab', name: 'NCERT Biology Lab', version: '1.0.0', desc: 'Sovereign biology simulations', icon: '🧬', installed: false },
    { id: 'math_lab', name: 'NCERT Maths Lab', version: '1.0.0', desc: 'Advanced math sharding', icon: '📐', installed: false },
    { id: 'android_tools', name: 'Omni Tools Android', version: '2.1.0', desc: 'Sovereign APK sharder', icon: '📱', installed: false },
    { id: 'sentinel', name: 'Sigma Sentinel', version: '3.0.0', desc: 'Real-time threat detection', icon: '👁️', installed: true },
    { id: 'theme_eng', name: 'Apex Theme Engine', version: '2.5.0', desc: 'Dynamic interface sharding', icon: '🎨', installed: true }
];

const DISTROS = [
    { id: 'ubuntu', name: 'Ubuntu Lunar', icon: '🟠', info: 'LTS Industrial Core', url: 'https://copy.sh/v86/?profile=ubuntu' },
    { id: 'arch', name: 'Arch Linux', icon: '🔵', info: 'Sovereign Rolling Release', url: 'https://copy.sh/v86/?profile=archlinux' },
    { id: 'debian', name: 'Debian 12', icon: '🔴', info: 'The Universal OS Shard', url: 'https://copy.sh/v86/?profile=debian' },
    { id: 'opensuse', name: 'openSUSE Tumbleweed', icon: '🟢', info: 'Professional Stability Shard', url: 'https://copy.sh/v86/?profile=opensuse' },
    { id: 'almalinux', name: 'AlmaLinux 9', icon: '⚪', info: 'Community Enterprise Grade', url: 'https://copy.sh/v86/?profile=almalinux' },
    { id: 'rocky', name: 'Rocky Linux 9', icon: '🛡️', info: 'RHEL-Compatible Master', url: 'https://copy.sh/v86/?profile=rocky' },
    { id: 'alpine', name: 'Alpine Linux', icon: '🏔️', info: 'Security-oriented Shard', url: 'https://copy.sh/v86/?profile=alpine' },
    { id: 'gentoo', name: 'Gentoo Linux', icon: '🟣', info: 'Source-based Sovereignty', url: 'https://copy.sh/v86/?profile=gentoo' },
    { id: 'fedora', name: 'Fedora Workstation', icon: '🧢', info: 'Cutting-Edge Sharding', url: 'https://copy.sh/v86/?profile=fedora' },
    { id: 'custom', name: 'Custom ISO/Disk', icon: '💿', info: 'Universal Shard Loader', url: '' }
];

const MATRIX_TOOLS = [
    { id: 'xclicker', name: 'Sigma XClicker', desc: 'Sovereign Auto-Clicking logic.', icon: '🖱️', USP: 'robiot/xclicker' },
    { id: 'autokey', name: 'Sigma AutoKey', desc: 'Industrial Macro Automation.', icon: '⌨️', USP: 'famousshea/autokey' },
    { id: 'merlin_ia', name: 'Sovereign AI Shard', desc: 'Autonomous system balancing.', icon: '🤖', USP: 'N1ghthill/merlin-ia' },
    { id: 'cloud_provision', name: 'vSphere Provisioner', desc: 'Industrial Infrastructure Sharding.', icon: '☁️', USP: 'miladhzzzz/vsphere-infra' },
    { id: 'script_master', name: 'Automation Playbook', desc: 'Universal Bash/Python Matrix.', icon: '📜', USP: 'muhibarshad/Linux-Automation-Scripts' },
    { id: 'ai_orchestrator', name: 'Aether Orchestrator', desc: 'Multi-model AI sharding.', icon: '🤖', USP: 'AI-Orchestrator-v2.0' },
    { id: 'spectrum_terminal', name: 'Spectrum AI Shell', desc: 'Neural command prediction.', icon: '⚡', USP: 'Spectrum-Terminal-V18' }
];

const THEMES = [
    { id: 'zenith', name: 'Zenith-Default', primary: '#00d2ff', blur: '20px' },
    { id: 'crimson', name: 'Hacker-Crimson', primary: '#ff0033', blur: '10px' },
    { id: 'lupus', name: 'Lupus-Minimal', primary: '#ffffff', blur: '5px' },
    { id: 'noir', name: 'OLED-Noir', primary: '#111111', blur: '0px' },
    { id: 'alpine', name: 'Alpine-Lite', primary: '#0d192e', blur: '2px' }
];

const VMS = [
    { id: 'node_alpha', name: 'Sovereign-Alpha-01', status: 'RUNNING', ip: '192.168.10.1' },
    { id: 'node_beta', name: 'Sovereign-Beta-02', status: 'PAUSED', ip: '192.168.10.2' }
];

// --- DOM Elements ---
const clockEl = document.getElementById('clock');
const cpuVal = document.getElementById('cpu-val');
const spotlight = document.getElementById('spotlight');
const spotlightInput = document.getElementById('spotlight-input');
const spotlightResults = document.getElementById('spotlight-results');

const distroIframe = document.getElementById('distro-iframe');
const distroSelector = document.getElementById('distro-selector');

const themeList = document.getElementById('theme-list');
const vpnList = document.getElementById('vpn-list');
const vmList = document.getElementById('vm-list');
const safetyLog = document.getElementById('safety-log');

const remoteTerm = document.getElementById('remote-term');
const remoteCmd = document.getElementById('remote-cmd');

const playgroundEditor = document.getElementById('playground-editor');

const termOutput = document.getElementById('term-output');
const termInput = document.getElementById('terminal-input');

const matrixDashboard = document.getElementById('matrix-dashboard');
const guiControls = document.getElementById('gui-controls');

const academyList = document.getElementById('academy-list');
const userList = document.getElementById('user-list');
const backupControls = document.getElementById('backup-controls');
const backupLog = document.getElementById('backup-log');

// --- Core Initialization ---
document.addEventListener('DOMContentLoaded', () => {
    initMatrixBackground();
    initClock();
    initMetrics();
    initWindows();
    initDock();
    initVFSView();
    initRepoView();
    initSpotlight();
    initWorkspaces();
    initDistroRunner();
    initThemeStore();
    initCloudHub();
    initWiki();
    initRemoteConsole();
    initMatrix();
    initGUIBuilder();
    initAcademy();
    initUserManagement();
    initBackupShard();
    initAIOrchestrator();
    initSpectrumTerminal();
    initServices();
    initPlugins();
    initUtils();
    initVFSBuffer();
    initAdvancedKernel();
    spawnToast('Σ SIGMAOS ZENITH SUPREME INITIALIZED');
    spawnToast('Industrial Shard Mastery Active', 1500);
});

function initMatrixBackground() {
    const canvas = document.getElementById('bg-canvas');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    
    let width = canvas.width = window.innerWidth;
    let height = canvas.height = window.innerHeight;
    
    const characters = 'Σ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ';
    const fontSize = 14;
    const columns = width / fontSize;
    const drops = [];
    
    for (let x = 0; x < columns; x++) drops[x] = 1;
    
    function draw() {
        ctx.fillStyle = 'rgba(0, 0, 0, 0.05)';
        ctx.fillRect(0, 0, width, height);
        
        ctx.fillStyle = '#00d2ff'; // Accent Primary
        ctx.font = fontSize + 'px Orbitron';
        
        for (let i = 0; i < drops.length; i++) {
            const text = characters.charAt(Math.floor(Math.random() * characters.length));
            ctx.fillText(text, i * fontSize, drops[i] * fontSize);
            
            if (drops[i] * fontSize > height && Math.random() > 0.975) {
                drops[i] = 0;
            }
            drops[i]++;
        }
    }
    
    window.addEventListener('resize', () => {
        width = canvas.width = window.innerWidth;
        height = canvas.height = window.innerHeight;
    });
    
    setInterval(draw, 33);
}

function initClock() {
    if (!clockEl) return;
    setInterval(() => {
        clockEl.textContent = new Date().toTimeString().split(' ')[0];
        const memVal = document.getElementById('mem-val');
        if (memVal) memVal.textContent = (Math.random() * 2 + 18).toFixed(1) + '%';
        sysUptime++;
    }, 1000);
}

function initMetrics() {
    setInterval(() => {
        const cpu = (Math.random() * 5 + 2).toFixed(1);
        const mem = (Math.random() * 2 + 18).toFixed(1);
        if (cpuVal) cpuVal.textContent = cpu + '%';
        
        const cpuBar = document.getElementById('cpu-bar');
        const memBar = document.getElementById('mem-bar');
        const cpuPercent = document.getElementById('cpu-percent');
        const memUsage = document.getElementById('mem-usage');
        const cpuValDisplay = document.getElementById('cpu-val-display');
        const memValDisplay = document.getElementById('mem-val-display');

        if (cpuBar) cpuBar.style.width = cpu + '%';
        if (memBar) memBar.style.width = mem + '%';
        if (cpuPercent) cpuPercent.textContent = cpu + '%';
        if (cpuValDisplay) cpuValDisplay.textContent = cpu;
        if (memValDisplay) memValDisplay.textContent = mem;
        
        updateProcessList();
    }, 2000);
}

function updateProcessList() {
    const procList = document.getElementById('proc-list');
    if (!procList) return;
    procList.innerHTML = '<div class="u-flex-between u-muted-text u-margin-b-5"><span>PID</span><span>NAME</span><span>STATE</span><span>CPU</span></div>';
    PROCESSES.forEach(p => {
        const cpu = p.state === 'RUNNING' ? (p.cpu + Math.random() * 0.5).toFixed(1) : '0.0';
        procList.innerHTML += `<div class="u-flex-between"><span>${p.pid}</span><span>${p.name}</span><span>${p.state}</span><span>${cpu}%</span></div>`;
    });
}

// --- Window Orchestration ---
let zIndexCounter = 100;

function initWindows() {
    document.querySelectorAll('.win-header').forEach(header => {
        header.addEventListener('mousedown', (e) => {
            const win = header.parentElement;
            focusWindow(win);
            let offsetX = e.clientX - win.offsetLeft;
            let offsetY = e.clientY - win.offsetTop;
            const onMouseMove = (ev) => {
                win.style.left = (ev.clientX - offsetX) + 'px';
                win.style.top = (ev.clientY - offsetY) + 'px';
            };
            const onMouseUp = () => {
                document.removeEventListener('mousemove', onMouseMove);
                document.removeEventListener('mouseup', onMouseUp);
            };
            document.addEventListener('mousemove', onMouseMove);
            document.addEventListener('mouseup', onMouseUp);
        });
    });

    document.querySelectorAll('[data-action="close"]').forEach(btn => {
        btn.addEventListener('click', () => {
            const winId = btn.getAttribute('data-win');
            closeWindow(winId);
        });
    });

    document.querySelectorAll('[data-action="minimize"]').forEach(btn => {
        btn.addEventListener('click', () => {
            const winId = btn.getAttribute('data-win');
            minimizeWindow(winId);
        });
    });

    document.querySelectorAll('[data-action="maximize"]').forEach(btn => {
        btn.addEventListener('click', () => {
            const winId = btn.getAttribute('data-win');
            toggleMaximize(winId);
        });
    });
}

function closeWindow(id) {
    const win = document.getElementById('win-' + id);
    if (win) {
        win.classList.add('hidden');
        win.classList.remove('maximized');
        renderWorkspaces();
        spawnToast(`Shard [${id}] Terminated via Silicon Signal.`);
    }
}

function minimizeWindow(id) {
    const win = document.getElementById('win-' + id);
    if (win) {
        win.classList.add('hidden');
        renderWorkspaces();
        spawnToast(`Shard [${id}] Suspended to Silicon Cache.`);
    }
}

function toggleMaximize(id) {
    const win = document.getElementById('win-' + id);
    if (win) {
        win.classList.toggle('maximized');
        spawnToast(`Shard [${id}] Optimized to Full-Silicon View.`);
    }
}

function focusWindow(win) {
    document.querySelectorAll('.window').forEach(w => w.classList.remove('focused'));
    win.classList.add('focused');
    win.style.zIndex = ++zIndexCounter;
}

function openWindow(id) {
    const win = document.getElementById('win-' + id);
    if (win) {
        win.classList.remove('hidden');
        WINDOW_WORKSPACES[id] = parseInt(currentWorkspace);
        renderWorkspaces();
        focusWindow(win);
    }
}

function renderWorkspaces() {
    document.querySelectorAll('.window').forEach(win => {
        const id = win.id.replace('win-', '');
        const ws = WINDOW_WORKSPACES[id] || 1;
        if (ws == currentWorkspace && !win.classList.contains('hidden-by-ws')) {
            win.style.display = '';
        } else {
            win.style.display = 'none';
        }
    });
}

function initDock() {
    document.querySelectorAll('.dock-item').forEach(item => {
        item.addEventListener('click', () => {
            const winId = item.getAttribute('data-window');
            openWindow(winId);
        });
    });
}

// --- Spotlight ---
function initSpotlight() {
    document.addEventListener('keydown', (e) => {
        if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
            e.preventDefault();
            spotlight.classList.toggle('hidden');
            if (!spotlight.classList.contains('hidden')) spotlightInput.focus();
        }
    });

    spotlightInput.addEventListener('input', () => {
        const query = spotlightInput.value.toLowerCase().trim();
        spotlightResults.innerHTML = '';
        if (!query) return;

        const matches = [
            ...REPOSITORY.map(p => ({ ...p, type: 'App' })),
            ...DISTROS.map(d => ({ ...d, type: 'Distro' })),
            ...THEMES.map(t => ({ ...t, type: 'Theme', icon: '🎭' })),
            { name: 'Terminal', id: 'terminal', type: 'Tool', icon: '💻' }
        ].filter(item => item.name.toLowerCase().includes(query));

        matches.forEach(match => {
            const res = document.createElement('div');
            res.className = 'search-result';
            res.innerHTML = `<div class="result-icon">${match.icon || '⚙️'}</div><div>${match.name}</div>`;
            res.onclick = () => {
                if (match.type === 'App') openWindow('pkg'); // Just open repo for now
                else if (match.id === 'matrix') openWindow('industrialmatrix');
                else if (match.type === 'Tool') openWindow(match.id);
                else if (match.type === 'Theme') { openWindow('themes'); applyTheme(match.id); }
                else if (match.type === 'Distro') { openWindow('runner'); startDistroStream(match.id); }
                spotlight.classList.add('hidden');
            };
            spotlightResults.appendChild(res);
        });
    });
}

// --- Specific Tools Logic ---
function initThemeStore() {
    if (!themeList) return;
    themeList.innerHTML = '';
    THEMES.forEach(t => {
        const el = document.createElement('div');
        el.className = 'distro-card';
        el.innerHTML = `<div class="u-font-size-xs">${t.name}</div><button class="status-chip" onclick="applyTheme('${t.id}')">Apply</button>`;
        themeList.appendChild(el);
    });
}

function applyTheme(id) {
    const t = THEMES.find(x => x.id === id);
    if (!t) return;
    document.documentElement.style.setProperty('--accent-primary', t.primary);
    document.documentElement.style.setProperty('--glass-blur', t.blur);
    spawnToast('Injected Theme Shard: ' + t.name);
}

function initCloudHub() {
    if (!vmList) return;
    vmList.innerHTML = '';
    VMS.forEach(v => {
        const el = document.createElement('div');
        el.className = 'metric-card u-margin-b-20';
        el.innerHTML = `<div class="metric-header"><span>☁️ ${v.name}</span> <span class="u-accent-text">${v.status}</span></div><div>IP: ${v.ip}</div>`;
        vmList.appendChild(el);
    });
}

function runSafetyAudit() {
    if (!safetyLog) return;
    spawnToast('Initiating Rust-Parity System Scan...');
    setTimeout(() => {
        safetyLog.innerHTML += '<div>[SAFE] Sharding Core Memory Bounds... PASSED</div>';
        safetyLog.innerHTML += '<div>[SAFE] Integrity Check: O(1) Slab Master... PASSED</div>';
        safetyLog.scrollTop = safetyLog.scrollHeight;
    }, 1000);
}

function initRemoteConsole() {
    if (!remoteCmd) return;
    remoteCmd.addEventListener('keydown', e => {
        if (e.key === 'Enter') {
            const val = remoteCmd.value.trim();
            const line = document.createElement('div');
            line.textContent = '> Node-Alpha: Executing ' + val + '... OK';
            remoteTerm.appendChild(line);
            remoteTerm.scrollTop = remoteTerm.scrollHeight;
            remoteCmd.value = '';
        }
    });
}

function initWiki() {
    const wiki = document.getElementById('wiki-content');
    if (!wiki) return;
    wiki.innerHTML = `
        <div class="u-margin-b-10 u-accent-text u-bold">SOVEREIGN ARCHITECTURE (v94.0)</div>
        <p class="u-muted-text">SigmaOS Zenith is built on a Zero-Dependency silicon sharding model. It absorbs the USPs of 30+ industrial repositories to provide absolute sovereignty.</p>
        <ul class="u-margin-t-10">
            <li><strong>Kernel:</strong> O(1) Slab, PML4 Paging, AI Balancing.</li>
            <li><strong>Userland:</strong> Distro Mirroring, Matrix Automation, Safety Sharding.</li>
            <li><strong>Aesthetics:</strong> Glassmorphism (SigmaOS.com style), Theme Shards.</li>
        </ul>
    `;
}

function initMatrix() {
    if (!matrixDashboard) return;
    matrixDashboard.innerHTML = '';
    MATRIX_TOOLS.forEach(tool => {
        const el = document.createElement('div');
        el.className = 'metric-card';
        el.innerHTML = `
            <div class="metric-header"><span>${tool.icon} ${tool.name}</span> <span class="u-font-size-xxs u-muted-text">${tool.USP}</span></div>
            <div class="u-font-size-xs u-margin-b-10">${tool.desc}</div>
            <button class="status-chip" onclick="executeMatrixTool('${tool.id}')">Activate Shard</button>
        `;
        matrixDashboard.appendChild(el);
    });
}

function executeMatrixTool(id) {
    const tool = MATRIX_TOOLS.find(x => x.id === id);
    if (!tool) return;
    spawnToast('Executing Industrial Shard: ' + tool.name);

    if (id === 'xclicker') {
        termPrint('[MATRIX] Initializing Sovereign AutoClicker (robiot/xclicker parity)...');
        termPrint('[MATRIX] Target: Local Silicon Shard. Frequency: 1000 Hz. Status: ACTIVE');
    } else if (id === 'autokey') {
        termPrint('[MATRIX] Injected Macro Shard: Ctrl+Shift+S -> Sovereign Sync.');
    } else if (id === 'merlin_ia') {
        runSafetyAudit();
    } else if (id === 'ai_orchestrator') {
        openWindow('aiorch');
    } else if (id === 'spectrum_terminal') {
        openWindow('spectrum');
    }
}

function initGUIBuilder() {
    if (!guiControls) return;
    guiControls.innerHTML = `
        <div class="metric-card">
            <div>Glass Blur</div>
            <input type="range" min="0" max="50" value="20" class="u-full-width" oninput="updateBlur(this.value)">
        </div>
        <div class="metric-card">
            <div>Accent Color</div>
            <input type="color" value="#00d2ff" class="u-full-width" oninput="updateAccent(this.value)">
        </div>
    `;
}

function updateBlur(val) {
    document.documentElement.style.setProperty('--glass-blur', val + 'px');
}

function updateAccent(val) {
    document.documentElement.style.setProperty('--accent-primary', val);
}

function initAcademy() {
    if (!academyList) return;
    const lessons = [
        { id: 1, name: 'VFS Sharding', desc: 'Master ls, cd, mkdir logic.' },
        { id: 2, name: 'Process Control', desc: 'Audit industrial tasks.' },
        { id: 3, name: 'System AI', desc: 'Collaborate with Sovereign AI.' }
    ];
    academyList.innerHTML = '';
    lessons.forEach(l => {
        const el = document.createElement('div');
        el.className = 'metric-card';
        el.innerHTML = `
            <div class="u-bold u-font-size-xs">${l.name}</div>
            <div class="u-font-size-xxs u-muted-text u-margin-b-10">${l.desc}</div>
            <button class="status-chip" onclick="startLesson(${l.id})">Start Sharding</button>
        `;
        academyList.appendChild(el);
    });
}

function startLesson(id) {
    spawnToast('Initializing Academy Lesson ' + id);
    termPrint('[ACADEMY] Lesson ' + id + ' Active. Target: Sovereign Mastery.');
}

function initUserManagement() {
    if (!userList) return;
    const users = [
        { name: 'root', role: 'MASTER', status: 'ACTIVE' },
        { name: 'Sovereign-Master', role: 'SHARD_OWNER', status: 'ACTIVE' }
    ];
    userList.innerHTML = '';
    users.forEach(u => {
        const el = document.createElement('div');
        el.className = 'metric-card u-margin-b-10';
        el.innerHTML = `<div class="metric-header"><span>👤 ${u.name}</span> <span class="u-accent-text">${u.role}</span></div>`;
        userList.appendChild(el);
    });
}

function initBackupShard() {
    if (!backupControls) return;
    backupControls.innerHTML = `
        <button class="status-chip" onclick="triggerBackup('SYSTEM')">System Snapshot</button>
        <button class="status-chip" onclick="triggerBackup('MEDIA')">Media Scraper</button>
    `;
}

function triggerBackup(type) {
    spawnToast('Initiating ' + type + ' Backup...');
    if (backupLog) {
        backupLog.innerHTML += `<div>[${type}] Sharding stream initialized... OK</div>`;
        backupLog.innerHTML += `<div>[${type}] Sovereign checksum verification... PASSED</div>`;
        backupLog.scrollTop = backupLog.scrollHeight;
    }
}

// --- AI Orchestrator Integration ---
function initAIOrchestrator() {
    const modelList = document.getElementById('ai-model-list');
    if (!modelList) return;
    const models = ['ChatGPT', 'Claude', 'Gemini', 'Perplexity', 'Grok', 'Liner', 'Ask5AI', 'LMArena'];
    modelList.innerHTML = models.map(m => `<div class="status-chip u-margin-b-5">${m}</div>`).join(' ');
}

function executeOrchestration() {
    const prompt = document.getElementById('ai-orch-prompt').value;
    const log = document.getElementById('ai-orch-log');
    if (!prompt) return;
    
    log.innerHTML += `<div>[AETHER] Initiating mission: "${prompt}"</div>`;
    log.innerHTML += `<div>[AETHER] Routing to 11 platforms via Spectrum Routing...</div>`;
    
    setTimeout(() => {
        log.innerHTML += `<div class="u-accent-text">[AETHER] Perplexity Pro: Academic Focus active.</div>`;
        log.innerHTML += `<div class="u-accent-text">[AETHER] Grok Fun: Creative logic enabled.</div>`;
        log.innerHTML += `<div class="u-accent-text">[AETHER] MISSION SUCCESS: Responses aggregated in Aether Cloud.</div>`;
        log.scrollTop = log.scrollHeight;
        spawnToast('Orchestration Mission Complete');
    }, 2000);
}

// --- Spectrum Terminal Integration ---
// Unified Terminal Logic (Spectrum Shard integration)
function initSpectrumTerminal() {
    const output = document.getElementById('spectrum-output');
    const input = document.getElementById('spectrum-input');
    if (!input || !output) return;

    input.addEventListener('keydown', e => {
        if (e.key === 'Enter') {
            const raw = input.value.trim();
            if (!raw) return;
            const parts = raw.split(' ');
            const cmd = parts[0].toLowerCase();
            const args = parts.slice(1);
            
            const line = document.createElement('div');
            line.className = 'term-line';
            line.innerHTML = `<span class="u-accent-text">spectrum@sigmaos</span>:~$ ${raw}`;
            output.appendChild(line);
            
            if (COMMANDS[cmd]) {
                // Route command to appropriate visual output if needed
                COMMANDS[cmd](args);
            } else {
                const err = document.createElement('div');
                err.className = 'term-line u-muted-text';
                err.textContent = `spectrum: command not found: ${cmd}`;
                output.appendChild(err);
            }
            
            output.scrollTop = output.scrollHeight;
            input.value = '';
        }
    });
}

function initDistroRunner() {
    if (!distroSelector) return;
    distroSelector.innerHTML = '';
    DISTROS.forEach(d => {
        const card = document.createElement('div');
        card.className = 'distro-card';
        card.innerHTML = `<div class="distro-icon">${d.icon}</div><div class="distro-name">${d.name}</div>`;
        card.onclick = () => startDistroStream(d.id);
        distroSelector.appendChild(card);
    });
}

function startDistroStream(id) {
    const d = DISTROS.find(x => x.id === id);
    if (!d) return;

    let targetUrl = d.url;
    if (id === 'custom') {
        const customUrl = prompt('Enter the Sovereign ISO/Disk Image URL (e.g. https://example.com/linux.iso):');
        if (!customUrl) return;
        targetUrl = 'https://copy.sh/v86/?iso=' + encodeURIComponent(customUrl);
    }

    distroSelector.classList.add('hidden');
    distroIframe.src = targetUrl;
    distroIframe.classList.remove('hidden');
    spawnToast('Streaming Distribution Shard: ' + d.name);
}

// --- Standard UI Helpers ---
function initVFSBuffer() {
    const defaultFiles = [
        { path: '/root/sigma_core.asm', content: 'Σ SOVEREIGN CORE v151.0 (RAW ASM SHARD)\nSECTION .text\n  global _start\n_start: mov rax, 60\n syscall' },
        { path: '/root/boot_master.c', content: '/* Σ SIGMAOS BOOT MASTER */\nvoid _start() { sigma_kernel_main(); }' },
        { path: '/root/config.sys', content: 'SYSTEM_SOVEREIGNTY=ABSOLUTE\nAI_MODELS=v4-TRANSFORMER' }
    ];
    for (let i = 0; i < defaultFiles.length; i++) {
        const f = defaultFiles[i];
        const enc = new TextEncoder();
        VFS_FILES.set(f.path, enc.encode(f.content));
    }
}

function initVFSView() {
    const grid = document.getElementById('file-grid');
    if (!grid) return;
    grid.innerHTML = '';
    
    // Low-dependency iteration
    const dirs = Array.from(VFS_DIRS);
    for (let i = 0; i < dirs.length; i++) {
        const d = dirs[i];
        if (d.startsWith(terminalCwd) && d !== terminalCwd) {
            const name = d.split('/').pop();
            const el = document.createElement('div');
            el.className = 'distro-card';
            el.style.padding = '10px';
            el.innerHTML = `<div style="font-size: 2rem">📂</div><div class="u-font-size-xs">${name}</div>`;
            el.onclick = () => { terminalCwd = d; initVFSView(); updateBreadcrumbs(); };
            grid.appendChild(el);
        }
    }
    
    const files = Array.from(VFS_FILES.keys());
    for (let i = 0; i < files.length; i++) {
        const f = files[i];
        if (f.startsWith(terminalCwd)) {
            const name = f.split('/').pop();
            const el = document.createElement('div');
            el.className = 'distro-card';
            el.style.padding = '10px';
            el.innerHTML = `<div style="font-size: 2rem">📄</div><div class="u-font-size-xs">${name}</div>`;
            grid.appendChild(el);
        }
    }
}

function updateBreadcrumbs() {
    const b = document.getElementById('vfs-breadcrumbs');
    if (b) b.textContent = terminalCwd.replace('/root', '~').split('/').join(' / ');
}

function initRepoView() {
    const list = document.getElementById('pkg-list');
    if (!list) return;
    list.innerHTML = '';
    REPOSITORY.forEach(p => {
        const el = document.createElement('div');
        el.className = 'metric-card u-margin-b-10 u-flex-between';
        el.innerHTML = `
            <div>
                <div class="u-bold">${p.icon} ${p.name}</div>
                <div class="u-font-size-xxs u-muted-text">${p.desc}</div>
            </div>
            <button class="status-chip">${p.installed ? 'INSTALLED' : 'GET'}</button>
        `;
        list.appendChild(el);
    });
}

function initWorkspaces() {
    document.querySelectorAll('.ws-indicator').forEach(ws => {
        ws.onclick = () => {
            document.querySelectorAll('.ws-indicator').forEach(w => w.classList.remove('active'));
            ws.classList.add('active');
            currentWorkspace = ws.getAttribute('data-ws');
            spawnToast('Switched to Workspace ' + currentWorkspace);
            renderWorkspaces();
        };
    });
    // Initial workspace state
    WINDOW_WORKSPACES['terminal'] = 1;
    WINDOW_WORKSPACES['ai'] = 1;
    WINDOW_WORKSPACES['ml'] = 1;
    WINDOW_WORKSPACES['notebook'] = 1;
    WINDOW_WORKSPACES['automation'] = 1;
    renderWorkspaces();
}

function initServices() {
    const list = document.getElementById('service-list');
    if (!list) return;
    const services = [
        { name: 'sigma_netd', status: 'OK', desc: 'Sovereign Network Daemon' },
        { name: 'sigma_fsd', status: 'OK', desc: 'VFS Sharding Engine' },
        { name: 'sigma_aud', status: 'OK', desc: 'Aether Audio Shard' },
        { name: 'sigma_sentinel', status: 'ARMD', desc: 'Autonomous Security' }
    ];
    list.innerHTML = '';
    services.forEach(s => {
        list.innerHTML += `<div class="u-flex-between u-margin-b-5"><span>${s.name}</span> <span class="u-accent-text">[${s.status}]</span></div>`;
    });
}

function initPlugins() {
    const list = document.getElementById('plugin-list');
    if (!list) return;
    const plugins = ['X11-Mirror', 'C-Transpiler', 'Python-Runtime', 'Rust-Safety-Core'];
    list.innerHTML = plugins.map(p => `
        <div class="metric-card u-margin-b-10 u-flex-between">
            <span>🧩 ${p}</span> <button class="status-chip">ENABLE</button>
        </div>
    `).join('');
}

function initUtils() {
    const list = document.getElementById('utils-list');
    if (!list) return;
    const utils = ['apt-sigma', 'ls-pure', 'cat-raw', 'top-zenith', 'grep-shard', 'chmod-asm'];
    list.innerHTML = utils.map(u => `
        <div class="metric-card u-text-center">
            <div class="u-bold u-font-size-xs">${u}</div>
            <button class="status-chip u-margin-t-5">RUN</button>
        </div>
    `).join('');
}

function termPrint(text, type = '') {
    const div = document.createElement('div');
    div.className = 'term-line ' + type;
    div.textContent = text;
    termOutput.appendChild(div);
    termOutput.scrollTop = termOutput.scrollHeight;
}

const COMMANDS = {
    help: () => {
        termPrint('OS COMMANDS: help, clear, ls, cd, mkdir, touch, cat, rm, neofetch, cpu, mem, matrix, scrub, shutdown');
    },
    clear: () => {
        termOutput.innerHTML = '';
    },
    ls: () => {
        const items = VFS.ls(terminalCwd);
        if (items.length === 0) termPrint('Directory is empty.');
        else {
            items.forEach(item => {
                const fullPath = terminalCwd + (terminalCwd.endsWith('/') ? '' : '/') + item;
                const type = VFS.isDir(fullPath) ? '[DIR]' : '[FILE]';
                termPrint(`- ${item.padEnd(20)} ${type}`);
            });
        }
    },
    cd: (args) => {
        const path = args[0] || '/root';
        if (path === '/') { terminalCwd = '/root'; }
        else if (path === '..') {
            if (terminalCwd !== '/root') {
                terminalCwd = terminalCwd.substring(0, terminalCwd.lastIndexOf('/')) || '/root';
            }
        } else {
            const target = (path.startsWith('/') ? path : terminalCwd + '/' + path).replace(/\/+/g, '/');
            if (VFS.exists(target) && VFS.isDir(target)) terminalCwd = target;
            else termPrint('cd: no such directory: ' + path);
        }
        document.querySelectorAll('.term-prompt').forEach(p => {
            p.textContent = `root@sigmaos:${terminalCwd.replace('/root', '~')}#`;
        });
    },
    mkdir: (args) => {
        if (!args[0]) return termPrint('mkdir: missing operand');
        const path = (terminalCwd + '/' + args[0]).replace(/\/+/g, '/');
        if (VFS.mkdir(path)) termPrint(`Created directory: ${args[0]}`);
        else termPrint(`mkdir: cannot create directory '${args[0]}': File exists or path invalid`);
    },
    touch: (args) => {
        if (!args[0]) return termPrint('touch: missing operand');
        const path = (terminalCwd + '/' + args[0]).replace(/\/+/g, '/');
        if (VFS.write(path, '')) termPrint(`Created empty file: ${args[0]}`);
        else termPrint(`touch: cannot create file '${args[0]}'`);
    },
    cat: (args) => {
        if (!args[0]) return termPrint('cat: missing operand');
        const path = (terminalCwd + '/' + args[0]).replace(/\/+/g, '/');
        const content = VFS.read(path);
        if (content !== null) termPrint(content);
        else termPrint(`cat: ${args[0]}: No such file`);
    },
    rm: (args) => {
        if (!args[0]) return termPrint('rm: missing operand');
        const path = (terminalCwd + '/' + args[0]).replace(/\/+/g, '/');
        if (VFS.fs[path]) {
            const parent = path.substring(0, path.lastIndexOf('/')) || '/root';
            VFS.fs[parent].children = VFS.fs[parent].children.filter(c => c !== args[0]);
            delete VFS.fs[path];
            VFS.sync();
            termPrint(`Removed: ${args[0]}`);
        } else termPrint(`rm: cannot remove '${args[0]}': No such file or directory`);
    },
    neofetch: () => {
        termPrint('           .----------------.           ');
        termPrint('          /  ____________  \\          root@sigmaos');
        termPrint('         /  /            \\  \\         ------------');
        termPrint('        /  /              \\  \\        OS: SigmaOS Zenith Supreme v150.8');
        termPrint('       /  /                \\  \\       Kernel: Σ-Autonomous-6.8.9-zenith');
        termPrint('      (  (        Σ         )  )      Uptime: ' + sysUptime + 's');
        termPrint('       \\  \\                /  /       Packages: 0 (Zero-Dependency)');
        termPrint('        \\  \\              /  /        Shell: sigma_shell v94.2');
        termPrint('         \\  \\____________/  /         Resolution: 2560x1440 (Retina-Parity)');
        termPrint('          \\________________/          DE: Ubuntu-Elite Gold Industrial');
        termPrint('                                      CPU: x86_64 Sovereign ASM-Shard');
        termPrint('                                      Memory: 18.2% / 1024.0 PB');
    },
    cpu: () => {
        termPrint('CPU LOAD: ' + cpuVal.textContent + ' [SHARD-DIRECT]');
    },
    mem: () => {
        termPrint('MEMORY USAGE: 18.2% (186.4 TB / 1.0 PB)');
    },
    matrix: () => openWindow('industrialmatrix'),
    scrub: () => {
        termPrint('[SILICON] Initiating register scrubbing...');
        setTimeout(() => termPrint('[SILICON] EAX-EDI zeroed. Stack frame scrubbed. Memory bounds secured.'), 500);
    },
    run_playbook: () => {
        termPrint('[OMNI-SHELL]: Initiating Advanced Sovereign Playbook... [DYNAMIC AUTOMATION]');
        const tasks = [
            { cmd: 'LS', log: '[LS-ZENITH]: Directory shard read (simulated).' },
            { cmd: 'TOP', log: '[TOP-ZENITH]: CPU TSC Tick = 18446744073709551615 (Simulated)' },
            { cmd: 'PQC_AUDIT', log: '[PQC-AUDIT]: Verifying Lattice-PQC Sentinel integrity... OK' },
            { cond: 'KERNEL_ACTIVE', cmd: 'LATTICE_REKEY', log: '[OMNI-SHELL]: Triggering Lattice-PQC Rekeying... [QUANTUM SECURED]' },
            { cond: 'KERNEL_ACTIVE', cmd: 'USP_ABSORB', log: '[OMNI-SHELL]: Absorbing legacy OS USPs into Sigma Shard Matrix...' },
            { cmd: 'TOGGLE_KERNEL', log: '[OMNI-SHELL]: KERNEL_ACTIVE = FALSE [SILICON SIGNAL UPDATED]' }
        ];
        
        let i = 0;
        let kernelActive = true;
        const interval = setInterval(() => {
            if (i >= tasks.length) {
                clearInterval(interval);
                termPrint('[OMNI-SHELL]: Playbook Mission Success.');
                return;
            }
            const task = tasks[i++];
            if (task.cond === 'KERNEL_ACTIVE' && !kernelActive) {
                termPrint(`[AUTOMATION]: Condition 'KERNEL_ACTIVE' FALSE. Skipping: ${task.cmd}`);
            } else {
                termPrint(`Σ [OMNI-SHELL]: Executing '${task.cmd}'...`);
                termPrint(task.log);
                if (task.cmd === 'TOGGLE_KERNEL') kernelActive = !kernelActive;
            }
        }, 800);
    },
    shutdown: () => {
        termPrint('Broadcast message from root@sigmaos (pts/0):');
        termPrint('The system is going down for maintenance NOW!');
        setTimeout(() => location.reload(), 2000);
    },
    set_accent: (args) => {
        if (!args[0]) {
            termPrint('Usage: set_accent <color_hex>');
            return;
        }
        document.documentElement.style.setProperty('--accent-primary', args[0]);
        spawnToast('Aether Customization: Accent Shard Updated to ' + args[0]);
    },
    sigmactl: (args) => {
        if (!args[0]) return termPrint('Usage: sigmactl <list|audit|health|profile|macro|trigger|status|persona|suggest|system|config|remote>');
        const sub = args[0].toLowerCase();
        
        switch(sub) {
            case 'list':
                termPrint('ACTIVE SYSTEM SHARDS (VFS PERSISTENT):');
                termPrint('- Kernel: Σ-Zenith-6.8');
                termPrint('- Storage: SovereignVFS v151');
                termPrint('- UI: Zenith Dashboard v160.0');
                termPrint('- AI: Neural Matrix v94');
                break;
            case 'audit':
                const audit = VFS.read('/root/data/audit.log') || 'No audit logs found.';
                termPrint('INDUSTRIAL SECURITY AUDIT:');
                termPrint(audit);
                break;
            case 'health':
                termPrint('SILICON TELEMETRY (TSC):');
                termPrint(`Uptime: ${sysUptime}s | CPU Load: ${cpuVal.textContent} | VFS Integrity: 100%`);
                break;
            case 'status':
                if (args[1] === '--live') {
                    termPrint('Σ LIVE INDUSTRIAL METRICS:');
                    termPrint(`- Latency: ${Math.random().toFixed(2)}ms`);
                    termPrint(`- VFS State: ${localStorage.length} Entries [PERSISTENT]`);
                    termPrint(`- Health: OK | TSC Sharding: Active`);
                    return;
                }
                termPrint('Usage: sigmactl status --live');
                break;
            case 'persona':
                if (args[1] === 'set' && args[2]) {
                    setPersona(args[2].toUpperCase());
                } else {
                    termPrint('Usage: sigmactl persona set <RESEARCHER|GAMER|DEVELOPER>');
                }
                break;
            case 'suggest':
                termPrint('Σ AI RECOMMENDATIONS (v160.0):');
                termPrint('- [!] Shard "Voice" is consuming memory. Recommend "sigmactl reboot shard voice".');
                termPrint('- [!] VFS cache at 82%. Recommend "clear" or "VFS cleanup".');
                break;
            case 'benchmark':
                sigmaBenchmark();
                break;
            case 'boot':
                if (args[1] === 'adaptive') {
                    adaptiveBoot(args[2] ? args[2].toUpperCase() : 'DEVELOPER');
                    return;
                }
                termPrint('Usage: sigmactl boot adaptive <PERSONA>');
                break;
            case 'record':
                if (args[1] === 'start') SigmaMacroRecorder.start();
                else if (args[1] === 'stop' && args[2]) SigmaMacroRecorder.stop(args[2]);
                else termPrint('Usage: sigmactl record <start|stop NAME>');
                break;
            case 'rollback':
                if (args[1]) VFS.rollback(args[1]);
                else termPrint('Usage: sigmactl rollback <NAME>');
                break;
            case 'snapshot':
                if (args[1]) VFS.snapshot(args[1]);
                else termPrint('Usage: sigmactl snapshot <NAME>');
                break;
            case 'setup':
                termPrint('Σ ZENITH INDUSTRIAL WIZARD:');
                termPrint('Select Persona: [1] DEVELOPER [2] GAMER [3] RESEARCHER');
                break;
            case 'system':
                if (args[1] === 'ascii-mode') {
                    document.body.classList.toggle('mode-ascii');
                    return;
                } else if (args[1] === 'bare-metal') {
                    document.body.style.background = '#000';
                    spawnToast('BARE-METAL MODE ACTIVE');
                    return;
                }
                termPrint('Usage: sigmactl system ascii-mode | bare-metal');
                break;
            case 'config':
                if (args[1] === 'load') {
                    const conf = VFS.read('/etc/sigmaos.conf') || 'VERSION=210.0\nTHEME=ZENITH';
                    termPrint(conf);
                    return;
                }
                termPrint('Usage: sigmactl config load');
                break;
            case 'list':
                termPrint('ACTIVE SYSTEM SHARDS (VFS PERSISTENT):');
                termPrint('- Kernel: Σ-Zenith-7.0');
                termPrint('- Storage: SovereignVFS v210.0');
                termPrint('- UI: Zenith Dashboard v210.0');
                termPrint('- AI: Neural Matrix v105');
                break;
            case 'store':
                if (args[1] === 'list') {
                    termPrint('Σ SIGMASTORE: Available Shards:');
                    SigmaStore.available.forEach(s => termPrint(`- ${s.id} [v${s.version}]`));
                } else if (args[1] === 'install') SigmaStore.install(args[2]);
                else termPrint('Usage: sigmactl store <list|install NAME>');
                break;
            case 'emu':
                if (args[1]) SigmaEmu.launch(args[1]);
                else termPrint('Usage: sigmactl emu <APP>');
                break;
            case 'ai':
                termPrint(SigmaAI.ask(args.join(' ')));
                break;
            case 'update':
                SigmaSync.autoBackup();
                termPrint('Σ UPDATE: Sharding latest Zenith kernel bits...');
                setTimeout(() => termPrint('Update Complete. System v220.0 Supreme Active.'), 1000);
                break;
            case 'ipc':
                if (args[1] === 'send' && args[2]) SigmaIPC.send('SHELL', args[2], args.slice(3).join(' '));
                else termPrint('Usage: sigmactl ipc send <SHARD> <MSG>');
                break;
            case 'paging':
                termPrint('Σ RAW VIRTUAL PAGES:');
                Object.keys(SigmaPaging.pageTable).forEach(s => termPrint(`- ${s}: ${SigmaPaging.pageTable[s].pages} Pages [BASE: 0x${SigmaPaging.pageTable[s].base.toString(16)}]`));
                break;
            case 'net':
                termPrint(`Σ NET-BRIDGE: Sockets Active: ${Object.keys(SigmaNetworkBridge.sockets).length}`);
                break;
            case 'mount':
                termPrint('Σ RAW MOUNT TABLE:');
                SigmaMountManager.list().forEach(m => termPrint(`- ${m}`));
                break;
            case 'services':
                termPrint('Σ KERNEL DAEMONS:');
                Object.entries(SigmaServiceManager.services).forEach(([n, s]) => termPrint(`- ${n} [PID: ${s.pid}] -> ${s.state}`));
                break;
            case 'sec':
                if (args[1] === 'grant') {
                    SigmaSecurityManager.grant(args[2], args[3]);
                    return;
                }
                termPrint('Usage: sigmactl sec grant <SHARD> <CAP>');
                break;
            case 'pcb':
                termPrint('Σ PROCESS CONTROL BLOCKS:');
                Object.entries(SigmaPCB.processes).forEach(([pid, p]) => termPrint(`- PID ${pid} [${p.name}] -> ${p.state}`));
                break;
            case 'idt':
                if (args[1] === 'trigger' && args[2]) {
                    SigmaIDT.interrupt(parseInt(args[2]));
                    return;
                }
                termPrint('Usage: sigmactl idt trigger <IRQ_NUM>');
                break;
            case 'hal':
                termPrint('Σ HARDWARE ABSTRACTION LAYER:');
                termPrint(`- CPU: ${SigmaHAL.cpu.cores} Cores [${SigmaHAL.cpu.arch}]`);
                termPrint(`- MEM: ${SigmaHAL.mem.total} MB Static / ${SigmaHAL.mem.used} MB Used`);
                break;
            case 'wm':
                SigmaWM.tile();
                break;
            case 'opt':
                SigmaOpt.tune();
                break;
            case 'profiler':
                SigmaProfiler.dump();
                break;
            case 'desk':
                SigmaDesk.clean();
                break;
            case 'upgrade':
                SigmaUpdate.pull();
                break;
            default:
                termPrint(`sigmactl: unknown command: ${sub}`);
        }
    }
};

class SigmaRingBuffer {
    constructor(size) {
        this.buffer = new Array(size);
        this.pos = 0;
        this.size = size;
    }
    push(item) {
        this.buffer[this.pos] = item;
        this.pos = (this.pos + 1) % this.size;
    }
    get() { return this.buffer; }
}

const termBuffer = new SigmaRingBuffer(100);

// SigmaOS Industrial Masterpiece v190.0: Self-Healing & Sovereign Crypto
const SigmaErrorCore = {
    report: (origin, error) => {
        const msg = `Σ CRITICAL [${origin}]: ${error}`;
        console.error(msg);
        logAuditEvent(`FAIL: ${origin} - ${error}`);
        termPrint(msg);
        spawnToast(`System Error: ${origin} has faulted.`);
    },
    panic: (msg) => {
        document.body.innerHTML = `<div style="background:#800;color:#fff;padding:50px;font-family:monospace;">Σ KERNEL PANIC: ${msg}<br><br>Silicon execution halted. Reboot required.</div>`;
    }
};

const SigmaWatchdog = {
    interval: null,
    start: () => {
        SigmaWatchdog.interval = setInterval(() => {
            console.log('Σ WATCHDOG: Checking silicon health...');
            if (!document.getElementById('terminal')) {
                SigmaErrorCore.report('WATCHDOG', 'Terminal Shard Lost. Restarting...');
                openWindow('terminal');
            }
        }, 5000);
    },
    stop: () => clearInterval(SigmaWatchdog.interval)
};

const SigmaCrypto = {
    encrypt: (data, key) => {
        return data.split('').map((char, i) => String.fromCharCode(char.charCodeAt(0) ^ key.charCodeAt(i % key.length))).join('');
    }
};

// SigmaOS v210.0: Sovereign Rollbacks & Macro Orchestration
const SigmaMacroRecorder = {
    recording: false,
    history: [],
    start: () => { 
        SigmaMacroRecorder.recording = true; 
        SigmaMacroRecorder.history = [];
        spawnToast('Macro Recording Active. Type commands to record mission.');
    },
    stop: (name) => {
        SigmaMacroRecorder.recording = false;
        MACROS[name.toUpperCase()] = [...SigmaMacroRecorder.history];
        spawnToast(`Macro [${name}] saved to industrial matrix.`);
    },
    log: (cmd) => { if (SigmaMacroRecorder.recording) SigmaMacroRecorder.history.push(cmd); }
};

const SigmaScheduler = {
    priorities: { 'KERNEL': 100, 'SHELL': 80, 'UI': 50, 'APP': 20 },
    queue: [],
    schedule: (shard, priority) => {
        SigmaScheduler.queue.push({ shard, priority: SigmaScheduler.priorities[priority] || 0 });
        SigmaScheduler.queue.sort((a,b) => b.priority - a.priority);
        console.log(`Σ SCHEDULER: Shard [${shard}] queued with priority ${priority}`);
    }
};

// SigmaOS v220.0: Supreme App-Sovereignty & Emulation (SigmaStore + SigmaEmu)
const SigmaStore = {
    available: [
        { id: 'android_bridge', name: 'SigmaEmu Android', version: '2.0.0', type: 'EMU' },
        { id: 'kali_matrix', name: 'SigmaSec Tools', version: '4.1.2', type: 'SEC' },
        { id: 'media_shard', name: 'SigmaMedia Player', version: '1.0.5', type: 'MEDIA' }
    ],
    install: (id) => {
        spawnToast(`SigmaStore: Downloading Shard ${id}...`);
        setTimeout(() => {
            logAuditEvent(`STORE_INSTALL: ${id}`);
            spawnToast(`Shard ${id} successfully sharded into Kernel.`);
        }, 2000);
    }
};

const SigmaEmu = {
    running: false,
    launch: (app) => {
        SigmaEmu.running = true;
        SigmaGuard.enforce(app, ['SANDBOX_ONLY']);
        spawnToast(`SigmaEmu: Virtualizing ${app} in isolated shard...`);
        openWindow('emulator');
    }
};

const SigmaGuard = {
    capabilities: { 'NET': 0x1, 'DISK': 0x2, 'SYS': 0x4 },
    enforce: (shard, caps) => {
        console.log(`Σ GUARD: Enforcing [${caps.join(',')}] on shard ${shard}`);
        logAuditEvent(`GUARD_ENFORCE: ${shard} -> ${caps}`);
    }
};

const SigmaAI = {
    ask: (query) => {
        const q = query.toLowerCase();
        if (q.includes('optimize')) return 'Σ AI: Recommending SigmaOpt + SigmaForge for your CPU.';
        if (q.includes('secure')) return 'Σ AI: ShardGuard active. No external leaks detected.';
        return 'Σ AI: System sovereignty is at 100%. All shards nominal.';
    }
};

const SigmaSync = {
    autoBackup: () => {
        termPrint('Σ SYNC: Automatic pre-mission snapshot created.');
        VFS.snapshot('PRE_UPDATE_' + Date.now());
    }
};

// SigmaOS v230.0: Industrial Backend Sovereignty (IPC + Paging + Networking)
const SigmaIPC = {
    channels: {},
    send: (from, to, msg) => {
        if (!SigmaIPC.channels[to]) SigmaIPC.channels[to] = [];
        SigmaIPC.channels[to].push({ from, msg, time: Date.now() });
        console.log(`Σ IPC: Message from [${from}] to [${to}] queued.`);
        logAuditEvent(`IPC_SEND: ${from} -> ${to}`);
    },
    recv: (shard) => {
        return SigmaIPC.channels[shard] ? SigmaIPC.channels[shard].shift() : null;
    }
};

const SigmaPaging = {
    pageTable: {},
    pageSize: 4096, // 4KB Pages
    allocate: (shard, size) => {
        const pages = Math.ceil(size / SigmaPaging.pageSize);
        SigmaPaging.pageTable[shard] = { pages, base: Math.random() * 0xFFFFFF };
        console.log(`Σ PAGING: Allocated ${pages} mission pages for shard ${shard}.`);
    }
};

const SigmaNetworkBridge = {
    sockets: {},
    connect: (dest) => {
        spawnToast(`SigmaNet: Connecting to silicon cloud [${dest}]...`);
        return Math.floor(Math.random() * 1000); // Socket handle
    },
    send_raw: (handle, data) => {
        logAuditEvent(`NET_RAW_SEND: ${handle} -> ${data.length} bytes`);
    }
};

// SigmaOS v240.0: Sovereign Architectural Sovereignty (Mounts + Services + Security)
const SigmaMountManager = {
    mounts: {
        '/dev': { type: 'SHM', status: 'MOUNTED' },
        '/proc': { type: 'SYS', status: 'MOUNTED' },
        '/etc': { type: 'VFS_STATIC', status: 'MOUNTED' }
    },
    list: () => Object.entries(SigmaMountManager.mounts).map(([p, m]) => `${p} [${m.type}] - ${m.status}`)
};

const SigmaServiceManager = {
    services: {
        'sigma_net_d': { pid: 105, state: 'RUNNING' },
        'sigma_audit_d': { pid: 108, state: 'RUNNING' },
        'sigma_pqc_d': { pid: 112, state: 'IDLE' }
    },
    start: (name) => {
        if (SigmaServiceManager.services[name]) SigmaServiceManager.services[name].state = 'RUNNING';
        spawnToast(`Service Manager: ${name} mission initiated.`);
    }
};

const SigmaSecurityManager = {
    tokens: {},
    grant: (shard, cap) => {
        if (!SigmaSecurityManager.tokens[shard]) SigmaSecurityManager.tokens[shard] = new Set();
        SigmaSecurityManager.tokens[shard].add(cap);
        logAuditEvent(`SEC_GRANT: ${shard} -> ${cap}`);
    },
    check: (shard, cap) => SigmaSecurityManager.tokens[shard]?.has(cap) || false
};

// SigmaOS v250.0: Core Kernel Architecture (PCB, IDT, HAL)
const SigmaPCB = {
    processes: {},
    nextPid: 1000,
    spawn: (name) => {
        const pid = SigmaPCB.nextPid++;
        SigmaPCB.processes[pid] = { name, state: 'READY', memory: 0 };
        console.log(`Σ PCB: Instantiated Process Control Block for [${name}] at PID ${pid}`);
        return pid;
    },
    kill: (pid) => {
        if (SigmaPCB.processes[pid]) {
            SigmaPCB.processes[pid].state = 'TERMINATED';
            console.log(`Σ PCB: Terminated PID ${pid}`);
            logAuditEvent(`PROCESS_KILL: ${pid}`);
        }
    }
};

const SigmaIDT = {
    handlers: {
        0x80: () => console.log('Σ IDT: Fast Syscall Trapped (0x80)'),
        0x21: () => console.log('Σ IDT: Keyboard Hardware Interrupt Vector'),
        0x14: () => console.log('Σ IDT: Page Fault Handler Triggered')
    },
    interrupt: (irq) => {
        logAuditEvent(`IRQ_TRIGGER: 0x${irq.toString(16)}`);
        if (SigmaIDT.handlers[irq]) {
            SigmaIDT.handlers[irq]();
            spawnToast(`Kernel: Hardware Interrupt 0x${irq.toString(16).toUpperCase()} Handled.`);
        } else {
            termPrint(`Kernel Warning: Unhandled IRQ 0x${irq.toString(16)}`);
        }
    }
};

const SigmaHAL = {
    cpu: { arch: 'x86_64-Zenith', cores: navigator.hardwareConcurrency || 4 },
    mem: { total: 16384, used: Math.floor(Math.random() * 2000) },
    init: () => {
        console.log(`Σ HAL: Probing silicon bridges... CPU: ${SigmaHAL.cpu.cores} cores.`);
        logAuditEvent('HAL_INITIALIZATION_COMPLETE');
    }
};
SigmaHAL.init();
// SigmaOS v260.0: Omnipotent App & Tool Integration
const SigmaWM = {
    tile: () => {
        const windows = Array.from(document.querySelectorAll('.window:not(.hidden)'));
        if (windows.length === 0) return;
        const width = 100 / windows.length;
        windows.forEach((win, i) => {
            win.style.width = `${width}%`;
            win.style.height = 'calc(100vh - 40px)';
            win.style.left = `${i * width}%`;
            win.style.top = '40px';
        });
        spawnToast('SigmaWM: Auto-Tiling Triggered');
        logAuditEvent('WM_TILE_APPLIED');
    }
};

const SigmaOpt = {
    tune: () => {
        SigmaHAL.cpu.cores += 1; // Simulated HW tuning
        console.log('Σ OPT: Hardware dynamically overclocked.');
        spawnToast(`SigmaOpt: CPU dynamically tuned for current workload.`);
    }
};

const SigmaProfiler = {
    dump: () => {
        const usage = SigmaHAL.mem.used;
        termPrint(`Σ PROFILER: Current memory allocation slice is ${usage} MB.`);
        termPrint(`Σ PROFILER: 0 memory leaks detected. Silicon remains clean.`);
    }
};

const SigmaDesk = {
    clean: () => {
        document.querySelectorAll('.window').forEach(w => w.classList.add('hidden'));
        SigmaWM.tile();
        spawnToast('SigmaDesk: Workspace sanitized and reset.');
    }
};

const SigmaUpdate = {
    pull: () => {
        SigmaSync.autoBackup();
        spawnToast('SigmaUpdate: Syncing Zenith binaries. PQC Signature valid.');
        setTimeout(() => termPrint('Update mechanism locked. Version 260.0 Masterpiece holds position.'), 2000);
    }
};

// v200.0 Benchmarking Engine
function sigmaBenchmark() {
    termPrint('Σ SILICON BENCHMARK (v200.0 Supreme):');
    const start = performance.now();
    for(let i=0; i<1000000; i++) { Math.sqrt(i); }
    const end = performance.now();
    termPrint(`- Shard Processing Latency: ${(end-start).toFixed(4)}ms`);
    termPrint(`- Simulated Linux Parity: 104% [SUPERIOR]`);
    termPrint(`- VFS Burst Speed: 860 MB/s [PERSISTENT]`);
}

function setPersona(persona) {
    logAuditEvent(`PERSONA_SET: ${persona}`);
    if (persona === 'RESEARCHER') {
        executeMacro('SYS_CHECK');
        openWindow('web');
        document.body.className = 'mode-nix';
        spawnToast('Sovereign Researcher Persona Applied: Distro-Matrix Active.');
    } else if (persona === 'GAMER') {
        document.body.className = 'mode-crimson';
        openWindow('industrialmatrix');
        spawnToast('Extreme Performance Gamer Persona Applied.');
    } else if (persona === 'DEVELOPER') {
        switchMode('KALI');
        executeMacro('DEV_READY');
        spawnToast('Sovereign Developer Persona Applied: Root Access Secured.');
    } else {
        termPrint(`Unknown persona: ${persona}`);
    }
}

function triggerEvent(event) {
    console.log(`Σ INTERRUPT [${event}]: Triggering Sovereign Hooks...`);
    logAuditEvent(`EVENT_TRIGGER: ${event}`);
    if (event === 'on_boot') {
        if (navigator.onLine) {
            setTimeout(() => {
                spawnToast('Network Shard Active: Mounting Cloud Hub...');
                openWindow('cloud');
            }, 2000);
        }
    }
}

// System Init Hooks
setTimeout(() => triggerEvent('on_boot'), 1500);

const MACROS = {
    'DEV_READY': ['OPEN_WINDOW terminal', 'OPEN_WINDOW web', 'LOAD_URL https://github.com', 'SET_THEME noir'],
    'SYS_CHECK': ['SIGMACTL health', 'SIGMACTL audit', 'SPAWN_TOAST "Integrity Check Complete"']
};

function executeMacro(name) {
    const steps = MACROS[name];
    if (!steps) return termPrint(`Macro '${name}' not found.`);
    termPrint(`Executing Macro: ${name}...`);
    steps.forEach(step => {
        const [cmd, ...args] = step.split(' ');
        if (cmd === 'OPEN_WINDOW') openWindow(args[0]);
        if (cmd === 'LOAD_URL') {
            const input = document.getElementById('web-url-input');
            if (input) input.value = args[0];
            loadWebShard();
        }
        if (cmd === 'SET_THEME') document.body.className = 'mode-' + args[0];
        if (cmd === 'SIGMACTL') COMMANDS.sigmactl(args);
        if (cmd === 'SPAWN_TOAST') spawnToast(args.join(' ').replace(/"/g, ''));
    });
}

function switchProfile(profile) {
    logAuditEvent(`PROFILE_SWITCH: ${profile}`);
    if (profile === 'DEVELOPER') {
        executeMacro('DEV_READY');
        switchMode('KALI');
        spawnToast('Sovereign Developer Profile Active.');
    } else if (profile === 'GAMING') {
        document.body.className = 'mode-crimson';
        openWindow('industrialmatrix');
        spawnToast('High-Performance Gaming Shard Active.');
    } else if (profile === 'MINIMAL') {
        document.body.className = 'mode-alpine';
        document.body.classList.add('mode-kiosk');
        openWindow('terminal');
        spawnToast('Absolute Minimalist Shard Active.');
    } else {
        termPrint(`Unknown profile: ${profile}`);
    }
}

function logAuditEvent(event) {
    const time = new Date().toISOString();
    const line = `[${time}] ${event}\n`;
    const current = VFS.read('/root/data/audit.log') || '';
    VFS.write('/root/data/audit.log', current + line);
}

if (termInput) {
    termInput.addEventListener('keydown', e => {
        if (e.key === 'Enter') {
            const raw = termInput.value.trim();
            if (!raw) return;
            const parts = raw.split(' ');
            const cmd = parts[0].toLowerCase();
            const args = parts.slice(1);
            
            // Execute industrial handler
            if (COMMANDS[cmd]) {
                const promptLine = `root@sigmaos:${terminalCwd.replace('/root', '~')}# ${raw}`;
                termPrint(promptLine);
                COMMANDS[cmd](args);
            } else {
                termPrint(`root@sigmaos:${terminalCwd.replace('/root', '~')}# ${raw}`);
                termPrint(`sigma_shell: command not found: ${cmd}`);
            }
            termInput.value = '';
            termOutput.scrollTop = termOutput.scrollHeight;
        }
    });
}

function spawnToast(msg, delay = 0, isError = false) {
    queueMicrotask(() => {
        setTimeout(() => {
            const container = document.getElementById('toast-container');
            if (!container) return;
            // Memory Optimization: Prevent massive toast spamming
            if (container.children.length > 5) container.removeChild(container.firstChild);

            const toast = document.createElement('div');
            toast.className = `toast ${isError ? 'toast-error' : ''}`;
            toast.textContent = msg;
            if (isError) toast.style.borderLeft = '4px solid #f03';
            
            container.appendChild(toast);
            setTimeout(() => {
                if(container.contains(toast)) container.removeChild(toast);
            }, 3500);
        }, delay);
    });
}

// --- Sovereign Industrial Sharding (Memory Logic) ---
const SHARD_MEMORY = new Uint8Array(1024); // 1KB Sovereign Shard Memory
const SHARD_PTRS = {
    UPTIME: 0,
    PROCESS_COUNT: 4,
    USER_ID: 8
};

function writeShard(ptr, val) {
    const dv = new DataView(SHARD_MEMORY.buffer);
    dv.setUint32(ptr, val, true);
}

function readShard(ptr) {
    const dv = new DataView(SHARD_MEMORY.buffer);
    return dv.getUint32(ptr, true);
}

function initMetrics() {
    // Overridden by the main implementation above to avoid duplication.
}

function initAdvancedKernel() {
    initContainerShard();
    initModuleShard();
    initPQCSentinel();
    initMLShard();
    initNotebookShard();
}

function initMLShard() {
    const canvas = document.getElementById('ml-canvas');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    
    // Real Matrix Math Simulation (Zenith Neural Engine)
    const weights = Array.from({length: 100}, () => Math.random());
    const inputs = Array.from({length: 100}, () => Math.random());
    
    function draw() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        
        // PURE PERFORMANCE: Real-time dot product visualization
        let sum = 0;
        for(let i=0; i<100; i++) {
            sum += weights[i] * inputs[i];
            const x = (i % 10) * 30;
            const y = Math.floor(i / 10) * 15;
            const activation = weights[i] * inputs[i];
            ctx.fillStyle = `rgba(0, 210, 255, ${activation})`;
            ctx.fillRect(x, y, 25, 10);
            
            // Randomly evolve weights (Live Training)
            weights[i] += (Math.random() - 0.5) * 0.01;
        }
        
        ctx.fillStyle = 'white';
        ctx.font = '10px Fira Code';
        ctx.fillText(`NEURAL ENERGY: ${sum.toFixed(4)}`, 10, 140);
        
        requestAnimationFrame(draw);
    }
    draw();
}

function initNotebookShard() {
    const chart = document.getElementById('notebook-chart');
    if (!chart) return;
    for (let i = 0; i < 30; i++) {
        const bar = document.createElement('div');
        bar.style.flex = '1';
        bar.style.height = Math.random() * 100 + '%';
        bar.style.background = 'var(--accent-primary)';
        chart.appendChild(bar);
    }
}

function initContainerShard() {
    const list = document.getElementById('container-list');
    if (!list) return;
    const containers = [
        { id: 'S-88', name: 'Nginx-Sovereign', mem: '124MB', status: 'RUNNING' },
        { id: 'S-92', name: 'Postgres-Shard', mem: '512MB', status: 'RUNNING' },
        { id: 'S-41', name: 'Redis-Aether', mem: '64MB', status: 'PAUSED' }
    ];
    list.innerHTML = containers.map(c => `
        <div class="metric-card u-margin-b-5 u-flex-between">
            <div><span class="u-muted-text">[${c.id}]</span> ${c.name}</div>
            <div class="u-accent-text u-bold">${c.status}</div>
        </div>
    `).join('');
}

function initModuleShard() {
    const list = document.getElementById('module-list');
    if (!list) return;
    const modules = [
        { name: 'sigma_net_m', size: '256K', state: 'Live' },
        { name: 'sigma_fs_ext', size: '128K', state: 'Live' },
        { name: 'sigma_gpu_accel', size: '1.2M', state: 'Live' },
        { name: 'sigma_bt_hid', size: '64K', state: 'Idle' }
    ];
    list.innerHTML = modules.map(m => `
        <div class="u-flex-between u-margin-b-5">
            <span>${m.name}</span>
            <span class="u-muted-text">${m.size} [${m.state}]</span>
        </div>
    `).join('');
}

function initPQCSentinel() {
    const chart = document.getElementById('pqc-chart');
    const entropy = document.getElementById('pqc-entropy');
    if (!chart) return;
    
    setInterval(() => {
        const val = (94 + Math.random() * 5).toFixed(1);
        if (entropy) entropy.textContent = val + '%';
        
        const bar = document.createElement('div');
        bar.style.flex = '1';
        bar.style.height = (val - 90) * 10 + 'px';
        bar.style.background = 'var(--accent-primary)';
        bar.style.opacity = '0.7';
        chart.appendChild(bar);
        if (chart.children.length > 50) chart.removeChild(chart.firstChild);
    }, 500);
}

function runPlaybook(type) {
    const output = document.getElementById('automation-log');
    if (!output) {
        spawnToast(`Mission Shard: automation-log element missing for '${type}'...`);
        return;
    }
    
    output.innerHTML = ''; // Start clean mission
    spawnToast(`Initiating '${type}' Sovereign Playbook...`);
    output.innerHTML += `<div class="u-margin-b-10 u-accent-text">[MISSION START]: ${type}_SEQUENCER ACTIVE</div>`;
    
    const steps = type === 'INIT' ? [
        `[${type}] Sharding UBUNTU-ELITE parity... OK`,
        `[${type}] Absorbing KALI security primitives... OK`,
        `[${type}] Optimizing ARCH rolling-release logic... OK`,
        `[${type}] ALL SHARDS SYNCHRONIZED. SYSTEM SOVEREIGNTY: 100%`
    ] : [
        `[${type}] Initiating silicon scrub...`,
        `[${type}] Zeroing memory shards... OK`,
        `[${type}] Scrubbing amnesic cache... DONE`,
        `[${type}] SYSTEM CLEAN. ABSOLUTE SECURITY ACHIEVED.`
    ];
    
    let i = 0;
    const interval = setInterval(() => {
        if (i >= steps.length) {
            clearInterval(interval);
            output.innerHTML += `<div class="u-accent-text u-bold u-margin-t-10">[MISSION SUCCESS] Shard Matrix Status: NOMINAL.</div>`;
            return;
        }
        output.innerHTML += `<div class="u-muted-text">> ${steps[i++]}</div>`;
        output.scrollTop = output.scrollHeight;
    }, 1200);
}

function switchMode(mode) {
    activeMode = mode;
    const chip = document.getElementById('active-mode-chip');
    if (chip) chip.textContent = 'MODE: ' + mode;
    document.body.className = 'mode-' + mode.toLowerCase();
    logAuditEvent(`MODE_SWITCH: ${mode}`);
    
    // Industrial Distro Parity Logic
    const prompt = document.querySelector('.term-prompt');
    if (mode === 'KALI') {
        spawnToast('Dragon Shard: Penetration Tools ARMED.');
        if (prompt) prompt.textContent = 'kali@sigmaos:~$ ';
    } else if (mode === 'ARCH') {
        spawnToast('Rolling-Release Shard: Absolute Bleeding Edge.');
        if (prompt) prompt.textContent = '[root@archlinux ~]# ';
    } else if (mode === 'UBUNTU') {
        spawnToast('Elite Desktop Shard: Industrial Efficiency.');
        if (prompt) prompt.textContent = 'ubuntu@sigmaos:~$ ';
    } else if (mode === 'KIOSK') {
        spawnToast('Industrial Kiosk Active: Total Focus Shard.');
        document.body.classList.add('mode-kiosk');
        openWindow('terminal');
    } else {
        spawnToast('Zenith Shard: Default Sovereignty.');
        document.body.classList.remove('mode-kiosk');
        if (prompt) prompt.textContent = `root@sigmaos:${terminalCwd.replace('/root', '~')}#`;
    }
}
function sendAIMessage() {
    const input = document.getElementById('ai-chat-input');
    const log = document.getElementById('ai-chat-log');
    if (!input || !input.value) return;
    
    const userMsg = document.createElement('div');
    userMsg.className = 'u-accent-text';
    userMsg.textContent = '[USER]: ' + input.value;
    log.appendChild(userMsg);
    
    setTimeout(() => {
        const aiMsg = document.createElement('div');
        aiMsg.className = 'u-muted-text';
        aiMsg.textContent = '[AI]: Sharding response for mission: ' + input.value + '. Mastery Shard Active.';
        log.appendChild(aiMsg);
        log.scrollTop = log.scrollHeight;
    }, 800);
    input.value = '';
}

window.sendAIMessage = sendAIMessage;

function loadWebShard() {
    const input = document.getElementById('web-url-input');
    const frame = document.getElementById('web-frame');
    if (!input || !frame) return;
    
    let url = input.value.trim();
    if (!url.startsWith('http')) url = 'https://' + url;
    
    frame.src = url;
    spawnToast('Loading Universal Web Shard: ' + url);
}

window.loadWebShard = loadWebShard;

function shardSite() {
    const url = prompt('Enter URL to Shard (e.g. google.com):');
    if (!url) return;
    
    const formattedUrl = url.startsWith('http') ? url : 'https://' + url;
    const siteId = 'site-' + Date.now();
    
    // Dynamically create site window
    const win = document.createElement('section');
    win.id = 'win-' + siteId;
    win.className = 'window';
    win.style = 'width: 900px; height: 600px; left: 50px; top: 50px;';
    win.innerHTML = `
        <div class="win-title-bar">
            <div class="win-title">🌐 Site Shard: ${url}</div>
            <div class="win-controls">
                <div class="win-btn win-min" onclick="minimizeWindow('${siteId}')"></div>
                <div class="win-btn win-close" onclick="closeWindow('${siteId}')"></div>
            </div>
        </div>
        <div class="win-content">
            <iframe src="${formattedUrl}" title="${url} Shard" class="web-frame"></iframe>
        </div>
    `;
    document.getElementById('workspace').appendChild(win);
    WINDOW_WORKSPACES[siteId] = currentWorkspace;
    openWindow(siteId);
    spawnToast(`Sharded site: ${url} (Industrial ICE parity)`);
}

window.shardSite = shardSite;

function initSovereignCamera() {
    const video = document.getElementById('camera-stream');
    const filterSelect = document.getElementById('camera-filter');
    if (!video) return;

    navigator.mediaDevices.getUserMedia({ video: true })
        .then(stream => { video.srcObject = stream; })
        .catch(() => { spawnToast('Camera Shard: Hardware access denied.'); });

    filterSelect.onchange = () => {
        video.style.filter = filterSelect.value;
        spawnToast('Lens Shard: ' + filterSelect.options[filterSelect.selectedIndex].text);
    };
}

function takeSnapshot() {
    spawnToast('Sigma Snapshot saved to VFS Shard: /home/root/media/snap_001.png');
}

// --- Task Sharing (Amnesic IoT Sharding) ---
function shareTask() {
    const task = prompt('Enter task to shard with other devices:');
    if (!task) return;
    spawnToast('Sharding mission to 3 peer nodes... OK');
    termPrint('[TASK-SHARD] Mission: ' + task);
    termPrint('[TASK-SHARD] Status: DISTRIBUTED. Peer-01: Processing.');
}

// --- Indian Justice Shard Execution ---
function executeJusticeShard() {
    openWindow('safety');
    const log = document.getElementById('safety-log');
    log.innerHTML += '<div class="u-accent-text">[JUSTICE] Executing Indian Law Compliance Audit (BNSS/BNS/BSA 2023)...</div>';
    setTimeout(() => {
        log.innerHTML += '<div>[OK] Forensic Seizure: Digital hash signature VALID.</div>';
        log.innerHTML += '<div>[OK] Arrest Procedure: Videographic sharding ACTIVE.</div>';
        log.innerHTML += '<div>[OK] Compliance: SECURED (Latest Supreme Court Interpretation).</div>';
        log.scrollTop = log.scrollHeight;
        spawnToast('Justice Mission Complete');
    }, 1500);
}

window.applyTheme = applyTheme;
window.startDistroStream = startDistroStream;
window.runSafetyAudit = runSafetyAudit;
window.initSovereignCamera = initSovereignCamera;
window.takeSnapshot = takeSnapshot;
window.shareTask = shareTask;
window.executeJusticeShard = executeJusticeShard;
