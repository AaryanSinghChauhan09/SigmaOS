const COMMAND_RESPONSES = {
    'HELP': [
        "AVAILABLE ZENITH SHARDS:",
        "- FORK_TEST: Demonstrate xv6 process duplication.",
        "- DMA_CMD: Execute Silberschatz-grade DMA transfer.",
        "- PETERSON: Coordinate Peterson's critical section.",
        "- SCHEDULER: Execute O(1) MLFQ balancing (MIT).",
        "- CLOUD_FORGE: Forge elastic VPC shard (AWS).",
        "- SYSTEM_STATUS: Query silicon-direct registry."
    ],
    'FORK_TEST': [
        "[ZENITH-PIPE]: Forging native pipe shard...",
        "[CHILD]: I am the sovereign child. Executing XV6 Shard...",
        "[PARENT]: Child spawned (PID: 1024). Waiting for shard completion...",
        "[PARENT]: Child shard re-absorbed."
    ],
    'DMA_CMD': [
        "[ZENITH-HARDWARE]: Initiating DMA Transfer (4096 bytes). Bypassing CPU...",
        "[OK]: Block transfer complete. Host notified via silicon pulse."
    ],
    'PETERSON': [
        "[ZENITH-PETERSON]: CRITICAL SECTION ENTRY (Thread 0).",
        "[ZENITH-SYNC]: Readers-Writers priority logic initiated (Zero-Starvation)."
    ],
    'SYSTEM_STATUS': [
        "KERNEL: RING-0 (ZENITH)",
        "SYSCALLS: 256 DIRECT (SHARDED)",
        "MEMORY: HIERARCHICAL PAGING ACTIVE",
        "SOVEREIGNTY: 100%"
    ]
};

const MASTER_SHARDS = [
    "SovereignSuperCalculator.cpp", "SovereignML.cpp", "SovereignTranspilerZenith.cpp",
    "SovereignNetMesh.cpp", "SovereignWebBridge.cpp", "SovereignOmniTool.cpp",
    "SovereignAppStore.cpp", "SovereignVoiceShard.cpp", "SovereignAutomationShard.cpp",
    "SovereignHypervisorZenith.cpp", "SovereignDiagnosticsZenith.cpp", "SovereignEncyclopedia.cpp",
    "SovereignXV6Bridge.cpp", "SovereignKnowledgeAudit.cpp", "SovereignZenithComplete.cpp",
    "SovereignLibC.asm", "SovereignKernelFinality.asm", "SigmaOOP.hpp", "SigmaRustCore.rs"
];

// Window Management
let maxZ = 10;
function openWindow(id) {
    const win = document.getElementById(id);
    const task = document.getElementById(`task-${id}`);
    if (win) {
        win.classList.remove('hidden');
        win.style.zIndex = ++maxZ;
    }
    if (task) task.classList.remove('hidden');
}

function closeWindow(id) {
    const win = document.getElementById(id);
    const task = document.getElementById(`task-${id}`);
    if (win) win.classList.add('hidden');
    if (task) task.classList.add('hidden');
}

function focusWindow(id) {
    const win = document.getElementById(id);
    if (win) win.style.zIndex = ++maxZ;
}

function dragWindow(e, id) {
    const win = document.getElementById(id);
    let offsetX = e.clientX - win.offsetLeft;
    let offsetY = e.clientY - win.offsetTop;
    win.style.zIndex = ++maxZ;

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

// Terminal Logic
const output = document.getElementById('output');
const input = document.getElementById('command-input');

function addLine(text, className = '') {
    const p = document.createElement('p');
    p.classList.add('line');
    if (className) p.classList.add(className);
    p.textContent = text;
    if (output) {
        output.appendChild(p);
        output.scrollTop = output.scrollHeight;
    }
}

if (input) {
    input.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') {
            const cmd = input.value.trim().toUpperCase();
            addLine(`Σ://zenith> ${cmd}`, 'prompt');
            
            if (COMMAND_RESPONSES[cmd]) {
                COMMAND_RESPONSES[cmd].forEach(line => addLine(line));
            } else if (cmd !== '') {
                addLine(`[ERROR]: Unknown Shard '${cmd}'. Intent discarded.`);
            }
            
            input.value = '';
        }
    });
}

// Clock Logic
function updateClock() {
    const clock = document.getElementById('clock');
    if (clock) {
        const now = new Date();
        clock.textContent = now.getHours().toString().padStart(2, '0') + ":" + now.getMinutes().toString().padStart(2, '0');
    }
}
setInterval(updateClock, 1000);
updateClock();

// Default: Open the Shell
openWindow('omni-shell');

// Inject Catalog
function initCatalog() {
    const catalog = document.getElementById('catalog-content');
    if (catalog) {
        catalog.innerHTML = '';
        MASTER_SHARDS.forEach(shard => {
            const line = document.createElement('div');
            line.style.padding = '5px 0';
            line.innerHTML = `<span style="color:#00f2ff">[ACTIVE]</span> ${shard}`;
            catalog.appendChild(line);
        });
    }
}
initCatalog();

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
