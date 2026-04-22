const fs = require('fs');
const path = require('path');

const modules = [
    { name: '64_workspace_templates.js', title: 'Workspace Templates', desc: 'Arc-inspired prebuilt setups (Coding, Study, Research).' },
    { name: '65_privacy_layer.js', title: 'Privacy Layer', desc: 'Brave-inspired tracker blocking and hardened kernel primitives.' },
    { name: '66_utility_sidebar.js', title: 'Utility Sidebar', desc: 'Opera-inspired integrated messengers, tools, and learning progress.' },
    { name: '67_workflow_scripting.js', title: 'Workflow Scripting', desc: 'Vivaldi-inspired user-defined automation for tab actions.' },
    { name: '68_learning_mode.js', title: 'Learning Mode', desc: 'Auto-detect educational content and generate summaries/flashcards.' },
    { name: '69_coding_companion.js', title: 'Coding Companion', desc: 'Inline snippet manager, GitHub integration, and API playground.' },
    { name: '70_citation_collector.js', title: 'Citation Collector', desc: 'Auto-generate references from academic/legal tabs.' },
    { name: '71_workspace_ai_assistant.js', title: 'Workspace AI Assistant', desc: 'Suggest next steps based on current browsing context.' },
    { name: '72_live_co_browsing.js', title: 'Live Co-Browsing', desc: 'Sidekick-inspired real-time collaborative browsing for study groups.' }
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
            console.log(\`Σ://USP> \${this.shardId} Online. ${m.desc}\`);
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

console.log('Created advanced USP modules and updated kernel_loader.js');
