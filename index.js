const COMMAND_RESPONSES = {
    'HELP': [
        "AVAILABLE ZENITH SHARDS:",
        "- FORK_TEST: Demonstrate xv6 process duplication.",
        "- DMA_CMD: Execute Silberschatz-grade DMA transfer.",
        "- PETERSON: Coordinate Peterson's critical section.",
        "- SCHEDULER: Execute O(1) MLFQ balancing (MIT).",
        "- CLOUD_FORGE: Forge elastic VPC shard (AWS).",
        "- TOGGLE_GUI: Re-initialize Sovereign Workspace.",
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

// Window Management
function openWindow(id) {
    const win = document.getElementById(id);
    const task = document.getElementById(`task-${id}`);
    if (win) win.style.display = 'flex';
    if (task) task.style.display = 'block';
}

function closeWindow(id) {
    const win = document.getElementById(id);
    const task = document.getElementById(`task-${id}`);
    if (win) win.style.display = 'none';
    if (task) task.style.display = 'none';
}

function dragWindow(e, id) {
    const win = document.getElementById(id);
    let offsetX = e.clientX - win.offsetLeft;
    let offsetY = e.clientY - win.offsetTop;

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
    output.appendChild(p);
    output.scrollTop = output.scrollHeight;
}

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

// Clock Logic
function updateClock() {
    const now = new Date();
    document.getElementById('clock').textContent = now.getHours().toString().padStart(2, '0') + ":" + now.getMinutes().toString().padStart(2, '0');
}
setInterval(updateClock, 1000);
updateClock();

// Default: Open the Shell
openWindow('omni-shell');
