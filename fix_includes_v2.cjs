/**
 * SigmaOS Include Path Industrial Fixer v2.0
 * 
 * Root cause: Files inside include/ reference "../include/..." which breaks.
 * Files inside kernel/core/system/ reference "../../include/..." (too shallow).
 * 
 * Strategy: For every .c/.cpp/.h/.hpp file, calculate its depth from ROOT,
 * then rewrite all #include paths to be correct relative paths.
 */
const fs = require('fs');
const path = require('path');

const ROOT = process.cwd();
const INCLUDE_DIR = path.join(ROOT, 'include');

function walkDir(dir, callback) {
    fs.readdirSync(dir).forEach(f => {
        const full = path.join(dir, f);
        if (f === '.git' || f === 'node_modules' || f === 'build') return;
        try {
            const stat = fs.statSync(full);
            if (stat.isDirectory()) walkDir(full, callback);
            else callback(full);
        } catch(e) {}
    });
}

function fixFile(filepath) {
    if (!/\.(c|cpp|h|hpp)$/.test(filepath)) return;
    
    const fileDir = path.dirname(filepath);
    let content = fs.readFileSync(filepath, 'utf8');
    let changed = false;
    
    const lines = content.split('\n');
    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        const match = line.match(/^#include\s*(["<])([^">]+)([">\s])/);
        if (!match) continue;
        
        const openQ = match[1];
        const incPath = match[2];
        const closeQ = match[3].trim() || (openQ === '"' ? '"' : '>');
        
        // Skip system headers
        if (openQ === '<') continue;
        
        // Try to resolve the current include path
        let resolvedTarget = null;
        
        // Strip the current path to get the base filename/subpath
        // Remove all leading "../" and "./"
        let cleanPath = incPath.replace(/^(\.\.\/)+/g, '').replace(/^\.\//g, '');
        
        // If it starts with "include/", strip it
        if (cleanPath.startsWith('include/')) {
            cleanPath = cleanPath.substring('include/'.length);
        }
        
        // Now try to find this file in the include directory
        const candidates = [
            path.join(INCLUDE_DIR, cleanPath),
            // Handle "core/sigma_types.h" -> "include/core/sigma_types.h"
            path.join(INCLUDE_DIR, cleanPath),
            // Handle "sigma_types.h" -> "include/sigma_types.h" 
            // Handle "sigma_types.h" -> "include/core/sigma_types.h"
        ];
        
        // Also try common subdirs
        const subdirs = ['', 'core/', 'fs/', 'libc/', 'system/', 'hal/', 'net/', 'security/', 'ai/', 'boot/', 'drivers/', 'orchestration/', 'ui/', 'sched/', 'virt/'];
        const baseName = path.basename(cleanPath);
        for (const sub of subdirs) {
            candidates.push(path.join(INCLUDE_DIR, sub + baseName));
        }
        
        for (const cand of candidates) {
            if (fs.existsSync(cand)) {
                resolvedTarget = cand;
                break;
            }
        }
        
        if (!resolvedTarget) continue;
        
        // Calculate correct relative path from fileDir to resolvedTarget
        let correctRel = path.relative(fileDir, resolvedTarget).replace(/\\/g, '/');
        
        // Ensure it starts with "./" or "../"
        if (!correctRel.startsWith('.')) {
            correctRel = './' + correctRel;
        }
        
        const newLine = `#include "${correctRel}"`;
        if (newLine !== line.trimEnd()) {
            lines[i] = newLine;
            changed = true;
        }
    }
    
    if (changed) {
        fs.writeFileSync(filepath, lines.join('\n'));
        const rel = path.relative(ROOT, filepath);
        console.log(`Fixed: ${rel}`);
    }
}

console.log('=== SigmaOS Include Path Industrial Fixer v2.0 ===');
console.log(`ROOT: ${ROOT}`);
console.log('');

walkDir(ROOT, fixFile);

console.log('');
console.log('=== Fix complete ===');
