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
    { id: 'oopsshard', name: 'OOPS Audit', icon: '🧱', domain: 'System', enabled: true, description: 'Codebase inheritance and encapsulation audit.' },
    { id: 'networkshard', name: 'Net Shard', icon: '📡', domain: 'Network', enabled: false, description: 'Industrial network latency and topology.' },
    { id: 'automationshard', name: 'Automation', icon: '🤖', domain: 'System', enabled: true, description: 'Sovereign task scheduling and headless scripting.' },
    { id: 'personalshard', name: 'Zenith Styles', icon: '🎨', domain: 'Design', enabled: true, description: 'Absolute UI personalization and aesthetics.' },
    { id: 'amnesicshard', name: 'Amnesic Mode', icon: '🧼', domain: 'Security', enabled: false, description: 'Forensic-grade memory scrubbing and data-scrub.' },
    { id: 'pqcshard', name: 'Quantum Shard', icon: '⚛️', domain: 'Security', enabled: true, description: 'Post-Quantum LWE Lattice cryptography.' },
    { id: 'ledger-shard', name: 'Sovereign Ledger', icon: '⛓️', domain: 'Network', enabled: false, description: 'Peer-to-Peer sharded consensus ledger.' },
    { id: 'bioshard', name: 'Genomics', icon: '🧬', domain: 'Science', enabled: false, description: 'Bio-Informatics Sequence Alignment (Needleman-Wunsch).' },
    { id: 'llmshard', name: 'Transformer', icon: '🧠', domain: 'AI', enabled: true, description: 'Local-first matrix operations for LLM attention.' },
    { id: 'hftshard', name: 'HFT Oracle', icon: '📈', domain: 'Finance', enabled: false, description: 'Zero-latency High-Frequency Trading quantitative math.' },
    { id: 'coworkshard', name: 'Co-Work IPC', icon: '🤝', domain: 'Agent', enabled: true, description: 'Multi-Agent Local Silicon Collaboration.' },
    { id: 'oracleshard', name: 'Compute Oracle', icon: '🔮', domain: 'Agent', enabled: false, description: 'Silicon Knowledge Graph (Perplexity Eq).' },
    { id: 'clawshard', name: 'Macro Claw', icon: '🦾', domain: 'Agent', enabled: true, description: 'Hardware-level Cursor & Automation Driver.' },
    { id: 'qubesshard', name: 'Hypervisor Bounds', icon: '🧊', domain: 'Security', enabled: true, description: 'Virtual Machine Compartmentalization (Qubes OS Eq).' },
    { id: 'timeshard', name: 'Time Machine', icon: '⏳', domain: 'System', enabled: false, description: 'Delta-block Incremental Backups (macOS Eq).' },
    { id: 'tailshard', name: 'Onion Router', icon: '🧅', domain: 'Network', enabled: false, description: 'Deep Web 3-Node Obfuscation (Tails OS Eq).' },
    { id: 'plan9shard', name: '9P Network', icon: '🌌', domain: 'Network', enabled: false, description: 'Everything-is-a-file 9P Protocol (Plan 9 Eq).' },
    { id: 'holyshard', name: 'Ring-0 Oracle', icon: '✝️', domain: 'System', enabled: false, description: 'Direct unabstracted hardware mapping (TempleOS Eq).' },
    { id: 'kalishard', name: 'Pen-Test Map', icon: '🐉', domain: 'Security', enabled: true, description: 'Low-level Nmap port enumeration (Kali OS Eq).' },
    { id: 'ebpfshard', name: 'eBPF Sandbox', icon: '🐝', domain: 'Kernel', enabled: false, description: 'Ring-0 isolated byte-code verifier (Linux Kernel Eq).' },
    { id: 'cgroupshard', name: 'C-Groups', icon: '🗜️', domain: 'Kernel', enabled: true, description: 'Strict hardware allocation constraints (Linux Kernel Eq).' },
    { id: 'oomshard', name: 'OOM Grim Reaper', icon: '🪦', domain: 'Kernel', enabled: false, description: 'Memory-exhaustion sacrificial heuristic (Linux Kernel Eq).' },
    { id: 'vfsmanager', name: 'VFS Admin', icon: '💾', domain: 'System', enabled: false, description: 'Manage raw silicon storage blocks.' },
    { id: 'netviz', name: 'Net Shard', icon: '🌐', domain: 'Network', enabled: false, description: 'Visualize real local network topography.' },
    { id: 'principles', name: 'Principles', icon: '⚖️', domain: 'System', enabled: true, description: 'Sovereign OS Manifest & Performance USPs.' }
];

export const DISTROS = [
    { id: 'ubuntu', name: 'Ubuntu Lunar', icon: '🟠', info: 'LTS Industrial Core', url: 'https://copy.sh/v86/?profile=ubuntu' },
    { id: 'arch', name: 'Arch Linux', icon: '🔵', info: 'Sovereign Rolling Release', url: 'https://copy.sh/v86/?profile=archlinux' },
    { id: 'debian', name: 'Debian 12', icon: '🔴', info: 'The Universal OS Shard', url: 'https://copy.sh/v86/?profile=debian' },
    { id: 'opensuse', name: 'openSUSE Tumbleweed', icon: '🟢', info: 'Professional Stability Shard', url: 'https://copy.sh/v86/?profile=opensuse' },
    { id: 'almalinux', name: 'AlmaLinux 9', icon: '⚪', info: 'Community Enterprise Grade', url: 'https://copy.sh/v86/?profile=almalinux' },
    { id: 'rocky', name: 'Rocky Linux 9', icon: '🛡️', info: 'RHEL-Compatible Master', url: 'https://copy.sh/v86/?profile=rocky' },
    { id: 'alpine', name: 'Alpine Linux', icon: '🏔️', info: 'Security-oriented Shard', url: 'https://copy.sh/v86/?profile=alpine' },
    { id: 'gentoo', name: 'Gentoo Linux', icon: '🟣', info: 'Source-based Sovereignty', url: 'https://copy.sh/v86/?profile=gentoo' },
    { id: 'fedora', name: 'Fedora Workstation', icon: '🧢', info: 'Cutting-Edge Sharding', url: 'https://copy.sh/v86/?profile=fedora' },
    { id: 'custom', name: 'Custom ISO/Disk', icon: '💿', info: 'Universal Shard Loader', url: '' }
];

export const MATRIX_TOOLS = [
    { id: 'xclicker', name: 'Sigma XClicker', desc: 'Sovereign Auto-Clicking logic.', icon: '🖱️', USP: 'robiot/xclicker' },
    { id: 'autokey', name: 'Sigma AutoKey', desc: 'Industrial Macro Automation.', icon: '⌨️', USP: 'famousshea/autokey' },
    { id: 'merlin_ia', name: 'Sovereign AI Shard', desc: 'Autonomous system balancing.', icon: '🤖', USP: 'N1ghthill/merlin-ia' },
    { id: 'cloud_provision', name: 'vSphere Provisioner', desc: 'Industrial Infrastructure Sharding.', icon: '☁️', USP: 'miladhzzzz/vsphere-infra' },
    { id: 'script_master', name: 'Automation Playbook', desc: 'Universal Bash/Python Matrix.', icon: '📜', USP: 'muhibarshad/Linux-Automation-Scripts' },
    { id: 'ai_orchestrator', name: 'Aether Orchestrator', desc: 'Multi-model AI sharding.', icon: '🤖', USP: 'AI-Orchestrator-v2.0' },
    { id: 'spectrum_terminal', name: 'Spectrum AI Shell', desc: 'Neural command prediction.', icon: '⚡', USP: 'Spectrum-Terminal-V18' }
];

export class SigmaStore {
    constructor(system) {
        this.system = system;
        this.shards = JSON.parse(localStorage.getItem('SOVEREIGN_SHARDS')) || SOVEREIGN_SHARDS;
        this.distros = DISTROS;
        this.matrixTools = MATRIX_TOOLS;
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
