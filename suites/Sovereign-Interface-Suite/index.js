// $ZENITH Sovereign Wrapper (Zero-Dependency)
const $ZENITH = (id) => document.getElementById(id);

const COMMAND_RESPONSES = {
    'HELP': [
        "Σ SIGMAOS INDUSTRIAL COMMANDS:",
        "- SUITE_AUDIT: Perform bit-level verification of the 10 Master Suites.",
        "- SOVEREIGN_INIT: Re-mount all silicon-native shards.",
        "- LATTICE_LOCK: Rotate PQC entropy keys.",
        "- AETHER_SYNC: Synchronize mesh nodes across all distributed sites.",
        "- ZENITH_VERSION: Display architectural finality data.",
        "- SHARD_PURGE: Neutralize dross and legacy dependencies."
    ],
    'SUITE_AUDIT': ["[AUDIT]: Verifying 10 Master Suites...", "[OK]: 100% integrity. SigmaSovereign verified."],
    'SOVEREIGN_INIT': ["[ZENITH]: Re-mounting Industrial Shards...", "[OK]: Kernel, Hardware, and Aether Suites active."],
    'LATTICE_LOCK': ["[PQC]: Rotating KYBER-V5 entropy shards...", "[OK]: Quantum immunity refreshed."],
    'AETHER_SYNC': ["[MESH]: Syncing with 4 nodes...", "[OK]: Aether consistency reached."],
    'ZENITH_VERSION': ["BUILD: v94.0 SUPREME INDUSTRIAL", "ARCH: Σ-ZENITH-SHARD", "SOVEREIGNTY: 100% (Bit-Perfect)"],
    'SHARD_PURGE': ["[PURGE]: Analyzing dependencies...", "[OK]: Zero dependencies detected. Pure C11 context maintained."]
};

const SOVEREIGN_SUITES = {
    'kernel': ["SovereignAIKernelZenith.c", "SovereignMemoryZenith.c", "SovereignShardKernel.c", "slab.c", "SovereignLibC.c", "sigma_libc.h", "SovereignSyncZenith.h"],
    'hardware': ["arch/x86_64", "drivers/net", "drivers/gpu", "SovereignStandardHAL.asm", "SovereignHardwareIOZenith.h"],
    'storage': ["fs/SovereignVFS.c", "fs/SovereignBlock.c", "fs/SovereignDisk.c"],
    'network': ["SovereignAetherOrchestrator.c", "SovereignWebBridge.c", "SovereignNetMesh.c", "SovereignCloudMaestro.c"],
    'security': ["SovereignLatticePQC.c", "SovereignForensicMatrix.c", "SovereignIAM.c"],
    'ai': ["SovereignML.c", "SovereignConceptGenerator.c", "SovereignVoiceShard.c", "SovereignReasoning.c"],
    'analytical': ["SovereignSuperCalculator.c", "SovereignDataPreprocess.c", "SovereignResearchMatrix.c", "SovereignScienceZenith.c"],
    'knowledge': ["SovereignEncyclopedia.c", "SovereignNcertUnity.c", "SovereignScholasticDB.c", "SovereignKnowledgeAudit.c"],
    'interface': ["index.html", "index.css", "index.js", "SovereignGraphicsCompositor.c", "SovereignShell.c"],
    'industrial': ["SovereignBuildSystem.c", "SovereignContainerForge.c", "SovereignAutomationShard.c", "SovereignDistroForge.c"]
};

// Supremacy State
let maxZ = 100;

function openSuite(suiteId) {
    const suiteName = suiteId.toUpperCase() + " MASTER SUITE";
    const files = SOVEREIGN_SUITES[suiteId] || [];
    
    // Update Shard Audit window
    const catalog = document.getElementById('catalog-content');
    const catalogTitle = document.querySelector('#catalog-shard .title');
    if (catalogTitle) catalogTitle.textContent = `Σ ${suiteName} [SOVEREIGN AUDIT]`;
    
    if (catalog) {
        catalog.innerHTML = '';
        files.forEach(file => {
            const div = document.createElement('div');
            div.className = 'shard-item';
            div.style.padding = '8px 0';
            div.innerHTML = `<span style="color:var(--accent-primary)">STATUS: OPTIMAL</span> | <span>${file}</span>`;
            catalog.appendChild(div);
        });
    }
    
    openWindow('catalog-shard');
    showToast(`${suiteName} Shards Mounted.`);
}

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
    const term = document.getElementById('output');
    if (!term) return;
    openWindow('omni-shell');
    
    const p = document.createElement('p');
    p.className = 'line text-neon-gold';
    p.textContent = `Σ://macro> Pushing Shard: ${name}...`;
    term.appendChild(p);

    // Simulated Silicon Latency
    let i = 0;
    const interval = setInterval(() => {
        const dot = document.createElement('span');
        dot.textContent = '.';
        p.appendChild(dot);
        if (++i > 5) {
            clearInterval(interval);
            const ok = document.createElement('p');
            ok.className = 'line text-neon-green';
            ok.textContent = `[OK]: ${name} merged into Zenith context.`;
            term.appendChild(ok);
        }
    }, 100);
}

// Removed legacy catalog init in favor of Suite Master logic.

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

function execScript(name) {
    openWindow('omni-shell');
    addLine(`Σ://automation> Running script: ${name}...`, 'text-neon-gold');
    setTimeout(() => {
        addLine(`[OK]: ${name} completed successfully. Shard integrity pinned.`, 'text-neon-green');
    }, 1500);
}

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
