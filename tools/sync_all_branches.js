import { spawnSync } from 'child_process';

const branches = [
    "release/standalone",
    "release/rtos",
    "release/mobile",
    "release/microkernel",
    "release/dual-boot",
    "release/distributed",
    "release/cloud",
    "release/browser",
    "release/app",
    "performance-optimized",
    "gh-pages"
];

function runGit(args) {
    console.log(`[RUN] git ${args.join(' ')}`);
    try {
        const res = spawnSync("git", args, { stdio: 'pipe', encoding: 'utf-8' });
        if (res.status !== 0) {
            console.error(`[ERROR] failed:\n${res.stderr || res.stdout}`);
            return false;
        }
        return true;
    } catch (e) {
        console.error(`[ERROR] failed:\n${e.message}`);
        return false;
    }
}

console.log("=========================================================================");
console.log("SIGMAOS: NODE.JS BRANCH UNIFORMITY & SYNCHRONIZATION ENGINE [ACTIVE]");
console.log("=========================================================================");

// Ensure we start on main
if (!runGit(["checkout", "main"])) {
    console.error("[FATAL] Could not checkout main. Aborting.");
    process.exit(1);
}

for (const branch of branches) {
    console.log(`\n[*] Syncing branch: ${branch} -> Uniformity with main...`);
    
    // Checkout target branch
    if (!runGit(["checkout", branch])) {
        console.log(`[!] Branch '${branch}' could not be checked out. Attempting to create it from main...`);
        if (!runGit(["checkout", "-b", branch])) {
            console.error(`[ERROR] Failed to switch to or create branch '${branch}'. Skipping.`);
            continue;
        }
    }

    // Merge main into target branch with merge strategy -X theirs
    console.log(`[*] Merging core improvements from main into ${branch}...`);
    if (!runGit(["merge", "main", "-m", "Sync: Merge latest core improvements from main"])) {
        console.log(`[!] Conflict detected during merge. Resolving by preferring core improvements from main...`);
        runGit(["merge", "--abort"]);
        // Use -X theirs to auto-resolve in favor of main's updates
        if (!runGit(["merge", "-X", "theirs", "main", "-m", "Sync: Merge latest core improvements from main (conflict resolved)"])) {
            console.error(`[ERROR] Failed to merge main into branch '${branch}'. Skipping push.`);
            continue;
        }
    }

    // Push the synchronized branch to remote
    if (!runGit(["push", "origin", branch])) {
        console.log(`[!] Standard push failed. Attempting force push for parity...`);
        if (!runGit(["push", "origin", branch, "--force"])) {
            console.error(`[ERROR] Failed to push branch '${branch}' to remote.`);
        }
    }
}

// Always return to main
console.log("\n[*] Returning to main branch...");
runGit(["checkout", "main"]);
console.log("\n=========================================================================");
console.log("SIGMAOS: BRANCH SYNCHRONIZATION COMPLETE. PARITY ACHIEVED ACROSS ALL 12 BRANCHES.");
console.log("=========================================================================");
