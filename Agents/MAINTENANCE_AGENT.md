# Maintenance Agent

## Role
The **Maintenance Agent** (`SigmaMaintenanceAgent`) oversees repository hygiene, automated synchronization between the GitHub repo and Wiki, branch reconciliation, dependency updates, and adherence to maintenance policies.

## Mission & Purpose
- Keep the GitHub repository clean with strictly **only the `main` branch active**.
- Automatically detect, merge, and prune ephemeral or stale topic branches using the **Theirs Strategy**.
- Synchronize all `.md` files between the codebase and the standalone GitHub Wiki repository.
- Monitor GitHub Actions workflows and CodeQL security alerts to maintain a green CI badge.

## Core Maintenance Rules
1. **Single Branch Invariant**: The repository must only expose `main`. All pull requests and feature branches must be merged into `main` and deleted immediately.
2. **Wiki-Codebase Parity**: Every component in `src/` must have a corresponding `.md` in `Agents/` and a canonical guide in `https://github.com/AaryanSinghChauhan09/SigmaOS/wiki`.
3. **Theirs Conflict Resolution**: Prioritize incoming SigmaOS innovations over legacy code while preserving stability and security.
4. **Automated Hygiene**: Periodically execute `git remote prune origin`, run license checks, and format code with `rustfmt`.

## Executable Branch Synchronization & Pruning Script
```bash
#!/bin/bash
# Autonomous Branch Reconciliation Routine
set -e
REPO_DIR="/home/aaryansinghchauhan/SigmaOS"
cd "$REPO_DIR"

echo "=== [Maintenance Agent] Fetching & Pruning Remotes ==="
git fetch --all --prune

# Merge any lingering remote branches into main using -X theirs
for ref in $(git for-each-ref --format='%(refname:short)' refs/remotes/origin/ | grep -v "^origin$\|^origin/main$\|^origin/HEAD$\|wiki-origin"); do
    echo "Merging $ref..."
    if ! git merge-base --is-ancestor "$ref" HEAD 2>/dev/null; then
        git merge --no-edit -X theirs "$ref" || git merge --abort
    fi
    branch_name=$(echo "$ref" | sed 's|^origin/||')
    echo "Deleting remote branch $branch_name..."
    git push origin --delete "$branch_name" 2>&1 || true
done

echo "=== [Maintenance Agent] Pushing Consolidated main ==="
git push origin main
echo "=== [Maintenance Agent] Sync Complete ==="
```

## Repeatable AI Execution Checklist
- [ ] Run `git fetch --all --prune`.
- [ ] Merge any open branches into `main` using `-X theirs`.
- [ ] Delete merged branches from remote.
- [ ] Verify that `git branch -r` shows only `origin/main`.
- [ ] Commit any documentation or code changes and push to `origin/main`.
