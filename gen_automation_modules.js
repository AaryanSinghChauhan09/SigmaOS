const fs = require('fs');
const path = require('path');

const modules = [
    { name: '161_dual_boot_manager.js', title: 'Dual Boot Manager', desc: 'GRUB inspired bootloader switching between OS states.', cli: 'grub-sim' },
    { name: '162_hardware_abstraction_layer.js', title: 'Hardware Abstraction Layer', desc: 'Deeply integrating WebUSB, WebBluetooth, WebGPU.', cli: 'hal-ctrl' },
    { name: '163_container_sandbox.js', title: 'Container Sandbox', desc: 'Docker inspired sandboxed workspace containers.', cli: 'sandbox-run' },
    { name: '164_vm_orchestrator.js', title: 'VM Orchestrator', desc: 'KVM inspired lightweight VM support for Study/Coding VMs.', cli: 'vm-launch' },
    { name: '165_workspace_automation_engine.js', title: 'Workspace Automation Engine', desc: 'Auto-grouping, auto-resuming, and auto-archiving domains.', cli: 'auto-work' },
    { name: '166_learning_automation_engine.js', title: 'Learning Automation Engine', desc: 'Auto-summarize lectures, auto-generate flashcards and quizzes.', cli: 'auto-learn' },
    { name: '167_developer_automation_engine.js', title: 'Developer Automation Engine', desc: 'Auto-save snippets, auto-link GitHub, auto-test APIs.', cli: 'auto-dev' },
    { name: '168_collab_automation_engine.js', title: 'Collab Automation Engine', desc: 'Auto-share, auto-notify, and auto-version team workspaces.', cli: 'auto-collab' },
    { name: '169_privacy_automation_engine.js', title: 'Privacy Automation Engine', desc: 'Auto-block trackers, auto-encrypt, auto-switch to VPN.', cli: 'auto-priv' },
    { name: '170_system_automation_engine.js', title: 'System Automation Engine', desc: 'Auto-update modules, auto-rollback NixOS style configs.', cli: 'auto-sys' },
    { name: '171_combinatorial_trigger_bus.js', title: 'Combinatorial Trigger Bus', desc: 'Rule engine crossing contexts and triggers for 10,000+ automations.', cli: 'trigger-bus' },
    { name: '172_event_sourcing_journal.js', title: 'Event Sourcing Journal', desc: 'Immutable log of all automations to allow perfect state replay.', cli: 'es-journal' },
    { name: '173_cross_context_sync.js', title: 'Cross Context Sync', desc: 'Syncing project contexts and learning progress globally.', cli: 'ctx-sync' },
    { name: '174_gamification_xp_ledger.js', title: 'Gamification XP Ledger', desc: 'Securing XP and achievements for tasks completed and code written.', cli: 'xp-ledger' },
    { name: '175_sigma_omnipresence.js', title: 'Sigma Omnipresence', desc: 'The apex state unifying the 6 automation engines into one workflow.', cli: 'omnipresence' }
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
            console.log(\`Σ://AUTOMATION_MATRIX> \${this.shardId} Online. ${m.desc}\`);
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

console.log('Created Automation Engine modules (161-175) and updated kernel_loader.js');
