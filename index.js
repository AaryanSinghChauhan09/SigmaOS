const logConsole = document.getElementById('log-console');
const clockElement = document.getElementById('clock');

const logs = [
    "[BOOT] Initializing Sovereign Kernel v1.0-RC1...",
    "[CORE] Loading 135 Integrated Distribution Shards...",
    "[SECURITY] W^X Memory Mitigation Active",
    "[SHARD] Solaris FMA Self-Healing: ONLINE",
    "[SHARD] Windows IOCP Completion: READY",
    "[SHARD] Android Binder Bridge: CONNECTED",
    "[SHARD] QNX Adaptive Partition: SEATED",
    "[BOOT] All 135 tests PASSED in 4.2ms",
    "[SYSTEM] SigmaOS Zenith Supreme Operational."
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
