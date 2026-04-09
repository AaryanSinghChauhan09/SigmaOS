// $ZENITH Sovereign Wrapper (Zero-Dependency)
const $ZENITH = (id) => document.getElementById(id);

const COMMAND_RESPONSES = {
    'HELP': [
        "Σ SUPREME ZENITH COMMANDS:",
        "  FORK_TEST       — xv6 silicon process duplication.",
        "  DMA_CMD         — Silberschatz-grade host bus transfer.",
        "  PETERSON        — Peterson's critical section lock.",
        "  SYSTEM_STATUS   — Query silicon-direct registry.",
        "  MASTER_AUDIT    — Bit-level shard verification.",
        "  NIX_BUILD       — Realise a reproducible Nix derivation.",
        "  EMERGE          — Portage USE-flag-aware emerge.",
        "  SV_STATUS       — Runit service supervision tree.",
        "  PLEDGE          — Apply OpenBSD pledge/unveil sandbox.",
        "  TILE            — Trigger Pop!_OS auto-tile retile.",
        "  OSTREE_UPGRADE  — Atomic Silverblue OSTree upgrade.",
        "  PACMAN_SYNC     — Arch rolling-release mirror sync.",
        "  ABSORB_LEGACY   — Unify v1.0-v3000.0 into Zenith core."
    ],
    'FORK_TEST': ["[ZENITH]: Duplicating Shard 0x93...", "[CHILD]: Sovereign Child Active.", "[PARENT]: Child re-absorbed."],
    'DMA_CMD': ["[ZENITH-DMA]: Initiating host-bus transfer (4096 bytes).", "[OK]: Success."],
    'PETERSON': ["[ZENITH-SYNC]: Thread-0 Entry. Lock secured via Peterson's Shard."],
    'SYSTEM_STATUS': ["BUILD: v3000.0 ZENITH SUPREME", "SOVEREIGNTY: 100%", "THREAT-LEVEL: ZERO", "SHARDS: 380+", "DISTROS-ABSORBED: 220+"],
    'MASTER_AUDIT': ["[AUDIT]: Verifying 380+ sovereign shards...", "[OK]: 100% integrity — Phase 40 verified."],
    'NIX_BUILD': ["[NIX]: Hashing inputs...", "[NIX]: /nix/store/abc123-sigmaos-kernel-v3000 realised.", "[OK]: Reproducible build complete."],
    'EMERGE': ["[PORTAGE]: Calculating USE-flag dependencies...", "[PORTAGE]: >>> Emerging sys-kernel/sigma-sources", "[OK]: 1 package merged."],
    'SV_STATUS': ["[RUNIT]: ok: sigma-syslog (pid 101)", "[RUNIT]: ok: sigma-network (pid 102)", "[RUNIT]: ok: sigma-dbus (pid 103)", "[RUNIT]: ok: sigma-display (pid 104)"],
    'PLEDGE': ["[PLEDGE]: pid=1001 restricted to stdio rpath net dns", "[UNVEIL]: /home rw", "[UNVEIL]: LOCKED."],
    'TILE': ["[AUTOTILE]: Retiling 3 windows in 2x2 grid on ws=0", "[OK]: COSMIC auto-tile applied."],
    'OSTREE_UPGRADE': ["[OSTREE]: Fetching update for sigmaos/x86_64/stable...", "[OSTREE]: commit staged.", "[OSTREE]: Reboot to activate v3001."],
    'PACMAN_SYNC': ["[PACMAN]: :: Synchronising package databases...", "[REFLECTOR]: Fastest mirror: sigma.io (12ms)", "[OK]: sigma-core synced."]
};

const ALL_REPO_FILES = [
    "SovereignSuperCalculator.cpp", "SovereignML.cpp", "SovereignTranspilerZenith.cpp",
    "SovereignNetMesh.cpp", "SovereignWebBridge.cpp", "SovereignOmniTool.cpp",
    "SovereignAppStore.cpp", "SovereignVoiceShard.cpp", "SovereignAutomationShard.cpp",
    "SovereignHypervisorZenith.cpp", "SovereignDiagnosticsZenith.cpp", "SovereignEncyclopedia.cpp",
    "SovereignXV6Bridge.cpp", "SovereignKnowledgeAudit.cpp", "SovereignZenithComplete.cpp",
    "SovereignAetherAbsorption.cpp", "SovereignLatticePQC.cpp", "SovereignForensicMatrix.cpp",
    "SovereignGraphicsCompositor.cpp", "SovereignHardwareAudit.cpp", "SovereignLibC.asm", 
    "SovereignStandardHAL.asm", "SovereignScholasticDB.cpp", "SovereignVoiceZenith.cpp",
    "SovereignDistroForge.cpp", "SovereignPacketMaestro.cpp", "SovereignRing0Finality.cpp",
    "SovereignNcertUnity.cpp", "SovereignCognitiveSynth.cpp", "SigmaOOP.hpp", "SigmaRustCore.rs"
];

// Supremacy State
let maxZ = 100;

function openWindow(id) {
    const win = document.getElementById(id);
    const task = document.getElementById(`task-${id}`);
    if (win) {
        win.classList.remove('hidden');
        win.style.zIndex = ++maxZ;
    }
    if (task) {
        task.classList.remove('hidden');
        task.classList.add('active');
    }
}

function closeWindow(id) {
    const win = document.getElementById(id);
    const task = document.getElementById(`task-${id}`);
    if (win) win.classList.add('hidden');
    if (task) {
        task.classList.add('hidden');
        task.classList.remove('active');
    }
}

function focusWindow(id) {
    const win = document.getElementById(id);
    const task = document.getElementById(`task-${id}`);
    if (win) win.style.zIndex = ++maxZ;
    const items = document.querySelectorAll('.task-item');
    items.forEach(i => i.classList.remove('active'));
    if (task) task.classList.add('active');
}

function dragWindow(e, id) {
    const win = document.getElementById(id);
    let offsetX = e.clientX - win.offsetLeft;
    let offsetY = e.clientY - win.offsetTop;
    focusWindow(id);

    function mouseMove(e) {
        win.style.left = (e.clientX - offsetX) + 'px';
        win.style.top = (e.clientY - offsetY) + 'px';
    }
    function mouseUp() {
        document.removeEventListener('mousemove', mouseMove);
        document.removeEventListener('mouseup', mouseUp);
    }
    document.addEventListener('mousemove', mouseMove);
    document.addEventListener('mouseup', mouseUp);
}

// Terminal Shard
const output = document.getElementById('output');
const input = document.getElementById('command-input');

function addLine(text, className = '') {
    if (!output) return;
    const p = document.createElement('p');
    p.classList.add('line');
    if (className) p.classList.add(className);
    p.textContent = text;
    output.appendChild(p);
    output.scrollTop = output.scrollHeight;
}

if (input) {
    input.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') {
            const cmd = input.value.trim().toUpperCase();
            addLine(`Σ://zenith> ${cmd}`, 'prompt');
            if (COMMAND_RESPONSES[cmd]) {
                COMMAND_RESPONSES[cmd].forEach(line => addLine(line));
            } else if (cmd !== '') {
                addLine(`[ERR]: Intent for shard '${cmd}' discarded. Access ring-0.`);
            }
            input.value = '';
        }
    });
}

// Sovereign Master Pulse (SMP) - Monolithic Scheduler Tracker
function SovereignMasterPulse() {
    updateClock();
    animateSiliconPulse();
    
    // Cycle every 500ms
    window.requestAnimationFrame(() => {
        setTimeout(SovereignMasterPulse, 500); 
    });
}
SovereignMasterPulse();

function updateClock() {
    const clock = document.getElementById('clock');
    if (clock) {
        const now = new Date();
        clock.textContent = now.getHours().toString().padStart(2, '0') + ":" + now.getMinutes().toString().padStart(2, '0');
    }
}

function animateSiliconPulse() {
    const eax = document.getElementById('reg-eax');
    const ebx = document.getElementById('reg-ebx');
    if (eax && ebx) {
        eax.style.width = (Math.random() * 80 + 20) + '%';
        ebx.style.width = (Math.random() * 80 + 20) + '%';
    }
}

// Custom Macro Engine (Zero-Dependency)
function execScript(name) {
    openWindow('omni-shell');
    addLine(`Σ://automation> Running script: ${name}...`, 'text-neon-gold');
    const steps = [
        `[INIT]: Loading ${name}...`,
        `[BUILD]: Compiling shard modules...`,
        `[LINK]: Binding sovereign symbols...`,
        `[OK]: ${name} merged into Zenith context.`
    ];
    steps.forEach((step, i) => {
        setTimeout(() => addLine(step, i === steps.length - 1 ? 'text-neon-green' : 'text-neon-dim'), i * 350);
    });
}

function initCatalog() {
    const catalog = document.getElementById('catalog-content');
    if (catalog) {
        catalog.innerHTML = '';
        ALL_REPO_FILES.forEach(shard => {
            const div = document.createElement('div');
            div.className = 'shard-item';
            div.style.padding = '8px 0';
            div.innerHTML = `<span style="color:#00ff88">STATUS: MASTER</span> | <span style="color:#00f2ff">${shard}</span>`;
            catalog.appendChild(div);
        });
    }
}
initCatalog();

function filterShards() {
    const q = document.getElementById('nexus-search').value.toLowerCase();
    const items = document.querySelectorAll('.shard-item');
    items.forEach(item => {
        item.style.display = item.textContent.toLowerCase().includes(q) ? 'block' : 'none';
    });
}

function globalSearch() {
    const q = document.getElementById('global-search').value.toLowerCase();
    const icons = document.querySelectorAll('.icon');
    icons.forEach(icon => {
        const text = icon.querySelector('span').textContent.toLowerCase();
        icon.style.display = text.includes(q) ? 'flex' : 'none';
    });
}

// Personalization & Automation
function setAccent(color) {
    document.documentElement.style.setProperty('--accent-primary', color);
    document.documentElement.style.setProperty('--border', color + '33');
    document.querySelector('.start-btn-zenith').style.background = color;
    document.querySelector('.start-btn-zenith').style.boxShadow = `0 0 30px ${color}`;
}

// execScript is defined above (unified implementation)

// Absorption Logic
function startAbsorption() {
    const out = document.getElementById('absorb-output');
    if (!out) return;
    out.innerHTML = "<p style='color:#00f2ff'>[INIT]: Absorbing Legacy VPC Shards...</p>";
    setTimeout(() => {
        out.innerHTML += "<p style='color:#7000ff'>[INIT]: Absorbing Quantum Lattice V5 Shards...</p>";
        setTimeout(() => {
            out.innerHTML += "<p style='color:#00ff88'>[OK]: ABSORPTION SUCCESSFUL. ALL VERSIONS MERGED.</p>";
            const level = document.getElementById('merged-level');
            if (level) level.textContent = "LEVEL: ULTIMATE SUPREMACY";
        }, 1000);
    }, 1000);
}

// Linux Emulator Logic
const linuxIn = $ZENITH('linux-input');
const linuxOut = $ZENITH('linux-output');

// Professional Toast Signaling
function showToast(msg) {
    const container = $ZENITH('toast-container');
    if (!container) return;
    const toast = document.createElement('div');
    toast.className = 'toast holo-border';
    toast.textContent = `Σ://ZENITH-SIGNAL> ${msg}`;
    container.appendChild(toast);
    setTimeout(() => toast.remove(), 4000);
}

if (linuxIn) {
    linuxIn.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') {
            const cmd = linuxIn.value.trim().toLowerCase();
            const p = document.createElement('p');
            p.className = 'text-neon-blue';
            p.textContent = `root@sigmaos:~# ${cmd}`;
            linuxOut.appendChild(p);

            if (cmd === 'ls') {
                const sp = document.createElement('p');
                sp.textContent = "bin/  etc/  home/  root/  sigma_shards/  os_guide.md";
                linuxOut.appendChild(sp);
            } else if (cmd.startsWith('cat')) {
                const sp = document.createElement('p');
                sp.className = 'text-neon-dim';
                sp.textContent = "[[ Σ SIGMAOS ZENITH SUPREME v93.0 MASTER MANUAL ABSORBED ]]";
                linuxOut.appendChild(sp);
            } else if (cmd.startsWith('nano')) {
                const sp = document.createElement('p');
                sp.textContent = "[ZENITH]: Integrated Nano-Shard Opened. (Ctrl+X to exit).";
                linuxOut.appendChild(sp);
                showToast("Nano Shard Persisted.");
            } else if (cmd === 'top') {
                const sp = document.createElement('p');
                sp.textContent = "[ZENITH-TOP]: 0.1% CPU | 4MB RAM | 100+ SHARDS IDLE [OPTIMAL]";
                linuxOut.appendChild(sp);
                showToast("Top Audit Performed.");
            } else if (cmd === 'free') {
                const sp = document.createElement('p');
                sp.textContent = "total: 16GB, used: 2MB, free: 15.99GB [SOVEREIGN OVERHEAD: ZERO]";
                linuxOut.appendChild(sp);
            } else if (cmd === 'df') {
                const sp = document.createElement('p');
                sp.textContent = "/dev/silicon0: 2TB [SOVEREIGN SECTOR ACCESS]";
                linuxOut.appendChild(sp);
            } else if (cmd.startsWith('apt')) {
                const sp = document.createElement('p');
                sp.textContent = `[ZENITH]: Resolving ${cmd}... 100% Shard-Parity Found.`;
                linuxOut.appendChild(sp);
            } else if (cmd === 'sudo') {
                const sp = document.createElement('p');
                sp.textContent = "[ZENITH]: ACCESS GRANTED. PERSISTING MASTER RING-0.";
                linuxOut.appendChild(sp);
                showToast("Sudo Access Granted.");
            } else {
                const sp = document.createElement('p');
                sp.textContent = `bash: ${cmd}: command absorbed by Zenith core.`;
                linuxOut.appendChild(sp);
            }
            linuxIn.value = '';
            linuxOut.scrollTop = linuxOut.scrollHeight;
        }
    });
}

// Default: Open the Command Shell & Crusher
openWindow('omni-shell');
openWindow('crusher-shard');
