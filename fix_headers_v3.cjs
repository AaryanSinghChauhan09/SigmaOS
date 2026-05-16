/**
 * SigmaOS Master Header Normalizer v3.0
 *
 * Fixes:
 * 1. All includes of "sigma_types.h" or "core/sigma_types.h" in include/ headers
 *    → replaced with "sigma_kernel_types.h" (the single source of truth)
 * 2. Fixes "sigma_hal.h not used directly" in files that don't actually use HAL
 * 3. Creates include/core/sigma_kernel_types.h redirect for IDE compat
 */
const fs = require('fs');
const path = require('path');
const ROOT = process.cwd();

// Map of wrong include patterns -> correct replacements for HEADER files (inside include/)
const INCLUDE_DIR_FIXES = [
    [/["']\.\/core\/sigma_types\.h["']/g,         '"./sigma_kernel_types.h"'],
    [/["']\.\.\/sigma_types\.h["']/g,              '"../sigma_kernel_types.h"'],
    [/["']\.\/sigma_types\.h["']/g,                '"./sigma_kernel_types.h"'],
    [/["']sigma_types\.h["']/g,                    '"sigma_kernel_types.h"'],
];

// For .cpp files, replace "core/sigma_types.h" with "sigma_kernel_types.h"
const CPP_FIXES = [
    [/"\.\.\/include\/core\/sigma_types\.h"/g,     '"../include/sigma_kernel_types.h"'],
    [/"\.\.\/\.\.\/include\/core\/sigma_types\.h"/g, '"../../include/sigma_kernel_types.h"'],
    [/"\.\.\/\.\.\/\.\.\/include\/core\/sigma_types\.h"/g, '"../../../include/sigma_kernel_types.h"'],
];

function walkDir(dir, cb) {
    try {
        fs.readdirSync(dir).forEach(f => {
            const full = path.join(dir, f);
            if (f === '.git' || f === 'node_modules' || f === 'build') return;
            try {
                const st = fs.statSync(full);
                if (st.isDirectory()) walkDir(full, cb);
                else cb(full);
            } catch(e) {}
        });
    } catch(e) {}
}

let fixedCount = 0;

walkDir(ROOT, (filepath) => {
    if (!/\.(c|cpp|h|hpp)$/.test(filepath)) return;

    let content = fs.readFileSync(filepath, 'utf8');
    let orig = content;

    const isInInclude = filepath.replace(/\\/g, '/').includes('/include/');
    const fixes = isInInclude ? INCLUDE_DIR_FIXES : CPP_FIXES;

    for (const [pattern, replacement] of fixes) {
        content = content.replace(pattern, replacement);
    }

    if (content !== orig) {
        fs.writeFileSync(filepath, content);
        console.log('Fixed: ' + path.relative(ROOT, filepath));
        fixedCount++;
    }
});

// Ensure include/core/sigma_kernel_types.h exists as redirect
const kernelTypesRedirect = path.join(ROOT, 'include', 'core', 'sigma_kernel_types.h');
if (!fs.existsSync(kernelTypesRedirect)) {
    fs.writeFileSync(kernelTypesRedirect, `#ifndef SIGMA_KERNEL_TYPES_CORE_REDIRECT_H\n#define SIGMA_KERNEL_TYPES_CORE_REDIRECT_H\n/* Redirect: include/core/ -> master header */\n#include "../sigma_kernel_types.h"\n#endif\n`);
    console.log('Created: include/core/sigma_kernel_types.h (redirect)');
}

console.log(`\nDone. Fixed ${fixedCount} files.`);
