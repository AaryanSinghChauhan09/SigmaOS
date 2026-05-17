import fs from 'fs';
import path from 'path';
import crypto from 'crypto';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function hashFile(filepath) {
    try {
        const fileBuffer = fs.readFileSync(filepath);
        const hashSum = crypto.createHash('sha256');
        hashSum.update(fileBuffer);
        return hashSum.digest('hex');
    } catch (e) {
        return null;
    }
}

function walkDir(dir, callback) {
    if (dir.includes('.git') || dir.includes('node_modules') || dir.includes('wiki_repo')) {
        return;
    }
    let files;
    try {
        files = fs.readdirSync(dir);
    } catch (e) {
        return;
    }
    for (const file of files) {
        const filepath = path.join(dir, file);
        let stat;
        try {
            stat = fs.statSync(filepath);
        } catch (e) {
            continue;
        }
        if (stat.isDirectory()) {
            callback(filepath, true);
            walkDir(filepath, callback);
        } else if (stat.isFile()) {
            callback(filepath, false);
        }
    }
}

function scanShards(rootDir) {
    console.log(`[*] Scanning ${rootDir} for legacy shards and duplicates...\n`);
    
    const fileHashes = {};
    const legacyShards = [];
    
    walkDir(rootDir, (itemPath, isDir) => {
        const basename = path.basename(itemPath);
        if (isDir) {
            if (basename.startsWith('S') && basename.includes('_') && /\d/.test(basename)) {
                legacyShards.push(itemPath);
            }
        } else {
            const fhash = hashFile(itemPath);
            if (fhash) {
                if (!fileHashes[fhash]) {
                    fileHashes[fhash] = [];
                }
                fileHashes[fhash].push(itemPath);
            }
        }
    });

    console.log("=== Legacy Shard Directories Detected ===");
    if (legacyShards.length === 0) {
        console.log("  None detected! Architecture is clean.");
    } else {
        for (const shard of legacyShards) {
            console.log(`  [X] Mark for deprecation: ${shard}`);
        }
    }
            
    console.log("\n=== Exact File Duplicates Detected ===");
    let duplicatesFound = false;
    for (const [fhash, paths] of Object.entries(fileHashes)) {
        if (paths.length > 1) {
            duplicatesFound = true;
            console.log(`\n  Duplicate Cluster (${fhash.substring(0, 8)}...):`);
            for (const p of paths) {
                console.log(`    -> ${p}`);
            }
        }
    }
                
    if (!duplicatesFound) {
        console.log("  No duplicate files found.");
    }

    console.log("\n[+] Audit complete. Run `s-deps prune` to finalize removals.");
}

scanShards(".");
