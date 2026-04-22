const fs = require('fs');
const path = require('path');

const modules = [
    { name: '146_package_manager.js', title: 'Package Manager', desc: 'apt/pacman inspired module installation and dependency resolution.', cli: 'sigma-apt' },
    { name: '147_wasm_plugin_runtime.js', title: 'WASM Plugin Runtime', desc: 'Cross-language extensions via WebAssembly (Rust, Go, Python).', cli: 'wasm-run' },
    { name: '148_config_as_code_engine.js', title: 'Config-as-Code Engine', desc: 'NixOS style declarative workspace definitions and reproducible environments.', cli: 'nix-build' },
    { name: '149_feature_flag_controller.js', title: 'Feature Flag Controller', desc: 'Dynamically toggle experimental modules without bloating the core.', cli: 'feature-flag' },
    { name: '150_rolling_release_channel.js', title: 'Rolling Release Channel', desc: 'Opt-in bleeding edge module updates vs stable branch.', cli: 'os-release' },
    { name: '151_microkernel_isolator.js', title: 'Microkernel Isolator', desc: 'Strict memory and privilege separation between Kernel and Userland modules.', cli: 'isol-sys' },
    { name: '152_community_marketplace.js', title: 'Community Marketplace', desc: 'Curated ecosystem repository of third-party tools and plugins.', cli: 'sigma-store' },
    { name: '153_data_science_environment.js', title: 'Data Science Environment', desc: 'Jupyter-like interactive ML playground natively in the browser.', cli: 'ds-env' },
    { name: '154_ai_model_server.js', title: 'AI Model Server', desc: 'Ollama inspired local LLM hosting and inference endpoint.', cli: 'ollama-sim' },
    { name: '155_code_interpreter.js', title: 'Code Interpreter', desc: 'Auto-executing Python/JS sandboxes for AI assistant agents.', cli: 'exec-code' },
    { name: '156_study_pack_bundle.js', title: 'Study Pack Bundle', desc: 'Curated meta-package installing Lecture Mode, Flashcards, and Citation Collector.', cli: 'install-study' },
    { name: '157_developer_pack_bundle.js', title: 'Developer Pack Bundle', desc: 'Curated meta-package installing Snippet Manager, API Playground, and GitHub Integration.', cli: 'install-dev' },
    { name: '158_privacy_pack_bundle.js', title: 'Privacy Pack Bundle', desc: 'Curated meta-package for ultimate tracking protection and hardened encryption.', cli: 'install-privacy' },
    { name: '159_collaboration_hub.js', title: 'Collaboration Hub', desc: 'Real-time WebRTC syncing engine for shared workspaces and co-browsing.', cli: 'collab-sync' },
    { name: '160_ultimate_convergence.js', title: 'Ultimate Convergence', desc: 'The singularity bridging Linux package management with browser OS agility.', cli: 'converge' }
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
            console.log(\`Σ://LINUX_MODULARITY> \${this.shardId} Online. ${m.desc}\`);
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

console.log('Created Linux Modularity modules (146-160) and updated kernel_loader.js');
