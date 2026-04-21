/**
 * SigmaOS Enterprise Bootloader (v1.0)
 * Architecture: Object-Oriented Shard Orchestration
 * Compliance: Any Chromium-based browser (v121+)
 */

class ShardOrchestrator {
    constructor() {
        this.registry = new Map();
        this.maxZ = 1000;
        this.initialized = false;
    }

    async boot() {
        console.log("Σ://BOOT> Initiating ring-0 bootstrap sequence...");
        this.setupEventListeners();
        this.initHeartbeat();
        this.initialized = true;
        
        // Auto-open primary diagnostic shells
        this.openWindow('omni-shell');
        this.openWindow('crusher-shard');
        
        console.log("Σ://BOOT> Shard Parity: MASTER. Sovereignty: 100%.");
    }

    setupEventListeners() {
        // Universal delegated events for better performance (DRY)
        document.addEventListener('click', (e) => {
            const toggle = e.target.closest('[data-toggle-window]');
            if (toggle) {
                this.openWindow(toggle.dataset.toggleWindow);
            }

            const close = e.target.closest('[data-close-window]');
            if (close) {
                this.closeWindow(close.dataset.closeWindow);
            }
        });
    }

    openWindow(id) {
        const win = document.getElementById(id);
        if (!win) return;
        win.classList.remove('hidden');
        win.style.zIndex = ++this.maxZ;
        
        const task = document.getElementById(`task-${id}`);
        if (task) {
            task.classList.remove('hidden');
            task.classList.add('active');
        }
    }

    closeWindow(id) {
        const win = document.getElementById(id);
        if (win) win.classList.add('hidden');
        
        const task = document.getElementById(`task-${id}`);
        if (task) {
            task.classList.add('hidden');
            task.classList.remove('active');
        }
    }

    initHeartbeat() {
        // High-performance clock & silicon pulse using RAF
        const pulse = () => {
            this.updateClock();
            this.animateSilicon();
            requestAnimationFrame(() => setTimeout(pulse, 1000));
        };
        pulse();
    }

    updateClock() {
        const clock = document.getElementById('clock');
        if (clock) {
            const now = new Date();
            clock.textContent = now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false });
        }
    }

    animateSilicon() {
        const eax = document.getElementById('reg-eax');
        const ebx = document.getElementById('reg-ebx');
        if (eax && ebx) {
            eax.style.width = `${Math.floor(Math.random() * 60 + 20)}%`;
            ebx.style.width = `${Math.floor(Math.random() * 60 + 20)}%`;
        }
    }

    // Dynamic Shard Discovery
    execScript(name) {
        const term = document.getElementById('output');
        if (!term) return;
        
        const p = document.createElement('p');
        p.className = 'line highlight-gold';
        p.textContent = `Σ://macro> Pushing Shard: ${name}...`;
        term.appendChild(p);

        setTimeout(() => {
            const ok = document.createElement('p');
            ok.className = 'line highlight-cyan';
            ok.textContent = `[OK]: ${name} merged into Zenith context.`;
            term.appendChild(ok);
            term.scrollTop = term.scrollHeight;
        }, 1200);
    }
}

// Instantiate the Kernel Orchestrator
window.Orchestrator = new ShardOrchestrator();
document.addEventListener('DOMContentLoaded', () => window.Orchestrator.boot());
