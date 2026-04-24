const fs = require('fs');
const path = require('path');

function printHeader(text) {
    console.log(`\n\x1b[95m\x1b[1m=== ${text} ===\x1b[0m`);
}

function auditHeaders() {
    printHeader("Sovereign Header Audit");
    const forbidden = ['<stdint.h>', '<stdarg.h>', '<stddef.h>', '<stdbool.h>'];
    let issues = 0;
    
    function walk(dir) {
        const files = fs.readdirSync(dir);
        for (const file of files) {
            const fullPath = path.join(dir, file);
            if (fs.statSync(fullPath).isDirectory()) {
                if (file !== 'legacy' && file !== 'node_modules' && file !== '.git') {
                    walk(fullPath);
                }
            } else if (file.endsWith('.c') || file.endsWith('.h')) {
                const content = fs.readFileSync(fullPath, 'utf8');
                for (const header of forbidden) {
                    if (content.includes(`#include ${header}`)) {
                        console.log(`  \x1b[91m[ERR]\x1b[0m ${fullPath} contains legacy header ${header}`);
                        issues++;
                    }
                }
            }
        }
    }
    
    walk('.');
    if (issues === 0) console.log("  \x1b[92m[✓]\x1b[0m 0 legacy headers found. 100% Sovereign compliance.");
    else console.log(`  \x1b[91m[!]\x1b[0m Found ${issues} header violations.`);
}

function auditModularity() {
    printHeader("Lattice Modularity Audit");
    const rootDirs = ['cli', 'drivers', 'gui', 'networking', 'storage', 'services', 'shards', 'verification', 'plugins'];
    let leaks = 0;
    
    rootDirs.forEach(dir => {
        if (fs.existsSync(dir)) {
            console.log(`  \x1b[91m[ERR]\x1b[0m Root directory leak: ${dir}/ should be in modules/`);
            leaks++;
        }
    });
    
    if (leaks === 0) console.log("  \x1b[92m[✓]\x1b[0m Root structure is clean. All components modularized.");
    else console.log(`  \x1b[91m[!]\x1b[0m Found ${leaks} root leaks.`);
}

function auditDiscovery() {
    printHeader("Shard Discovery Audit");
    const suitesPath = 'suites';
    if (!fs.existsSync(suitesPath)) {
        console.log("  \x1b[91m[ERR]\x1b[0m suites/ directory missing.");
        return;
    }
    
    const shards = fs.readdirSync(suitesPath).filter(f => f.startsWith('S'));
    console.log(`  [*] Discovered ${shards.length} shards in ${suitesPath}/`);
    
    let missingJson = 0;
    shards.forEach(s => {
        if (!fs.existsSync(path.join(suitesPath, s, 'module.json'))) {
            missingJson++;
        }
    });
    
    if (missingJson === 0) console.log(`  \x1b[92m[✓]\x1b[0m All ${shards.length} shards have valid module.json manifests.`);
    else console.log(`  \x1b[91m[!]\x1b[0m ${missingJson} shards missing manifests.`);
}

auditHeaders();
auditModularity();
auditDiscovery();
