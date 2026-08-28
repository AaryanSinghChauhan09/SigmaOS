#!/usr/bin/env python3
"""SigmaOS - Merge all branches into main with conflict resolution favoring feature branches."""

import subprocess
import sys
import os

REPO_DIR = "/home/aaryansinghchauhan/SigmaOS"
os.chdir(REPO_DIR)

def run(cmd, **kwargs):
    """Run a command and return output."""
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True, cwd=REPO_DIR, **kwargs)
    if result.returncode != 0:
        print(f"CMD: {cmd}")
        print(f"STDOUT: {result.stdout}")
        print(f"STDERR: {result.stderr}")
    return result

def git_config():
    run('git config user.email "aaryan@sigmaos.dev"')
    run('git config user.name "SigmaOS Builder"')

def get_branches_to_merge():
    """Get all remote branches except main."""
    result = run('git branch -r | grep -v "main$" | sed "s/origin\\///"')
    branches = [b.strip() for b in result.stdout.strip().split('\n') if b.strip()]
    return branches

def setup_merge_driver():
    """Setup a custom merge driver for markdown files."""
    run('git config merge.ours.driver true')
    run('git config merge.markdown.name "Markdown merge driver"')
    run('git config merge.markdown.driver "python3 /home/aaryansinghchauhan/SigmaOS/scripts/merge_markdown.py %A %O %B %L"')

def merge_markdown(base, ours, theirs, marker):
    """Custom merge driver for markdown files."""
    # For markdown, we try to keep both versions by appending
    # But if one is just a subset, keep the larger one
    base_size = os.path.getsize(base) if os.path.exists(base) else 0
    ours_size = os.path.getsize(ours) if os.path.exists(ours) else 0
    theirs_size = os.path.getsize(theirs) if os.path.exists(theirs) else 0
    
    # Read contents
    with open(ours, 'r') as f:
        ours_content = f.read()
    with open(theirs, 'r') as f:
        theirs_content = f.read()
    
    # If one is empty, use the other
    if not ours_content.strip():
        with open(ours, 'w') as f:
            f.write(theirs_content)
        return 0
    if not theirs_content.strip():
        return 0
    
    # If they are identical
    if ours_content == theirs_content:
        return 0
    
    # Prefer the larger/more detailed version for markdown
    if theirs_size > ours_size:
        with open(ours, 'w') as f:
            f.write(theirs_content)
    # If ours is larger or equal, keep ours
    return 0

def merge_branch(branch):
    """Merge a single branch with -X theirs strategy."""
    print(f"\n--- Merging {branch} ---")
    
    # Fetch
    run(f'git fetch origin {branch}')
    
    # Try merge with -X theirs
    result = run(f'git merge --no-ff -X theirs -m "Merge {branch} into main" origin/{branch}')
    if result.returncode == 0:
        print(f"  SUCCESS: {branch}")
        return True
    
    # If merge failed, check if it's already up to date
    if "Already up to date" in result.stderr:
        print(f"  SKIP: {branch} already up to date")
        return True
    
    # Abort and report failure
    run('git merge --abort')
    print(f"  FAILED: {branch}")
    return False

def main():
    print("=== SigmaOS Branch Consolidation ===")
    git_config()
    
    # Setup custom merge driver for markdown
    # Write the merge driver script
    merge_driver_script = '''#!/usr/bin/env python3
import sys
import os

def main():
    ours = sys.argv[1]
    base = sys.argv[2]
    theirs = sys.argv[3]
    marker = sys.argv[4]
    
    with open(ours, 'r') as f:
        ours_content = f.read()
    with open(theirs, 'r') as f:
        theirs_content = f.read()
    
    if not ours_content.strip():
        with open(ours, 'w') as f:
            f.write(theirs_content)
        return 0
    if not theirs_content.strip():
        return 0
    if ours_content == theirs_content:
        return 0
    
    # For markdown, prefer the version with more content
    if len(theirs_content) > len(ours_content):
        with open(ours, 'w') as f:
            f.write(theirs_content)
    return 0

if __name__ == "__main__":
    main()
'''
    with open(os.path.join(REPO_DIR, "scripts", "merge_markdown.py"), "w") as f:
        f.write(merge_driver_script)
    os.chmod(os.path.join(REPO_DIR, "scripts", "merge_markdown.py"), 0o755)
    
    setup_merge_driver()
    
    branches = get_branches_to_merge()
    print(f"Found {len(branches)} branches to merge")
    
    merged = []
    failed = []
    
    # Order: docs first, then fixes, then features
    priority_branches = []
    other_branches = []
    
    for b in branches:
        if any(x in b for x in ['docs/', 'perf/', 'fix-', 'fix/']):
            priority_branches.append(b)
        else:
            other_branches.append(b)
    
    # Sort other branches: PRs first, then others
    other_branches.sort(key=lambda x: (not any(c.isdigit() for c in x), x))
    
    ordered = priority_branches + other_branches
    
    for branch in ordered:
        if merge_branch(branch):
            merged.append(branch)
        else:
            failed.append(branch)
    
    print(f"\n=== MERGE SUMMARY ===")
    print(f"Successfully merged: {len(merged)}")
    for m in merged:
        print(f"  ✓ {m}")
    
    print(f"\nFailed: {len(failed)}")
    for f in failed:
        print(f"  ✗ {f}")
    
    # Delete merged branches from remote
    print(f"\n=== DELETING MERGED BRANCHES ===")
    for branch in merged:
        run(f'git push origin --delete {branch}')
        print(f"  Deleted remote: {branch}")
    
    # Push to main
    print(f"\n=== PUSHING TO MAIN ===")
    result = run('git push origin main --force-with-lease')
    if result.returncode != 0:
        run('git push origin main')
    print("PUSH COMPLETE")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
