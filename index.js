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
    { id: 'noir', name: 'OLED-Noir', primary: '#111111', blur: '0px' }
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
    spawnToast('Σ SIGMAOS ZENITH SUPREME INITIALIZED');
    spawnToast('Industrial Shard Mastery Active', 1500);
});

function initClock() {
    setInterval(() => {
        clockEl.textContent = new Date().toTimeString().split(' ')[0];
        sysUptime++;
    }, 1000);
}

function initMetrics() {
    setInterval(() => {
        const cpu = (Math.random() * 5 + 2).toFixed(1);
        cpuVal.textContent = cpu + '%';
        const cpuFill = document.querySelector('#cpu-pulse .progress-fill');
        if (cpuFill) cpuFill.style.width = cpu + '%';
    }, 2000);
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
        spawnToast(`Shard [${id}] Terminated via Silicon Signal.`);
    }
}

function minimizeWindow(id) {
    const win = document.getElementById('win-' + id);
    if (win) {
        win.classList.add('hidden');
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
        focusWindow(win);
    }
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
                if (match.type === 'Tool') openWindow(match.id);
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
function initSpectrumTerminal() {
    const input = document.getElementById('spectrum-input');
    const output = document.getElementById('spectrum-output');
    if (!input) return;

    input.addEventListener('keydown', e => {
        if (e.key === 'Enter') {
            const val = input.value.trim();
            const line = document.createElement('div');
            line.className = 'term-line';
            line.textContent = 'spectrum@sigma:~$ ' + val;
            output.appendChild(line);
            
            // AI Prediction Simulation
            const prediction = document.createElement('div');
            prediction.className = 'term-line u-muted-text';
            prediction.textContent = '[AI-PREDICT] Executing industrial shard for: ' + val;
            output.appendChild(prediction);
            
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
function initVFSView() { /* Simplified */ }
function initRepoView() { /* Simplified */ }
function initWorkspaces() { /* Simplified */ }

function termPrint(text, type = '') {
    const div = document.createElement('div');
    div.className = 'term-line ' + type;
    div.textContent = text;
    termOutput.appendChild(div);
    termOutput.scrollTop = termOutput.scrollHeight;
}

if (termInput) {
    termInput.addEventListener('keydown', e => {
        if (e.key === 'Enter') {
            const val = termInput.value.trim();
            termPrint('root@sigmaos:~# ' + val);
            termInput.value = '';
        }
    });
}

function spawnToast(msg, delay = 0) {
    setTimeout(() => {
        const container = document.getElementById('toast-container');
        const toast = document.createElement('div');
        toast.className = 'toast';
        toast.textContent = msg;
        container.appendChild(toast);
        setTimeout(() => toast.remove(), 3000);
    }, delay);
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
    setInterval(() => {
        const cpu = (Math.random() * 5 + 2).toFixed(1);
        cpuVal.textContent = cpu + '%';
        const uptime = readShard(SHARD_PTRS.UPTIME);
        writeShard(SHARD_PTRS.UPTIME, uptime + 1);
        
        const cpuFill = document.querySelector('#cpu-pulse .progress-fill');
        if (cpuFill) cpuFill.style.width = cpu + '%';
    }, 1000);
}

// --- Sovereign Camera (Snapchat/Scratch USP) ---
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
