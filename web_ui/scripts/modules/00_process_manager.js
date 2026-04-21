/**
 * SigmaOS Sovereign Process Manager
 * Module 00: Industrial-grade shard lifecycle and execution state orchestration.
 */

const ProcessManager = {
    activeShards: new Map(),

    registerShard(suiteId, shardName) {
        const pid = Math.floor(Math.random() * 10000);
        this.activeShards.set(pid, { suiteId, shardName, state: 'RUNNING', startTime: Date.now() });
        console.log(`Σ ProcessManager: Shard [${shardName}] registered with PID ${pid}`);
        return pid;
    },

    neutralize(pid) {
        if (this.activeShards.has(pid)) {
            const shard = this.activeShards.get(pid);
            this.activeShards.delete(pid);
            UIUtils.appendLog('audit-log', `Process: Neutralized shard [${shard.shardName}] (PID:${pid})`, 'warning');
            return true;
        }
        return false;
    },

    listProcesses() {
        return Array.from(this.activeShards.entries()).map(([pid, info]) => ({ pid, ...info }));
    }
};

window.ProcessManager = ProcessManager;
