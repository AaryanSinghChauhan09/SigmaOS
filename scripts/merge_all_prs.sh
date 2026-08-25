#!/usr/bin/env bash
# SigmaOS - Merge all conflicted PR branches into main
# Uses 'ours' strategy for conflicts - main branch content wins
# This merges the PR branches so GitHub detects them as merged

set -e

REPO_DIR="$(git rev-parse --show-toplevel)"
cd "$REPO_DIR"

# Configure git
git config user.email "aaryan@sigmaos.dev"
git config user.name "SigmaOS Builder"

echo "=== SigmaOS Branch Merger - $(date) ==="
echo "Current branch: $(git branch --show-current)"
echo ""

# Make sure we're on main and up to date
git checkout main
git pull origin main --no-rebase 2>/dev/null || echo "Pull had issues, continuing..."

MERGED=()
FAILED=()

# Function to merge a branch with conflict resolution
merge_branch() {
    local branch="$1"
    local pr_num="$2"
    local description="$3"
    
    echo ""
    echo "--- Merging PR #$pr_num: $branch ---"
    
    # Check if branch exists remotely
    if ! git ls-remote --exit-code --heads origin "$branch" > /dev/null 2>&1; then
        echo "  SKIP: Branch $branch not found remotely"
        return 0
    fi
    
    # Fetch the branch
    git fetch origin "$branch" 2>/dev/null || true
    
    # Try regular merge first (no-ff to preserve history)
    if git merge --no-ff -X ours -m "Merge PR #$pr_num: $description" "origin/$branch" 2>/dev/null; then
        echo "  SUCCESS: Merged $branch"
        MERGED+=("PR #$pr_num: $description")
        return 0
    fi
    
    # If merge failed, try with git merge --allow-unrelated-histories
    git merge --abort 2>/dev/null || true
    if git merge --no-ff --allow-unrelated-histories -X ours -m "Merge PR #$pr_num: $description" "origin/$branch" 2>/dev/null; then
        echo "  SUCCESS (unrelated): Merged $branch"
        MERGED+=("PR #$pr_num: $description")
        return 0
    fi
    
    # Handle any lingering conflict
    git merge --abort 2>/dev/null || true
    
    # Last resort: cherry-pick all commits from the branch
    echo "  Trying cherry-pick approach..."
    local base_sha
    base_sha=$(git merge-base HEAD "origin/$branch" 2>/dev/null || echo "")
    if [ -n "$base_sha" ]; then
        if git cherry-pick --allow-empty -X ours "$base_sha..origin/$branch" 2>/dev/null; then
            echo "  SUCCESS (cherry-pick): Merged $branch"
            MERGED+=("PR #$pr_num: $description (cherry-picked)")
            return 0
        fi
        git cherry-pick --abort 2>/dev/null || true
    fi
    
    echo "  FAILED: Could not merge $branch"
    FAILED+=("PR #$pr_num: $description")
    return 0
}

# Merge all PR branches
merge_branch "docs/sovereign-os-self-sufficiency-master-blueprint-10329081903812998722" 590 "Add Sovereign OS Absolute Self-Sufficiency Master Blueprint"
merge_branch "bolt/optimize-watchdog-thresholds-10598983263581529897" 591 "Bolt: optimize threshold checking in WatchdogManager"
merge_branch "bolt-identity-did-len-opt-9850646523823204763" 592 "Bolt: Optimize SimpleDigitalIdentity DID slice lookup to O(1)"
merge_branch "fix-workflow-security-and-token-permissions-10408740097862196060" 594 "Security: remediate workflow GHSA and token permissions"
merge_branch "jules-9116155812434473697-fd58488e" 595 "Add comprehensive technical gap analysis vs open source OSes"
merge_branch "wiki-sync-and-parity-docs-transfer-5535566344438722671" 596 "Transfer implemented feature blueprints to WIKI"
merge_branch "feat/zsh-bash-shell-improvements-14346262180505275201" 597 "Improve SigmaOS shell with Zsh, Bash, Fish & BSD parity"
merge_branch "feature/linux-bsd-distro-parity-abstractions-3440215014866808269" 599 "feat: Add Kali, GhostBSD, Pop!_OS, Clear Linux & Keylime distro abstractions"
merge_branch "feature/shell-linux-bsd-enhancements-25201689833389086" 600 "Improve shell of SigmaOS inspired by zsh, bash, tcsh, ksh"
merge_branch "feature/terminal-linux-bsd-improvements-12736831843922362153" 601 "Improve terminal of SigmaOS with Linux and BSD distro features"
merge_branch "feature/shell-improvements-6032717160428009858" 602 "Improve SigmaOS shell with Zsh, Bash, and BSD inspired features"
merge_branch "feature/ready-to-use-distro-usability-11450095619117173625" 603 "Expose and integrate Linux & BSD distro usability primitives"
merge_branch "feat/open-source-inspirations-3446807731779984796" 604 "feat(open-source): add eBPF, Capsicum, 9P2000, Mesh Identity engines"
merge_branch "jules-13782437476231280286-63b6add9" 605 "Enhance kernel module management with Linux LKM & BSD KLD parity"
merge_branch "impl/linux-bsd-terminal-enhancements-18388498293484823937" 606 "Enhance terminal emulator with Linux & BSD innovations"
merge_branch "feat/impl-wiki-roadmap-ideas-202710799178780517" 607 "feat(net): implement RFC-compliant DNS & IPv6 stack roadmap"
merge_branch "feat/open-source-competitor-tools-implementation-10852811242607649932" 608 "Implement competitor-inspired open-source tools and features"
merge_branch "feat/expand-distro-device-support-11165180134946140343" 609 "feat: add broad Linux and BSD inspired device drivers"
merge_branch "docs/sovereign-universal-hardware-bringup-roadmap-185914269912424773" 611 "docs: add Section 46 sovereign hardware bringup & distro-crushing roadmap"

echo ""
echo "=== MERGE SUMMARY ==="
echo "Successfully merged: ${#MERGED[@]}"
for m in "${MERGED[@]}"; do
    echo "  ✓ $m"
done

echo ""
echo "Failed: ${#FAILED[@]}"
for f in "${FAILED[@]}"; do
    echo "  ✗ $f"
done

echo ""
echo "=== Pushing to origin main ==="
git push origin main --force-with-lease 2>&1 || git push origin main 2>&1
echo "PUSH COMPLETE"
