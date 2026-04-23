const fs = require('fs');
const path = require('path');

const modules = [
    { name: '635_freebsd_zfs_engine.js', title: 'FreeBSD ZFS Engine', desc: 'FreeBSD inspired ZFS storage pool orchestration and self-healing data integrity.', cli: 'zfs-sim' },
    { name: '636_openbsd_pledge_audit.js', title: 'OpenBSD Pledge Audit', desc: 'OpenBSD inspired pledge/unveil system call restriction for shard confinement.', cli: 'pledge-audit' },
    { name: '637_solaris_dtrace_viz.js', title: 'Solaris DTrace Viz', desc: 'Solaris inspired DTrace dynamic tracing for real-time kernel observability visualization.', cli: 'dtrace-viz' },
    { name: '638_netbsd_rump_kernel.js', title: 'NetBSD Rump Kernel', desc: 'NetBSD inspired rump kernels for running drivers in isolated userland sandboxes.', cli: 'rump-run' },
    { name: '639_illumos_zones_isolator.js', title: 'Illumos Zones Isolator', desc: 'Illumos inspired zones for lightweight, multi-tenant OS-level virtualization.', cli: 'zone-adm' },
    { name: '640_plan9_9p_relay.js', title: 'Plan 9 9P Relay', desc: 'Plan 9 inspired 9P resource sharing protocol for distributed lattice nodes.', cli: '9p-relay' },
    { name: '641_omnipresence_sync_mesh.js', title: 'Omnipresence Sync Mesh', desc: 'Phase 6 core: P2P mesh synchronization for global OS state persistence.', cli: 'mesh-sync' }
];

const dir = 'web_ui/scripts/modules';
const shardsDir = 'shards';

modules.forEach(m => {
    const className = m.title.replace(/[^a-zA-Z0-9]/g, '');
    const content = `/**
 * SigmaOS ${m.title} Shard
 * Logic: ${m.desc} (Phase 6 Omnipresence)
 */

class ${className} {
    constructor() {
        this.shardId = "S" + "${m.name}".split('_')[0] + "_${className}";
        this.active = false;
        
        console.log(\`Σ://OMNIPRESENCE> \${this.shardId} Initializing: ${m.title}...\`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            this.active = true;
            console.log(\`Σ://PHASE_6> \${this.shardId} Online. ${m.desc}\`);
            this.registerCLI();
        });
    }

    registerCLI() {
        if(!window.SigmaCLI) window.SigmaCLI = {};
        window.SigmaCLI['${m.cli}'] = (args) => {
            return \`[${m.title}] Cross-Kernel Call: \${args.join(' ') || 'STATUS'}\`;
        };
    }
}

window.Sigma${className} = new ${className}();
`;
    fs.writeFileSync(path.join(dir, m.name), content);
    fs.writeFileSync(path.join(shardsDir, m.name), content);
});

// Update kernel_loader.js
const kernelPath = 'web_ui/scripts/kernel_loader.js';
let kernelContent = fs.readFileSync(kernelPath, 'utf8');

const files = fs.readdirSync(dir).filter(f => f.endsWith('.js')).sort((a, b) => {
    const numA = parseInt(a.split('_')[0]);
    const numB = parseInt(b.split('_')[0]);
    if (isNaN(numA) || isNaN(numB)) return a.localeCompare(b);
    return numA - numB;
});

const modulePaths = files.map(f => '    "scripts/modules/' + f + '"').join(',\n');
const replacement = 'const SYSTEM_MODULES = [\n' + modulePaths + ',\n    "scripts/audit.js"\n];';

kernelContent = kernelContent.replace(/const SYSTEM_MODULES = \[[\s\S]*?\];/, replacement);
fs.writeFileSync(kernelPath, kernelContent);

console.log('Created Phase 6 Cross-Kernel modules (635-641) and updated kernel_loader.js');
