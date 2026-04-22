const fs = require('fs');
const path = require('path');

const modules = [
    { name: '191_deepin_ux_elegance.js', title: 'Deepin UX Elegance', desc: 'Deepin inspired highly polished, elegant desktop environment aesthetics.', cli: 'dde-sim' },
    { name: '192_elementary_pantheon_flow.js', title: 'Elementary Pantheon Flow', desc: 'elementary OS inspired Pantheon flow, minimalism, and focus.', cli: 'pantheon-ui' },
    { name: '193_zorin_os_chameleon.js', title: 'Zorin OS Chameleon', desc: 'Zorin OS inspired shape-shifting UI to mimic Windows or macOS on the fly.', cli: 'zorin-morph' },
    { name: '194_pfsense_firewall_router.js', title: 'pfSense Firewall Router', desc: 'pfSense inspired enterprise-grade firewall and web routing capabilities.', cli: 'pf-route' },
    { name: '195_truenas_zfs_storage.js', title: 'TrueNAS ZFS Storage', desc: 'TrueNAS inspired ZFS file system management for workspace data pools.', cli: 'zfs-pool' },
    { name: '196_openwrt_mesh_networking.js', title: 'OpenWrt Mesh Networking', desc: 'OpenWrt inspired lightweight mesh networking for decentralized browser communication.', cli: 'mesh-link' },
    { name: '197_coreos_immutable_containers.js', title: 'CoreOS Immutable Containers', desc: 'CoreOS inspired completely immutable states designed strictly for container orchestration.', cli: 'core-img' },
    { name: '198_blackarch_arsenal.js', title: 'BlackArch Arsenal', desc: 'BlackArch inspired massive tool repository mapping for web security and deep inspection.', cli: 'blackarch-run' },
    { name: '199_garuda_zen_gaming.js', title: 'Garuda Zen Gaming', desc: 'Garuda Linux inspired Zen kernel optimizations prioritizing UI responsiveness over throughput.', cli: 'zen-opt' },
    { name: '200_lfs_absolute_genesis.js', title: 'LFS Absolute Genesis', desc: 'Linux From Scratch inspired capability: the absolute genesis of compiling everything from bare logic, achieving the 200th Shard Singularity.', cli: 'lfs-build' }
];

const dir = 'web_ui/scripts/modules';

modules.forEach(m => {
    const className = m.title.replace(/[^a-zA-Z0-9]/g, '');
    const content = `/**
 * SigmaOS ${m.title} Shard
 * USP/Logic: ${m.desc}
 */

class ${className} {
    constructor() {
        this.shardId = "S" + "${m.name}".split('_')[0] + "_${className}";
        this.active = false;
        
        console.log(\`Σ://INIT> \${this.shardId} Initializing: ${m.title}...\`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            this.active = true;
            console.log(\`Σ://LINUX_DISTROS_FINAL> \${this.shardId} Online. ${m.desc}\`);
            this.registerCLI();
        });
    }

    registerCLI() {
        if(!window.SigmaCLI) window.SigmaCLI = {};
        window.SigmaCLI['${m.cli}'] = (args) => {
            return \`[${m.title}] Executing \${args.join(' ')}...\`;
        };
    }
}

window.Sigma${className} = new ${className}();
`;
    fs.writeFileSync(path.join(dir, m.name), content);
});

// Update kernel_loader.js
const kernelPath = 'web_ui/scripts/kernel_loader.js';
let kernelContent = fs.readFileSync(kernelPath, 'utf8');

const files = fs.readdirSync(dir).filter(f => f.endsWith('.js'));
const modulePaths = files.map(f => '    "scripts/modules/' + f + '"').join(',\\n');
const replacement = 'const SYSTEM_MODULES = [\\n' + modulePaths + ',\\n    "scripts/audit.js"\\n];';

kernelContent = kernelContent.replace(/const SYSTEM_MODULES = \[[\s\S]*?\];/, replacement.replace(/\\n/g, '\n'));
fs.writeFileSync(kernelPath, kernelContent);

console.log('Created Final Linux Distros modules (191-200) and updated kernel_loader.js');
