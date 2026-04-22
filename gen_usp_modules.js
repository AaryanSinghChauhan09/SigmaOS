const fs = require('fs');
const path = require('path');

const modules = [
    { name: '54_smart_tab_workflows.js', title: 'Smart Tab Workflows', desc: 'Auto-group tabs by project, context, or domain.' },
    { name: '55_task_linked_tabs.js', title: 'Task-Linked Tabs', desc: 'Attach tabs directly to to-do items or project tasks.' },
    { name: '56_adaptive_tab_memory.js', title: 'Adaptive Tab Memory', desc: 'Tabs reopen with scroll position, highlights, and notes preserved.' },
    { name: '57_learning_integration.js', title: 'Learning Dashboard Integration', desc: 'Detect learning platforms like Sololearn and log progress.' },
    { name: '58_workspace_collaboration.js', title: 'Workspace Collaboration', desc: 'Share tab groups or workspaces with teammates with annotations.' },
    { name: '59_github_integration.js', title: 'GitHub Integration', desc: 'Inline repo previews, issue tracking, and PR commenting.' },
    { name: '60_code_snippet_manager.js', title: 'Code Snippet Manager', desc: 'Save snippets directly from tutorials into a searchable library.' },
    { name: '61_api_playground.js', title: 'API Playground', desc: 'Built-in lightweight REST client for quick testing.' },
    { name: '62_academic_mode.js', title: 'Legal & Academic Mode', desc: 'Auto-organize references, citations, and notes for study tabs.' },
    { name: '63_offline_continuity.js', title: 'Offline Continuity', desc: 'Allow offline tab editing/annotation that syncs back online.' }
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

console.log('Created modules and updated kernel_loader.js');
