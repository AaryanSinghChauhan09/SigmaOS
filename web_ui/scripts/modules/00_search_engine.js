/**
 * SigmaOS Sovereign Search Engine
 * Module 00: Unified query processing and directive mapping.
 */

const SovereignSearch = {
    directives: [
        { phrase: 'install', cmd: 'sigpkg install', desc: 'Sovereign Package Installation' },
        { phrase: 'update',  cmd: 'sigupdate',      desc: 'System Universal Sync' },
        { phrase: 'firewall',cmd: 'sigwall --gui',  desc: 'Open Firewall Orchestrator' },
        { phrase: 'audit',   cmd: 'sig-audit',      desc: 'Security Purity Sweep' }
    ],

    query(text) {
        const results = [];
        const q = text.toLowerCase().trim();
        if (!q) return results;

        // 1. Suites
        SovereignRegistry.getAllSuites().forEach(s => {
            if (s.name.toLowerCase().includes(q) || s.id.toLowerCase().includes(q)) {
                results.push({ type: 'SUITE', icon: s.icon, item: s });
            }
        });

        // 2. Directives
        this.directives.forEach(d => {
            if (q.includes(d.phrase)) {
                results.push({ 
                    type: 'DIRECTIVE', 
                    icon: '⚡', 
                    item: { name: `${d.cmd}...`, desc: d.desc },
                    action: () => alert(`Sovereign NLP Executing: ${d.cmd}`)
                });
            }
        });

        return results;
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

window.SovereignSearch = SovereignSearch;
