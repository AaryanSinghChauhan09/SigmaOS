const logConsole = document.getElementById('log-console');
const clockElement = document.getElementById('clock');

const logs = [
    "[BOOT] Σ SigmaOS Zenith Supreme vROADMAP_1005 initiating...",
    "[ARCH] Sector 1: x86_64 / ARM64 / RISC-V Support Matrix: ONLINE",
    "[MEM]  Sector 2: Sovereign Slab & Page Allocation: SEATED",
    "[ABI]  Sector 3: Linux Syscall Sector & IPC Shards: ACTIVE",
    "[FS]   Sector 4: Sovereign VFS (Ext4/ZFS/NTFS) Shards: MOUNTED",
    "[NET]  Sector 5: Industrial TCP/IP & XDP Shunts: TRAFFIC_LOCK",
    "[COORD]Sector 6: MQ Scheduler & Sovereign Security Jails: HARDENED",
    "[INIT] Sector 7: PID-1 Service Orchestration: ACTIVATED",
    "[SYNC] All 425 Sovereign Shards converged. System Sovereignty Verified."
];

let logIndex = 0;

function addLog() {
    if (logIndex < logs.length) {
        const line = document.createElement('div');
        line.textContent = logs[logIndex];
        logConsole.appendChild(line);
        logIndex++;
        setTimeout(addLog, 400);
    }
}

function updateClock() {
    const now = new Date();
    clockElement.textContent = now.toLocaleTimeString([], { hour12: false });
}

setInterval(updateClock, 1000);
updateClock();
setTimeout(addLog, 1000);

// Interaction Logic
document.querySelectorAll('.taskbar-item').forEach(item => {
    item.addEventListener('click', () => {
        item.style.transform = 'scale(0.8)';
        setTimeout(() => {
            item.style.transform = '';
        }, 150);
    });
});
