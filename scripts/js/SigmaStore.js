"use strict";

/**
 * Σ SHARD STORE
 * Autonomy management and tool orchestration.
 */
export const SOVEREIGN_SHARDS = [
    { id: 'terminal', name: 'Sigma Shell', icon: '🐚', domain: 'System', enabled: true, description: 'Core system command line interface.' },
    { id: 'shardmanager', name: 'Shard Store', icon: '🧩', domain: 'System', enabled: true, description: 'Full autonomy over tools & performance.' },
    { id: 'sysinfo', name: 'Metrics Hub', icon: '📊', domain: 'System', enabled: true, description: 'Real-time performance telemetry.' },
    { id: 'devshard', name: 'Dev IDE', icon: '👨‍💻', domain: 'Dev', enabled: true, description: 'C11/ASM IDE - Pure performance.' },
    { id: 'aishard', name: 'AI Lab', icon: '🧠', domain: 'AI', enabled: true, description: 'Gradient Descent & Tensor flow kernels.' },
    { id: 'dsshard', name: 'Data Sci', icon: '📉', domain: 'DS', enabled: true, description: 'Statistical analysis & Real-time math.' },
    { id: 'dsashard', name: 'DSA Viz', icon: '🧮', domain: 'DSA', enabled: true, description: 'Algorithm auditing & Real-performance sorting.' },
    { id: 'cybershard', name: 'Cyber Sec', icon: '🐲', domain: 'CS', enabled: true, description: 'Zero-trust audit & VFS path security.' },
    { id: 'mlshard', name: 'ML Ops', icon: '🚀', domain: 'ML', enabled: true, description: 'Deployment pipelines & Feature engineering.' },
    { id: 'financeshard', name: 'Finance', icon: '💰', domain: 'Finance', enabled: true, description: 'Industrial Finance - Stock & Crypto Matrix.' },
    { id: 'contentshard', name: 'Studio', icon: '🎬', domain: 'Media', enabled: true, description: 'Creative media studio - Direct Rendering.' },
    { id: 'productivity', name: 'Tasks', icon: '✅', domain: 'Productivity', enabled: true, description: 'Industrial Task Orchestration.' },
    { id: 'distromirror', name: 'Mirror', icon: '🐧', domain: 'System', enabled: true, description: 'Linux Distro principles mirroring.' },
    { id: 'uxaudit', name: 'UX Audit', icon: '📐', domain: 'Design', enabled: true, description: 'UI/UX principle auditing & consistency.' },
    { id: 'mediahub', name: 'Media Hub', icon: '🖼️', domain: 'Media', enabled: false, description: 'Professional media manipulation tools.' },
    { id: 'planmaster', name: 'Plan Master', icon: '📅', domain: 'Management', enabled: false, description: 'Sovereign project orchestration.' },
    { id: 'vfsmanager', name: 'VFS Admin', icon: '💾', domain: 'System', enabled: false, description: 'Manage raw silicon storage blocks.' },
    { id: 'netviz', name: 'Net Shard', icon: '🌐', domain: 'Network', enabled: false, description: 'Visualize real local network topography.' },
    { id: 'principles', name: 'Principles', icon: '⚖️', domain: 'System', enabled: true, description: 'Sovereign OS Manifest & Performance USPs.' }
];

export class SigmaStore {
    constructor(system) {
        this.system = system;
        this.shards = JSON.parse(localStorage.getItem('SOVEREIGN_SHARDS')) || SOVEREIGN_SHARDS;
        this.save();
    }

    save() {
        localStorage.setItem('SOVEREIGN_SHARDS', JSON.stringify(this.shards));
    }

    toggle(id) {
        const shard = this.shards.find(s => s.id === id);
        if (!shard) return;
        shard.enabled = !shard.enabled;
        this.save();
        this.system.renderShardManager();
        this.system.renderMenu();
        this.system.spawnToast(`Shard [${id}] ${shard.enabled ? 'Enabled' : 'Disabled'}.`);
        if (!shard.enabled) this.system.wm.close(id);
    }

    purge() {
        this.shards = this.shards.filter(s => s.enabled || s.id === 'shardmanager');
        this.save();
        this.system.renderShardManager();
        this.system.renderMenu();
        this.system.spawnToast('AUTONOMY TRIGGERED: Unused Shards Purged from Silicon.');
    }
}
