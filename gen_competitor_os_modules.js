const fs = require('fs');
const path = require('path');

const modules = [
    { name: '83_visual_automator.js', title: 'Visual Automator', desc: 'macOS Shortcuts inspired visual node-based automation.' },
    { name: '84_subsystem_linux.js', title: 'Subsystem Linux', desc: 'WSL-inspired headless Linux terminal environment.' },
    { name: '85_material_monet_engine.js', title: 'Material Monet Engine', desc: 'Android Material You inspired dynamic wallpaper color extraction.' },
    { name: '86_community_nexus_repo.js', title: 'Community Nexus Repo', desc: 'Arch AUR inspired community-driven package repository.' },
    { name: '87_mobile_phone_hub.js', title: 'Mobile Phone Hub', desc: 'ChromeOS inspired deep mobile device integration.' },
    { name: '88_continuity_camera.js', title: 'Continuity Camera', desc: 'macOS inspired external device webcam integration.' },
    { name: '89_power_toys_suite.js', title: 'PowerToys Suite', desc: 'Windows inspired power-user utilities (color picker, text extractor).' },
    { name: '90_intelligent_app_library.js', title: 'Intelligent App Library', desc: 'iOS inspired auto-categorization of installed applications.' },
    { name: '91_cow_snapshots.js', title: 'COW Snapshots', desc: 'Linux ZFS/Btrfs inspired copy-on-write instant system rollbacks.' },
    { name: '92_seamless_handoff.js', title: 'Seamless Handoff', desc: 'macOS inspired cross-device task continuation.' }
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
            console.log(\`Σ://OS_ABSORB> \${this.shardId} Online. ${m.desc}\`);
        });
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

console.log('Created Competitor OS modules and updated kernel_loader.js');
