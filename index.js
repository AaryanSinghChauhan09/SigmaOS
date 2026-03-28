const COMMAND_RESPONSES = {
    'HELP': [
        "SUPREME ZENITH COMMANDS:",
        "- FORK_TEST: Demonstrate xv6 silicon process duplication.",
        "- DMA_CMD: Execute Silberschatz-grade host bus transfer.",
        "- PETERSON: Coordinate Peterson's critical section lock.",
        "- ABSORB_LEGACY: Unify v1.0-v92.0 features into Zenith core.",
        "- SYSTEM_STATUS: Query silicon-direct registry.",
        "- MASTER_AUDIT: Perform bit-level shard verification."
    ],
    'FORK_TEST': ["[ZENITH]: Duplicating Shard 0x93...", "[CHILD]: Sovereign Child Active.", "[PARENT]: Child re-absorbed."],
    'DMA_CMD': ["[ZENITH-DMA]: Initiating host-bus transfer (4096 bytes).", "[OK]: Success."],
    'PETERSON': ["[ZENITH-SYNC]: Thread-0 Entry. Lock secured via Peterson's Shard."],
    'SYSTEM_STATUS': ["BUILD: v93.0 SUPREME", "SOVEREIGNTY: 100%", "THREAT-LEVEL: ZERO", "SHARD-PARITY: MASTER"],
    'MASTER_AUDIT': ["[AUDIT]: Verifying 91+ legacy shards...", "[OK]: 100% integrity across all versions."]
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

// Clock & Audit Logic
function updateClock() {
    const clock = document.getElementById('clock');
    if (clock) {
        const now = new Date();
        clock.textContent = now.getHours().toString().padStart(2, '0') + ":" + now.getMinutes().toString().padStart(2, '0');
    }
}
setInterval(updateClock, 1000);
updateClock();

function animateSiliconPulse() {
    const eax = document.getElementById('reg-eax');
    const ebx = document.getElementById('reg-ebx');
    if (eax && ebx) {
        eax.style.width = (Math.random() * 80 + 20) + '%';
        ebx.style.width = (Math.random() * 80 + 20) + '%';
    }
}
setInterval(animateSiliconPulse, 500);

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

// Default: Open the Command Shell & Crusher
openWindow('omni-shell');
openWindow('crusher-shard');
