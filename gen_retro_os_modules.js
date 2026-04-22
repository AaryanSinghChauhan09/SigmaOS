const fs = require('fs');
const path = require('path');

const modules = [
    { name: '121_plan9_protocol.js', title: 'Plan 9 Protocol', desc: 'Plan 9 inspired 9P protocol and Everything-is-a-File abstraction.', cli: '9p-mount' },
    { name: '122_haiku_bfs_metadata.js', title: 'Haiku BFS Metadata', desc: 'BeOS/Haiku inspired database-like filesystem queries and rich metadata.', cli: 'bfs-query' },
    { name: '123_qnx_hard_realtime.js', title: 'QNX Hard Realtime', desc: 'QNX inspired hard real-time microkernel thread scheduling.', cli: 'qnx-rt' },
    { name: '124_openbsd_pledge.js', title: 'OpenBSD Pledge', desc: 'OpenBSD inspired strict security sandboxing via pledge/unveil.', cli: 'pledge-sys' },
    { name: '125_amiga_arexx.js', title: 'Amiga ARexx', desc: 'AmigaOS inspired ARexx robust inter-process communication bus.', cli: 'arexx-msg' },
    { name: '126_templeos_holyc.js', title: 'TempleOS HolyC', desc: 'TempleOS inspired HolyC JIT compilation and hardware-based PRNG.', cli: 'holyc-jit' },
    { name: '127_webos_card_ui.js', title: 'WebOS Card UI', desc: 'Palm WebOS inspired card-based multitasking and Synergy cloud sync.', cli: 'webos-cards' },
    { name: '128_symbian_power_mgmt.js', title: 'Symbian Power Management', desc: 'Symbian inspired extreme power state optimization and hibernation.', cli: 'symbian-pwr' },
    { name: '129_freebsd_jails.js', title: 'FreeBSD Jails', desc: 'FreeBSD inspired lightweight containerized system environments.', cli: 'jail-mgr' },
    { name: '130_serenity_visual_engine.js', title: 'Serenity Visual Engine', desc: 'SerenityOS inspired 90s aesthetic compositing via modern WebGL.', cli: 'serenity-ui' }
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
            console.log(\`Σ://RETRO_OS> \${this.shardId} Online. ${m.desc}\`);
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

console.log('Created Retro/Niche OS modules (121-130) and updated kernel_loader.js');
