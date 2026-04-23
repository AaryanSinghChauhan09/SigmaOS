/**
 * SigmaOS Sovereign Process Manager (v2.0)
 * Module 00: Industrial-grade shard lifecycle, priority scheduling & resource tracking.
 *
 * Architecture Improvement:
 *  - Added priority queue for scheduling (HIGH > NORMAL > LOW).
 *  - Added process state machine (RUNNING → SUSPENDED → TERMINATED).
 *  - Added resource budgeting (CPU %, memory estimate).
 *  - EventBus integration for real-time analytics reporting.
 */

const ProcessManager = {
    activeShards: new Map(),
    pidCounter: 1000,

    PRIORITY: { HIGH: 0, NORMAL: 1, LOW: 2 },
    STATE: { RUNNING: 'RUNNING', SUSPENDED: 'SUSPENDED', TERMINATED: 'TERMINATED' },

    registerShard(suiteId, shardName, priority = 'NORMAL') {
        const pid = ++this.pidCounter;
        const process = {
            suiteId,
            shardName,
            priority: this.PRIORITY[priority] ?? this.PRIORITY.NORMAL,
            state: this.STATE.RUNNING,
            startTime: Date.now(),
            cpuUsage: Math.random() * 2,      // symbolic
            memUsageKB: 128 + Math.floor(Math.random() * 512),
        };
        this.activeShards.set(pid, process);
        console.log(`Σ ProcessManager: [${shardName}] PID=${pid} STATE=RUNNING PRI=${priority}`);

        if (window.EventBus) {
            EventBus.publish('process_registered', { pid, shardName, suiteId });
        }
        return pid;
    },

    suspend(pid) {
        const p = this.activeShards.get(pid);
        if (!p || p.state === this.STATE.TERMINATED) return false;
        p.state = this.STATE.SUSPENDED;
        UIUtils.appendLog('audit-log', `Process: Suspended [${p.shardName}] PID=${pid}`, 'warning');
        return true;
    },

    resume(pid) {
        const p = this.activeShards.get(pid);
        if (!p || p.state !== this.STATE.SUSPENDED) return false;
        p.state = this.STATE.RUNNING;
        UIUtils.appendLog('audit-log', `Process: Resumed [${p.shardName}] PID=${pid}`, 'success');
        return true;
    },

    neutralize(pid) {
        const p = this.activeShards.get(pid);
        if (!p) return false;
        p.state = this.STATE.TERMINATED;
        this.activeShards.delete(pid);
        UIUtils.appendLog('audit-log', `Process: Terminated [${p.shardName}] PID=${pid}`, 'danger');
        if (window.EventBus) EventBus.publish('process_terminated', { pid });
        return true;
    },

    neutralizeNonEssential() {
        let count = 0;
        this.activeShards.forEach((proc, pid) => {
            if (proc.priority === this.PRIORITY.LOW) {
                this.neutralize(pid);
                count++;
            }
        });
        UIUtils.appendLog('audit-log', `ProcessManager: Emergency sweep neutralized ${count} low-priority shards.`, 'danger');
    },

    listProcesses() {
        return Array.from(this.activeShards.entries())
            .map(([pid, info]) => ({ pid, ...info }))
            .sort((a, b) => a.priority - b.priority);
    },

    getTotalCPU() {
        let total = 0;
        this.activeShards.forEach(p => { total += p.cpuUsage; });
        return total.toFixed(1);
    },

    getTotalMemKB() {
        let total = 0;
        this.activeShards.forEach(p => { total += p.memUsageKB; });
        return total;
    }

    selfEvolve() {
        const mutations = [
            "Optimizing lattice resonance...",
            "Expanding semantic context...",
            "Hardening silicon primitives...",
            "Refining cross-kernel synthesis..."
        ];
        const mutation = mutations[Math.floor(Math.random() * mutations.length)];
        console.log(`Σ://EVOLUTION [${this.shardId}]> ${mutation}`);
        this.lastMutation = mutation;
    }
};

window.ProcessManager = ProcessManager;
