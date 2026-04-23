const fs = require('fs');
const path = require('path');

const modulesDir = 'web_ui/scripts/modules';
const files = fs.readdirSync(modulesDir).filter(f => f.endsWith('.js'));

console.log("Σ://EVOLVE> Initiating Self-Evolution Heuristic...");

let evolvedCount = 0;

// Apply evolution to a random sample of shards (or specific Phase 6 shards)
files.forEach(file => {
    const filePath = path.join(modulesDir, file);
    let content = fs.readFileSync(filePath, 'utf8');

    // Heuristic: If the shard is not already "Self-Evolving", add a mutation hook
    if (!content.includes('selfEvolve()')) {
        const evolutionHook = `
    selfEvolve() {
        const mutations = [
            "Optimizing lattice resonance...",
            "Expanding semantic context...",
            "Hardening silicon primitives...",
            "Refining cross-kernel synthesis..."
        ];
        const mutation = mutations[Math.floor(Math.random() * mutations.length)];
        console.log(\`Σ://EVOLUTION [\${this.shardId}]> \${mutation}\`);
        this.lastMutation = mutation;
    }
`;
        // Insert before the last closing brace
        const lastBraceIndex = content.lastIndexOf('}');
        if (lastBraceIndex !== -1) {
            content = content.substring(0, lastBraceIndex) + evolutionHook + content.substring(lastBraceIndex);
            
            // Also call it in init
            content = content.replace(/this\.registerCLI\(\);/g, 'this.registerCLI();\n            this.selfEvolve();');
            
            fs.writeFileSync(filePath, content);
            evolvedCount++;
        }
    }
});

console.log(`Σ://EVOLVE> Evolution cycle complete. ${evolvedCount} shards mutated.`);
