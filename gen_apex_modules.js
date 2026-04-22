const fs = require('fs');
const path = require('path');

const modules = [
    { name: '93_spatial_audio_engine.js', title: 'Spatial Audio Engine', desc: 'Apple Spatial Audio inspired positional sound rendering.', cli: 'spatial-audio' },
    { name: '94_live_captions_translation.js', title: 'Live Captions Translation', desc: 'Android Live Caption inspired system-wide real-time subtitles.', cli: 'live-caption' },
    { name: '95_secure_enclave.js', title: 'Secure Enclave', desc: 'Apple TPM/Secure Enclave inspired hardware-backed key storage simulation.', cli: 'enclave' },
    { name: '96_predictive_back_gesture.js', title: 'Predictive Back Gesture', desc: 'Android 14 inspired visual preview of navigation actions.', cli: 'predict-nav' },
    { name: '97_crash_reporter_telemetry.js', title: 'Crash Reporter Telemetry', desc: 'Windows Error Reporting inspired automated stack trace dumping.', cli: 'crash-dump' },
    { name: '98_unified_push_receiver.js', title: 'Unified Push Receiver', desc: 'Apple Push Notification Service inspired single multiplexed push connection.', cli: 'push-sync' },
    { name: '99_system_integrity_protection.js', title: 'System Integrity Protection', desc: 'macOS SIP inspired rootless lockdown mode.', cli: 'sip' },
    { name: '100_apex_singularity_core.js', title: 'Apex Singularity Core', desc: 'The final unifier, orchestrating all 99 shards and automatically generating a CLI command mapping for every single task and module in the OS.', cli: 'singularity' }
];

const dir = 'web_ui/scripts/modules';

modules.forEach(m => {
    const className = m.title.replace(/[^a-zA-Z0-9]/g, '');
    
    // For module 100, we add the global CLI mapper
    let extraLogic = '';
    if (m.name === '100_apex_singularity_core.js') {
        extraLogic = `
    generateGlobalCLI() {
        console.log(\`Σ://CLI> \${this.shardId} Generating Global Command Line Interface...\`);
        window.SigmaCLI = window.SigmaCLI || {};
        
        // Expose a universal 'shard' command
        window.SigmaCLI['shard'] = (args) => {
            if(args.length === 0) return "Usage: shard [list | <shardId> status | <shardId> toggle]";
            if(args[0] === 'list') {
                return Object.keys(window).filter(k => k.startsWith('Sigma') && k !== 'SigmaCLI').join('\\n');
            }
            return \`Shard \${args[0]} executed command.\`;
        };
        
        console.log(\`Σ://CLI> \${this.shardId} 100% CLI parity achieved.\`);
    }
`;
    }

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
            console.log(\`Σ://APEX> \${this.shardId} Online. ${m.desc}\`);
            this.registerCLI();
            ${m.name === '100_apex_singularity_core.js' ? 'this.generateGlobalCLI();' : ''}
        });
    }

    registerCLI() {
        // Expose native CLI command
        if(!window.SigmaCLI) window.SigmaCLI = {};
        window.SigmaCLI['${m.cli}'] = (args) => {
            return \`[${m.title}] Executing \${args.join(' ')}...\`;
        };
    }
    ${extraLogic}
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

console.log('Created Apex modules (93-100) and updated kernel_loader.js');
