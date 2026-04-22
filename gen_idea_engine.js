const fs = require('fs');
const path = require('path');

const modules = [
    { name: '73_lecture_mode.js', title: 'Lecture Mode', desc: 'Auto-summarize YouTube lectures into notes and flashcards.' },
    { name: '74_quiz_generator.js', title: 'Quiz Generator', desc: 'Turn study material into interactive practice questions.' },
    { name: '75_study_group_mode.js', title: 'Study Group Mode', desc: 'Shared workspaces tailored for collaborative learning.' },
    { name: '76_debug_mode.js', title: 'Workflow Debug Mode', desc: 'Inspect and log automated workflows across tasks.' },
    { name: '77_comment_layer.js', title: 'Web Comment Layer', desc: 'Add persistent comments directly on web pages.' },
    { name: '78_versioned_workspaces.js', title: 'Versioned Workspaces', desc: 'Roll back to previous tab and task states.' },
    { name: '79_workspace_chat.js', title: 'Workspace Chat', desc: 'Built-in messaging tied contextually to tasks.' },
    { name: '80_task_suggestions.js', title: 'Task Suggestions', desc: 'Recommend next steps based on browsing history.' },
    { name: '81_offline_study_mode.js', title: 'Offline Study Mode', desc: 'Download lectures and generate notes entirely offline.' },
    { name: '82_gamification_engine.js', title: 'Gamification Engine', desc: 'XP system for tasks completed, lectures watched, and code written.' }
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
            console.log(\`Σ://ENGINE> \${this.shardId} Online. ${m.desc}\`);
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

console.log('Created idea engine modules and updated kernel_loader.js');
