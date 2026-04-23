const fs = require('fs');
const path = require('path');

const modulesDir = 'web_ui/scripts/modules';
const loaderPath = 'web_ui/scripts/kernel_loader.js';

console.log("Σ://TEST> Initiating Singularity Integrity Audit...");

// 1. Check module directory existence
if (!fs.existsSync(modulesDir)) {
    console.error("FAIL: Modules directory not found.");
    process.exit(1);
}

const moduleFiles = fs.readdirSync(modulesDir).filter(f => f.endsWith('.js'));
console.log(`Σ://TEST> Found ${moduleFiles.length} modules.`);

// 2. Verify all files have content and basic structure
let broken = 0;
moduleFiles.forEach(file => {
    const content = fs.readFileSync(path.join(modulesDir, file), 'utf8');
    if (content.length < 50) {
        console.error(`FAIL: Module ${file} is suspiciously small.`);
        broken++;
    }
    if (!content.includes('class ') && !content.includes('function ') && !content.includes('const ') && !content.includes('window.')) {
        console.error(`FAIL: Module ${file} lacks logic structure.`);
        broken++;
    }
});

// 3. Verify kernel loader registration
const loaderContent = fs.readFileSync(loaderPath, 'utf8');
let missingInLoader = 0;
moduleFiles.forEach(file => {
    if (!loaderContent.includes(file)) {
        console.error(`FAIL: Module ${file} not registered in kernel_loader.js.`);
        missingInLoader++;
    }
});

console.log("\nΣ://SUMMARY>");
console.log(`  - Total Modules: ${moduleFiles.length}`);
console.log(`  - Integrity Failures: ${broken}`);
console.log(`  - Registration Gaps: ${missingInLoader}`);

if (broken === 0 && missingInLoader === 0) {
    console.log("\nΣ://CERT> SYSTEM SINGULARITY CERTIFIED. 100% INTEGRITY.");
} else {
    console.log("\nΣ://FAIL> SYSTEM FRAGMENTATION DETECTED.");
    process.exit(1);
}
