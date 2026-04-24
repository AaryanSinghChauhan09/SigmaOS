/**
 * SigmaOS Task Manager (v2.0)
 * Module 05: Real-time process monitoring backed by the live ProcessManager registry.
 *
 * Architecture Fix:
 *  - Replaced all hardcoded static process data with live ProcessManager.listProcesses().
 *  - Kill button now calls ProcessManager.neutralize(pid) — real cleanup.
 *  - Subscribes to process_registered / process_terminated for instant UI refresh.
 *  - Renders CPU% and memory from actual tracked values.
 *  - neutralizeNonEssential() wired to ProcessManager for emergency sweeps.
 */

const TaskManager = {
    intervalId: null,

    init() {
        console.log("Σ Task Manager v2.0: Live process registry connected.");
        this.updateProcessList();
        this.intervalId = setInterval(() => this.updateProcessList(), 3000);

        // Instant refresh on process lifecycle events
        if (window.EventBus) {
            EventBus.subscribe('process_registered', () => this.updateProcessList());
            EventBus.subscribe('process_terminated', () => this.updateProcessList());
        }
    },

    updateProcessList() {
        const container = document.getElementById('process-list');
        if (!container) return;

        const processes = window.ProcessManager
            ? ProcessManager.listProcesses()
            : [];

        if (processes.length === 0) {
            container.innerHTML = '<div class="process-row" style="color:rgba(255,255,255,0.4)">No active shards.</div>';
            return;
        }

        container.innerHTML = '';
        processes.forEach(p => {
            const uptime = Math.round((Date.now() - p.startTime) / 1000);
            const priorityLabel = ['HIGH', 'NORMAL', 'LOW'][p.priority] || '?';
            const stateColor = p.state === 'RUNNING'
                ? 'var(--acc-cyan)' : p.state === 'SUSPENDED'
                ? 'var(--acc-gold)' : 'var(--acc-magenta)';

            const row = document.createElement('div');
            row.className = 'process-row';
            row.innerHTML = `
                <span class="p-id" style="color:var(--acc-cyan)">PID:${p.pid}</span>
                <span class="p-name">${StringEngine ? StringEngine.truncate(p.shardName, 18) : p.shardName}</span>
                <span class="p-suite" style="color:rgba(255,255,255,0.5)">${p.suiteId}</span>
                <span class="p-cpu">${p.cpuUsage.toFixed(1)}%</span>
                <span class="p-mem">${StringEngine ? StringEngine.formatBytes(p.memUsageKB * 1024) : p.memUsageKB + 'KB'}</span>
                <span style="color:${stateColor};font-size:10px">${p.state}</span>
                <span style="color:rgba(255,255,255,0.4);font-size:10px">${uptime}s | PRI:${priorityLabel}</span>
                <button class="p-kill cyber-btn small-btn secondary">KILL</button>
            `;
            row.querySelector('.p-kill').onclick = () => this.killProcess(p.pid);
            container.appendChild(row);
        });

        // Update summary header if present
        const summary = document.getElementById('process-summary');
        if (summary && window.ProcessManager) {
            summary.textContent = `${processes.length} shards | CPU: ${ProcessManager.getTotalCPU()}% | MEM: ${StringEngine ? StringEngine.formatBytes(ProcessManager.getTotalMemKB() * 1024) : ProcessManager.getTotalMemKB() + 'KB'}`;
        }
    },

    killProcess(pid) {
        if (window.ProcessManager) {
            ProcessManager.neutralize(pid);
        } else {
            UIUtils.appendLog('audit-log', `Task Manager: PID:${pid} removed (no ProcessManager).`, 'warning');
        }
        this.updateProcessList();
    },

    neutralizeNonEssential() {
        if (window.ProcessManager) {
            ProcessManager.neutralizeNonEssential();
        }
        this.updateProcessList();
    }
};

window.TaskManager = TaskManager;
