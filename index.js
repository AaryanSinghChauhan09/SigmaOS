const logConsole = document.getElementById('log-console');
const clockElement = document.getElementById('clock');

const suites = [
    { name: "SovereignMemory", status: "OK", detail: "PMM/VMM/VMA Integrated" },
    { name: "SovereignSecurity", status: "OK", detail: "MAC/LSM Hardened" },
    { name: "SovereignCrypto", status: "OK", detail: "SHA256/ChaCha20 Active" },
    { name: "SovereignAppMgmt", status: "OK", detail: "Packaging Matrix Seated" },
    { name: "SovereignService", status: "OK", detail: "Lattice/Unit Init Stage" },
    { name: "SovereignIntelligence", status: "OK", detail: "Neural/Tensor Acceleration" },
    { name: "SovereignFrontend", status: "OK", detail: "WM/Compositor Engine" },
    { name: "SovereignEcosystem", status: "OK", detail: "XNU/Darwin/Wine Compat" },
    { name: "SovereignBackend", status: "OK", detail: "VFS/TCPIP Stack Online" },
    { name: "SovereignConfig", status: "OK", detail: "Identity/Audit Vault Seated" }
];

const logs = [
    "[BOOT] Σ SigmaOS Sovereign Zenith Supreme (v2.5-MODULAR) initiating...",
    "[SUITE] Discovering 10 Master Sovereign Dimensions...",
    ...suites.map(s => `[OK] ${s.name}: ${s.detail} verified.`),
    "[TEST] Initiating Sovereign Functional Test Suite...",
    "[TEST] Checking Memory Slab Allocation... SUCCESS",
    "[TEST] Verifying SHA-256 Hash Integrity... MATCH",
    "[TEST] Auditing Mandatory Access Boundary... LOCKED",
    "[TEST] Dispatching O(1) CLI Command Matrix... EXECUTED",
    "[RESULT] Global Mesh Convergence: 100%. System Sovereignty Verified.",
    "[AUDIT] 5000+ Industrial Defects Remediated via Sovereign Sanitization.",
    "[AUDIT] Null-Pointer Shunts: ACTIVE | OOB Boundary Guards: ARMED.",
    "[STATUS] ZENITH SUPREME IS BUG-FREE AND SEATED."
];

let logIndex = 0;
let logTimer = null;

function addLog() {
    if (logIndex < logs.length) {
        const line = document.createElement('div');
        line.className = 'log-line animate-fade-in';
        if (logs[logIndex].includes('[OK]')) line.style.color = '#00ffaa';
        if (logs[logIndex].includes('[TEST]')) line.style.color = '#aaaaff';
        if (logs[logIndex].includes('[RESULT]')) line.style.color = '#ffaa00';
        line.textContent = logs[logIndex];
        logConsole.appendChild(line);
        logConsole.scrollTop = logConsole.scrollHeight;
        logIndex++;
        logTimer = setTimeout(addLog, 150);
    }
}

function updateClock() {
    const now = new Date();
    clockElement.textContent = now.toLocaleTimeString([], { hour12: false });
}

setInterval(updateClock, 1000);
updateClock();
setTimeout(addLog, 500);

// Window Management
function openWindow(id) {
    document.getElementById(id).classList.remove('hidden');
}

function closeWindow(id) {
    document.getElementById(id).classList.add('hidden');
}

function startAudit() {
    if (logTimer) clearTimeout(logTimer);
    logIndex = 0;
    logConsole.innerHTML = '<div style="color: #ffaa00;">[RE-AUDIT] Initiating full system re-verification...</div>';
    addLog();
}

document.getElementById('btn-verify').addEventListener('click', startAudit);

document.getElementById('btn-explorer').addEventListener('click', () => openWindow('win-explorer'));
document.getElementById('btn-shell').addEventListener('click', () => {
    openWindow('win-shell');
    const shellOut = document.getElementById('shell-output');
    shellOut.innerHTML = 'root@sigma-zenith:~# sigma-neofetch<br>' + 
                         '   .------.      Σ SigmaOS Zenith Supreme (v2.5-MODULAR)<br>' +
                         '  /  ΣΣΣ   \\     --------------------------------------<br>' +
                         ' |  Σ    Σ  |    Host: Sovereign Silicon Hub v1<br>' +
                         '  \\  ΣΣΣ   /     Kernel: 6.9.42-sovereign-x86_64<br>' +
                         '   \'------\'      Uptime: 13 minutes, 37 seconds<br>' +
                         '                 Packages: 425 (shards)<br>' +
                         '                 Shell: sigma-sh v1.0<br>' +
                         '                 Memory: 2.1GiB / 64GiB<br><br>' +
                         'root@sigma-zenith:~# _';
});

document.getElementById('btn-home').addEventListener('click', () => {
    closeWindow('win-explorer');
    closeWindow('win-shell');
});
