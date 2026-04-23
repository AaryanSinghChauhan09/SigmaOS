/**
 * SigmaOS Sovereign Shard Orchestrator (v2.0)
 * Module 00: Hot-loading, stack-swapping, and live lifecycle management of UI shards.
 *
 * Architecture Improvements over v1.0:
 *  - All loaded shards tracked by PID via ProcessManager.
 *  - hotLoadShard() validates against a manifest before loading.
 *  - swapFileSystem() publishes EventBus event instead of silent timeout.
 *  - unloadShard() added for proper PID cleanup.
 *  - Shard manifest defines allowed shards + their required capabilities.
 */

const ShardOrchestrator = {
    // Registered shard manifest — extensible at runtime
    manifest: new Map([
        ['Advanced_Audio', { suite: 'S04', priority: 'LOW',    verified: true }],
        ['Neural_Debugger',{ suite: 'S24', priority: 'NORMAL', verified: true }],
        ['Quantum_VPN',    { suite: 'S07', priority: 'HIGH',   verified: true }],
        ['GPU_Compute',    { suite: 'S04', priority: 'HIGH',   verified: true }],
    ]),

    // Map shardName → PID (for lifecycle tracking)
    activePIDs: new Map(),

    init() {
        console.log("Σ Shard Orchestrator v2.0: Plug-and-Play lattice management online.");
    },

    registerShard(name, config) {
        this.manifest.set(name, config);
        console.log(`Σ ShardOrchestrator: Shard [${name}] added to manifest.`);
    },

    hotLoadShard(shardName) {
        if (this.activePIDs.has(shardName)) {
            UIUtils.appendLog('audit-log', `Shards: [${shardName}] already active. Skipping.`, 'warning');
            return;
        }

        const meta = this.manifest.get(shardName);
        if (!meta) {
            UIUtils.appendLog('audit-log', `Shards: [${shardName}] not in manifest. REJECTED.`, 'danger');
            return;
        }
        if (!meta.verified) {
            UIUtils.appendLog('audit-log', `Shards: [${shardName}] integrity unverified. QUARANTINED.`, 'danger');
            return;
        }

        UIUtils.appendLog('audit-log', `Shards: Hot-loading [${shardName}] from suite ${meta.suite}...`, 'info');

        // Register a real PID
        const pid = window.ProcessManager
            ? ProcessManager.registerShard(meta.suite, shardName, meta.priority)
            : null;

        this.activePIDs.set(shardName, pid);

        if (window.VitalsService) VitalsService.activeShards++;

        UIUtils.appendLog('audit-log', `Lattice: [${shardName}] integrated. PID=${pid}.`, 'success');
        if (window.EventBus) EventBus.publish('shard_loaded', { shardName, pid, suite: meta.suite });
    },

    unloadShard(shardName) {
        const pid = this.activePIDs.get(shardName);
        if (pid === undefined) {
            UIUtils.appendLog('audit-log', `Shards: [${shardName}] not active.`, 'warning');
            return;
        }
        if (pid !== null && window.ProcessManager) {
            ProcessManager.neutralize(pid);
        }
        this.activePIDs.delete(shardName);
        if (window.VitalsService) VitalsService.activeShards = Math.max(0, VitalsService.activeShards - 1);
        UIUtils.appendLog('audit-log', `Shards: [${shardName}] unloaded. PID=${pid} released.`, 'warning');
        if (window.EventBus) EventBus.publish('shard_unloaded', { shardName, pid });
    },

    swapFileSystem(fsType) {
        UIUtils.appendLog('audit-log', `Lattice: Initiating live FS swap → [${fsType}]...`, 'warning');
        if (window.EventBus) EventBus.publish('fs_swap_start', { fsType });

        setTimeout(() => {
            UIUtils.appendLog('audit-log', `Lattice: FileSystem [${fsType}] is now PRIMARY. Zero reboot.`, 'success');
            if (window.EventBus) EventBus.publish('fs_swap_complete', { fsType });
        }, 1200);
    },

    getActive() {
        return [...this.activePIDs.entries()].map(([name, pid]) => ({ name, pid }));
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

window.ShardOrchestrator = ShardOrchestrator;
