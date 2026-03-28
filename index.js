const output = document.getElementById('output');
const input = document.getElementById('command-input');

const COMMAND_RESPONSES = {
    'HELP': [
        "AVAILABLE ZENITH SHARDS:",
        "- FORK_TEST: Demonstrate xv6 process duplication.",
        "- DMA_CMD: Execute Silberschatz-grade DMA transfer.",
        "- PETERSON: Coordinate Peterson's critical section.",
        "- SCHEDULER: Execute O(1) MLFQ balancing (MIT).",
        "- CLOUD_FORGE: Forge elastic VPC shard (AWS).",
        "- SYSTEM_STATUS: Query silicon-direct registry.",
        "- ABOUT: System philosophy and academic parity."
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
    'SCHEDULER': [
        "[ZENITH-SCHED]: Balancing MLFQ Queues (MIT/IITB parity).",
        "[OK]: All shards re-prioritized."
    ],
    'ABOUT': [
        "Σ SIGMAOS ZENITH v93.0",
        "A bit-perfect, zero-dependency environment for absolute system sovereignty.",
        "Industrial Parity: Silberschatz, Tanenbaum, AWS, Cisco.",
        "Academic Parity: MIT, Stanford, IIT Bombay, xv6, OSTEP."
    ],
    'SYSTEM_STATUS': [
        "KERNEL: RING-0 (ZENITH)",
        "SYSCALLS: 256 DIRECT (SHARDED)",
        "MEMORY: HIERARCHICAL PAGING ACTIVE",
        "SOVEREIGNTY: 100%"
    ]
};

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

// Focus terminal on any click
document.body.addEventListener('click', () => input.focus());
