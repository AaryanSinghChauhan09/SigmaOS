#!/usr/bin/env python3
"""Batch merge all remote branches into main with sovereign conflict resolution."""
import subprocess, sys

OURS_FILES = {
    "Cargo.toml", "src/lib.rs", "src/package/universal.rs",
    "src/sigpkg/universal_adapter.rs", "src/sigpkg/universal_engine.rs",
    "src/distro/linux_bsd_distro_gaps.rs", "src/distro/omarchy.rs",
    "src/distro/void_runit.rs", "src/filesystem/manager.rs", "src/klib/btreemap.rs",
    "src/package/bsd_linux_package_innovations.rs", "src/package/sigma_pkg.rs",
    "src/security/mod.rs", "src/compatibility/mod.rs", "src/kernel/mod.rs",
    "src/tools/mod.rs", "src/distro/mod.rs",
}

THEIRS_FILES = {
    "src/sigpkg/universal_oop_system.rs",
    "src/distro/linux_bsd_inspirations.rs",
    "src/compatibility/fedora.rs",
    "tests/test_universal_adapter.rs",
}

def run(cmd):
    return subprocess.run(cmd, shell=True, capture_output=True, text=True)

def get_branches():
    res = run("git branch -r | grep -v 'HEAD\\|/main$'")
    return [b.strip() for b in res.stdout.strip().split('\n') if b.strip()]

def resolve_conflicts():
    # Handle modify/delete
    unmerged = run("git status --porcelain").stdout.splitlines()
    for line in unmerged:
        if line.startswith("DU ") or line.startswith("UD "):
            f = line[3:].strip()
            run(f'git rm -f "{f}"')
            print(f"  Removed deleted: {f}")

    conflicts = run("git diff --name-only --diff-filter=U").stdout.splitlines()
    for f in [x.strip() for x in conflicts if x.strip()]:
        if f in OURS_FILES or f.startswith(".github/workflows/"):
            run(f'git checkout --ours "{f}" && git add "{f}"')
            print(f"  --ours: {f}")
        elif f in THEIRS_FILES:
            run(f'git checkout --theirs "{f}" && git add "{f}"')
            print(f"  --theirs: {f}")
        else:
            run(f'git checkout --ours "{f}" && git add "{f}"')
            print(f"  --ours(fallback): {f}")

def main():
    branches = get_branches()
    print(f"Found {len(branches)} remote branches to merge")
    
    merged, failed = 0, 0
    for rb in branches:
        branch_name = rb.replace("origin/", "")
        print(f"\n=== Merging {rb} ===")
        
        res = run(f'git merge --no-ff -m "merge: integrate {rb}" {rb}')
        if res.returncode == 0:
            print(f"  Clean merge")
            merged += 1
            continue
        
        # Check if there are actual conflicts or just an error
        check = run("git diff --name-only --diff-filter=U")
        status = run("git status --porcelain")
        if not check.stdout.strip() and not any(
            l.startswith("DU ") or l.startswith("UD ") or l.startswith("UU ") or l.startswith("AA ")
            for l in status.stdout.splitlines()
        ):
            print(f"  Merge failed (not a conflict), aborting")
            run("git merge --abort")
            failed += 1
            continue
        
        resolve_conflicts()
        
        remaining = run("git diff --name-only --diff-filter=U").stdout.strip()
        if remaining:
            print(f"  Unresolved conflicts remain, aborting: {remaining}")
            run("git merge --abort")
            failed += 1
            continue
        
        commit = run('git commit --no-edit')
        if commit.returncode != 0:
            commit = run(f'git commit -m "merge: integrate {rb} (resolved)"')
        if commit.returncode == 0:
            print(f"  Merged successfully")
            merged += 1
        else:
            print(f"  Commit failed, aborting")
            run("git merge --abort")
            failed += 1
    
    print(f"\n=== Summary: {merged} merged, {failed} failed out of {len(branches)} ===")

if __name__ == "__main__":
    main()
