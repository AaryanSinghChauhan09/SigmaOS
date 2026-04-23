const fs = require('fs');
const path = require('path');

const modules = [
    { name: '625_kaos_lean_plasma.js', title: 'KaOS Lean Plasma', desc: 'KaOS inspired rolling release focusing strictly on Qt/KDE lean integration.', cli: 'kaos-qt' },
    { name: '626_mageia_urpmi_db.js', title: 'Mageia URPMI DB', desc: 'Mageia inspired URPMI package database and transactional dependency solver.', cli: 'urpmi-sim' },
    { name: '627_artix_init_agnostic.js', title: 'Artix Init Agnostic', desc: 'Artix inspired flexibility between OpenRC, Runit, and s6 init systems.', cli: 'artix-init' },
    { name: '628_pardus_pi_setup.js', title: 'Pardus PI Setup', desc: 'Pardus inspired post-installation wizard for localized system tuning.', cli: 'pardus-pi' },
    { name: '629_clearos_gateway_rules.js', title: 'ClearOS Gateway Rules', desc: 'ClearOS inspired granular network gateway and perimeter security rules.', cli: 'clear-gate' },
    { name: '630_ghostbsd_openrc_live.js', title: 'GhostBSD OpenRC Live', desc: 'GhostBSD inspired OpenRC service management for live desktop sessions.', cli: 'ghost-rc' },
    { name: '631_mx_linux_snapshot_tool.js', title: 'MX Linux Snapshot Tool', desc: 'MX Linux inspired live system snapshotting and ISO remastering suite.', cli: 'mx-snapshot' },
    { name: '632_bunsenlabs_conky_viz.js', title: 'BunsenLabs Conky Viz', desc: 'BunsenLabs inspired high-performance system telemetry visualization on desktop.', cli: 'conky-viz' },
    { name: '633_tinycore_base_ram.js', title: 'TinyCore Base RAM', desc: 'TinyCore inspired ultra-minimalist execution entirely from RAM lattices.', cli: 'tce-load' },
    { name: '634_bodhi_moksha_flow.js', title: 'Bodhi Moksha Flow', desc: 'Bodhi Linux inspired Enlightenment-based Moksha UI flow and aesthetics.', cli: 'moksha-ui' }
];

const dir = 'web_ui/scripts/modules';
const shardsDir = 'shards';

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
            console.log(\`Σ://LINUX_PARITY> \${this.shardId} Online. ${m.desc}\`);
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

console.log('Created additional Linux Parity modules (625-634) and updated kernel_loader.js');
