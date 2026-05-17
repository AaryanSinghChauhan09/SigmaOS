import { spawnSync } from 'child_process';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const WORKSPACE_DIR = __dirname;
const WIKI_DIR = path.join(WORKSPACE_DIR, "wiki_repo");

function runGit(args, cwd = WORKSPACE_DIR) {
    console.log(`[RUN] git ${args.join(' ')} inside ${cwd}`);
    try {
        const res = spawnSync("git", args, { cwd, stdio: 'inherit' });
        return res.status === 0;
    } catch (e) {
        console.error(`[ERROR] git failed: ${e.message}`);
        return false;
    }
}

// Sync Wiki
console.log("Synchronizing Wiki Repository...");
runGit(["add", "."], WIKI_DIR);
runGit(["commit", "-m", "Enforce strict zero-dependency documentation"], WIKI_DIR);
runGit(["push", "origin", "main"], WIKI_DIR);

// Sync Main Repo
console.log("Synchronizing Main Repository...");
runGit(["add", "."]);
runGit(["commit", "-m", "Enforce absolute zero-dependency on high-level languages & pre-defined functions"]);
runGit(["push", "origin", "--all"]);

console.log("Final Zero-Dependency Synchronization Complete!");
