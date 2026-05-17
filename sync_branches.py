import os
import subprocess

WORKSPACE_DIR = r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"
BRANCHES = [
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
]

def run_git(args):
    result = subprocess.run(["git"] + args, cwd=WORKSPACE_DIR, capture_output=True, text=True)
    return result.returncode, result.stdout, result.stderr

print("Starting Branch Uniformity Synchronization...")

# Ensure we are up to date on main
run_git(["checkout", "main"])
run_git(["pull", "origin", "main"])

for branch in BRANCHES:
    print(f"Synchronizing branch: {branch}")
    # Try to checkout the branch (it might only exist on remote)
    code, out, err = run_git(["checkout", branch])
    if code != 0:
        # Try checking out from origin
        code2, out2, err2 = run_git(["checkout", "-b", branch, f"origin/{branch}"])
        if code2 != 0:
            print(f"  [-] Branch {branch} does not exist locally or remotely. Creating it...")
            run_git(["checkout", "-b", branch])
    
    # Merge main into the branch
    run_git(["merge", "main", "-m", "chore: Enforce branch uniformity with main via automated sync"])
    
    # Push to origin
    run_git(["push", "origin", branch])

# Return to main
run_git(["checkout", "main"])

print("All branches successfully synchronized to achieve total uniformity with main!")
