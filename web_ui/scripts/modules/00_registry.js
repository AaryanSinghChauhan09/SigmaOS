/**
 * SigmaOS Sovereign Registry
 * Module 00: Centralized metadata and lattice configuration.
 */

const SovereignRegistry = {
    suites: [
        { id: "S01", name: "Genesis", icon: "🧬", desc: "Core foundation and boot finality." },
        { id: "S02", name: "ZenithUI", icon: "💎", desc: "Premium high-fidelity interface." },
        { id: "S03", name: "Orchestrator", icon: "🎼", desc: "Master process synchronization." },
        { id: "S04", name: "HAL", icon: "🔌", desc: "Unified hardware abstraction (SUDI)." },
        { id: "S05", name: "Memory", icon: "🧠", desc: "Predictive memory compaction." },
        { id: "S06", name: "Storage", icon: "💾", desc: "NVMe-accelerated storage lattice." },
        { id: "S07", name: "Network", icon: "🌐", desc: "Sovereign protocol stacks." },
        { id: "S08", name: "Security", icon: "🛡️", desc: "Predictive AI-driven firewall." },
        { id: "S09", name: "Intelligence", icon: "👁️", desc: "Neural pattern recognition." },
        { id: "S10", name: "Registry", icon: "📖", desc: "Universal object storage." },
        { id: "S11", name: "Virtualization", icon: "📦", desc: "Zero-overhead isolation." },
        { id: "S12", name: "Ecosystem", icon: "🍀", desc: "Native productivity and apps." },
        { id: "S13", name: "Sentience", icon: "✨", desc: "Self-aware kernel orchestration." },
        { id: "S14", name: "Transcendence", icon: "🚀", desc: "Beyond-classical computation." },
        { id: "S15", name: "DevNexus", icon: "🛠️", desc: "Industrial-grade developer tools." },
        { id: "S16", name: "SoulMolding", icon: "🕯️", desc: "Personalized silicon identity." },
        { id: "S17", name: "BioNexus", icon: "🧬", desc: "Real-time biometric health sync." },
        { id: "S18", name: "QuantumLink", icon: "⚛️", desc: "Unbreakable QKD encryption." },
        { id: "S19", name: "SelfEvolution", icon: "🔄", desc: "Autonomous self-healing shards." },
        { id: "S20", name: "Interconnect", icon: "🔗", desc: "Infinite lattice connectivity." },
        { id: "S21", name: "EternalState", icon: "⏳", desc: "Hyper-stable system persistence." },
        { id: "S22", name: "SimulationNexus", icon: "🎮", desc: "Physics-correct world simulation." },
        { id: "S23", name: "OmniNexus", icon: "🌌", desc: "Cross-dimensional hardware links." },
        { id: "S24", name: "GlobalDebugger", icon: "🐞", desc: "Pan-galactic fault detection." },
        { id: "S25", name: "ZeroKernel", icon: "⚪", desc: "Zero-latency core execution." },
        { id: "S26", name: "OmniFabric", icon: "🕸️", desc: "16Tb/s fabric interconnect." },
        { id: "S27", name: "NeuralLink", icon: "🧠", desc: "Direct-to-silicon brain bypass." },
        { id: "S28", name: "OmniBus", icon: "🚌", desc: "Universal data transport." },
        { id: "S29", name: "LatticeMerge", icon: "🧪", desc: "Unification of all local shards." },
        { id: "S30", name: "Supremacy", icon: "👑", desc: "Master architectural finality." },
        { id: "S31", name: "GlobalGovernance", icon: "🏛️", desc: "Distributed consensus engine." },
        { id: "S32", name: "UnifiedSovereignty", icon: "🤝", desc: "Cross-OS assimilation layer." },
        { id: "S33", name: "TerminalFulfillment", icon: "🏁", desc: "Closure of the eternal loop." }
    ],

    getSuiteById(id) {
        return this.suites.find(s => s.id === id);
    },

    getAllSuites() {
        return this.suites;
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

// Backwards compatibility for late-binding modules
window.suitesData = SovereignRegistry.suites;
