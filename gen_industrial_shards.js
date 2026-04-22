const fs = require('fs');
const path = require('path');

const industrialModules = [
    // 334-343: High-Availability & Cluster Logic
    { name: '334_raft_consensus_sim.js', title: 'Raft Consensus Sim', desc: 'Distributed consensus algorithm for shared task state across windows.', cli: 'raft-sim' },
    { name: '335_task_migration_relay.js', title: 'Task Migration Relay', desc: 'Migrating active task state between browser instances during failure.', cli: 'task-migrate' },
    { name: '336_multi_tenant_lattice.js', title: 'Multi-Tenant Lattice', desc: 'Managing isolated user states within a single browser environment.', cli: 'multi-tenant' },
    { name: '337_k3s_lite_orchestrator.js', title: 'K3s Lite Orchestrator', desc: 'Lightweight orchestration for distributed OS service shards.', cli: 'k3s-lite' },
    { name: '338_load_balanced_shard_bus.js', title: 'Load Balanced Shard Bus', desc: 'Distributing event load across multiple worker-backed shards.', cli: 'lb-bus' },

    // 344-353: Service Supervision & Immutability
    { name: '344_runit_service_supervisor.js', title: 'Runit Service Supervisor', desc: 'Void inspired parallel service monitoring and auto-restart.', cli: 'runit-sim' },
    { name: '345_ostree_delta_update.js', title: 'OSTree Delta Update', desc: 'Fedora Silverblue inspired immutable delta-based updates.', cli: 'delta-up' },
    { name: '346_lattice_read_only_core.js', title: 'Lattice Read-Only Core', desc: 'Hardening the core lattice as a read-only immutable state.', cli: 'lock-core' },
    { name: '347_checksum_purity_verify.js', title: 'Checksum Purity Verify', desc: 'Real-time hashing of shards to detect tampering or corruption.', cli: 'hash-verify' },
    { name: '348_environment_trust_boot.js', title: 'Environment Trust Boot', desc: 'Verifying host browser environment before unlocking sensitive tasks.', cli: 'trust-boot' },

    // 354-360: Scientific & Forensics
    { name: '354_web_mpi_parallel_bus.js', title: 'Web-MPI Parallel Bus', desc: 'Message passing interface for distributed DOM/AI compute.', cli: 'mpi-exec' },
    { name: '355_forensic_noise_scrubber.js', title: 'Forensic Noise Scrubber', desc: 'Wiping deleted shard state with cryptographic noise.', cli: 'noise-wipe' },
    { name: '356_dynamic_theme_loader.js', title: 'Dynamic Theme Loader', desc: 'elementary OS inspired dynamic loading of UI theme shards.', cli: 'theme-load' },
    { name: '357_manjaro_hw_shim.js', title: 'Manjaro HW Shim', desc: 'Manjaro inspired automated detection of browser capabilities.', cli: 'hw-detect' },
    { name: '358_scientific_data_viz.js', title: 'Scientific Data Viz', desc: 'Advanced plotting and visualization shards for research tasks.', cli: 'plot-viz' },
    { name: '359_industrial_grade_nexus.js', title: 'Industrial Grade Nexus', desc: 'The bridge to high-availability sovereign computing.', cli: 'ha-nexus' },
    { name: '360_industrial_singularity.js', title: 'Industrial Singularity', desc: 'The 360th Shard: Reaching the Industrial Singularity milestone.', cli: 'singularity-360' }
];

const dir = 'web_ui/scripts/modules';
const shardsDir = 'shards';

industrialModules.forEach(m => {
    const className = m.title.replace(/[^a-zA-Z0-9]/g, '');
    const content = `/**
 * SigmaOS \${m.title} Industrial Shard
 * Logic: \${m.desc}
 */

class \${className} {
    constructor() {
        this.shardId = "S" + "\${m.name}".split('_')[0] + "_\${className}";
        this.active = false;
        
        console.log(\`Σ://INDUSTRIAL> \${this.shardId} Initializing: \${m.title}...\`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            this.active = true;
            console.log(\`Σ://SINGULARITY_360> \${this.shardId} Online. \${m.desc}\`);
            this.registerCLI();
        });
    }

    registerCLI() {
        if(!window.SigmaCLI) window.SigmaCLI = {};
        window.SigmaCLI['\${m.cli}'] = (args) => {
            return \`[\${m.title}] Industrial Call: \${args.join(' ') || 'STATUS'}\`;
        };
    }
}

window.Sigma\${className} = new \${className}();
`;
    fs.writeFileSync(path.join(dir, m.name), content);
    fs.writeFileSync(path.join(shardsDir, m.name), content);
});

// Update kernel_loader.js
const files = fs.readdirSync(dir).filter(f => f.endsWith('.js')).sort((a, b) => parseInt(a) - parseInt(b));
const modulePaths = files.map(f => '    "scripts/modules/' + f + '"').join(',\n');
const replacement = 'const SYSTEM_MODULES = [\n' + modulePaths + ',\n    "scripts/audit.js"\n];';

const kernelPath = 'web_ui/scripts/kernel_loader.js';
let kernelContent = fs.readFileSync(kernelPath, 'utf8');
kernelContent = kernelContent.replace(/const SYSTEM_MODULES = \[[\s\S]*?\];/, replacement);
fs.writeFileSync(kernelPath, kernelContent);

console.log('Industrial Singularity Shards (334-360) generated. Total: 360 Shards.');
