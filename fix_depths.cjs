const fs = require('fs');
const path = require('path');

const ROOT = process.cwd();

const INCLUDE_ROOT_FILES = [
    "sigma_kernel_types.h", "sigma_log.h", "sigma_hal.h", "SigmaOOP.hpp", 
    "SovereignLibC.h", "sigma_boot.h", "sigma_types.h", "sigma_syscall.h",
    "sigma_net.h", "sigma_fs.h", "sigma_vfs.h", "sigma_sched.h", "sigma_pqc.h",
    "sigma_armor.h", "sigma_audit.h", "sigma_iot.h", "sigma_gaming.h",
    "sigma_optimizer.h", "sigma_compliance.h", "sigma_kube.h", "sigma_regression.h"
];

const INCLUDE_DIRS = ["core", "hal", "libc", "system", "fs", "net", "security", "ai", "boot", "drivers", "orchestration", "ui"];

function walkDir(dir, callback) {
    fs.readdirSync(dir).forEach(f => {
        const dirPath = path.join(dir, f);
        if (f === '.git' || f === 'node_modules' || f === 'build') return;
        const isDirectory = fs.statSync(dirPath).isDirectory();
        isDirectory ? walkDir(dirPath, callback) : callback(dirPath);
    });
}

function fixIncludes(filepath) {
    if (!filepath.match(/\.(c|cpp|h|hpp)$/)) return;
    
    const relPath = path.relative(ROOT, filepath);
    const parts = relPath.replace(/\\/g, '/').split('/');
    const depth = parts.length - 1;
    const rootPrefix = depth > 0 ? "../".repeat(depth) : "./";

    let lines = fs.readFileSync(filepath, 'utf8').split('\n');
    let changed = false;

    lines = lines.map(line => {
        const match = line.match(/^#include\s*(["<])([^">]+)([">])/);
        if (match) {
            const openQuote = match[1];
            const incPath = match[2];
            const closeQuote = match[3];

            let cleanPath = incPath.replace(/^(\.\.\/)+/, '').replace(/^\.\//, '');
            const base = cleanPath.split('/')[0];

            let newInc = null;
            if (base === "include") {
                newInc = rootPrefix + cleanPath;
            } else if (INCLUDE_ROOT_FILES.includes(base)) {
                newInc = rootPrefix + "include/" + cleanPath;
            } else if (INCLUDE_DIRS.includes(base)) {
                newInc = rootPrefix + "include/" + cleanPath;
            } else if (cleanPath.includes("include/")) {
                newInc = rootPrefix + cleanPath;
            }

            if (newInc) {
                const newLine = `#include ${openQuote}${newInc}${closeQuote}`;
                if (newLine !== line) {
                    changed = true;
                    return newLine;
                }
            }
        }
        return line;
    });

    if (changed) {
        fs.writeFileSync(filepath, lines.join('\n'));
        console.log(`Fixed: ${relPath} (depth ${depth})`);
    }
}

walkDir(ROOT, fixIncludes);
