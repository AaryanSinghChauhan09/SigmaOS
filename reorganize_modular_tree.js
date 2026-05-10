const fs = require('fs');
const path = require('path');

const modulesDir = 'web_ui/scripts/modules';
const shardsDir = 'shards';

const getCategory = (num) => {
    if (num <= 50) return 'core';
    if (num <= 150) return 'essential';
    if (num <= 300) return 'optional';
    if (num <= 450) return 'third_party';
    return 'infinite';
};

const files = fs.readdirSync(modulesDir).filter(f => f.endsWith('.js') && !['audit.js', 'kernel_loader.js'].includes(f));

files.forEach(f => {
    const num = parseInt(f.split('_')[0]);
    if (isNaN(num)) return;

    const category = getCategory(num);
    
    // Move in web_ui
    const oldPath = path.join(modulesDir, f);
    const newPath = path.join(modulesDir, category, f);
    fs.renameSync(oldPath, newPath);

    // Move in root shards/
    const oldShardPath = path.join(shardsDir, f);
    const newShardPath = path.join(shardsDir, category, f);
    if (fs.existsSync(oldShardPath)) {
        fs.renameSync(oldShardPath, newShardPath);
    }
});

// Update kernel_loader.js with categories
const allFiles = [];
['core', 'essential', 'optional', 'third_party', 'infinite'].forEach(cat => {
    const catDir = path.join(modulesDir, cat);
    if (fs.existsSync(catDir)) {
        const catFiles = fs.readdirSync(catDir).filter(f => f.endsWith('.js'));
        catFiles.forEach(f => {
            allFiles.push(`    "scripts/modules/${cat}/${f}"`);
        });
    }
});

const replacement = 'const SYSTEM_MODULES = [\n' + allFiles.join(',\n') + ',\n    "scripts/audit.js"\n];';
const loaderPath = 'web_ui/scripts/kernel_loader.js';
let loaderContent = fs.readFileSync(loaderPath, 'utf8');
loaderContent = loaderContent.replace(/const SYSTEM_MODULES = \[[\s\S]*?\];/, replacement);
fs.writeFileSync(loaderPath, loaderContent);

console.log("Modular Reorganization Complete: 600 Shards categorized into Core, Essential, Optional, Third-Party, and Infinite folders.");
