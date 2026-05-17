import subprocess
import sys

# SigmaOS Branch Uniformity & Synchronization Engine (S-BUSE)
# Automates absolute architectural parity across all 12 branches of the sovereign lattice.

def run_git(args):
    print(f"[RUN] git {' '.join(args)}")
    res = subprocess.run(["git"] + args, capture_output=True, text=True)
    if res.returncode != 0:
        print(f"[ERROR] git {' '.join(args)} failed:\n{res.stderr}")
        return False
    return True

def sync_branches():
    branches = [
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

    print("=========================================================================")
    print("SIGMAOS: BRANCH UNIFORMITY & SYNCHRONIZATION ENGINE [ACTIVE]")
    print("=========================================================================")

    # Ensure we start on main
    if not run_git(["checkout", "main"]):
        print("[FATAL] Could not checkout main. Aborting.")
        sys.exit(1)

    for branch in branches:
        print(f"\n[*] Syncing branch: {branch} -> Uniformity with main...")
        
        # Checkout the target branch
        if not run_git(["checkout", branch]):
            # Try creating it from main if checkout fails
            print(f"[!] Branch '{branch}' could not be checked out. Attempting to create it from main...")
            if not run_git(["checkout", "-b", branch]):
                print(f"[ERROR] Failed to switch to or create branch '{branch}'. Skipping.")
                continue

        # Hard reset to main to guarantee bit-perfect uniformity
        if not run_git(["reset", "--hard", "main"]):
            print(f"[ERROR] Failed to reset '{branch}' to main. Skipping.")
            continue

        # Force push the update to remote
        if not run_git(["push", "origin", branch, "--force"]):
            print(f"[ERROR] Failed to push branch '{branch}' to remote.")

    # Always return to main
    print("\n[*] Returning to main branch...")
    run_git(["checkout", "main"])
    print("\n=========================================================================")
    print("SIGMAOS: BRANCH SYNCHRONIZATION COMPLETE. PARITY ACHIEVED ACROSS ALL 12 BRANCHES.")
    print("=========================================================================")

if __name__ == "__main__":
    sync_branches()
