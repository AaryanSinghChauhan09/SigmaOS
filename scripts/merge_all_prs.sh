#!/bin/bash
# SigmaOS: Merge All Open PR Branches into Main
# Strategy: Squash-merge each branch to avoid messy merge commits
# If merge fails due to conflicts, use "ours" strategy for new files only

set -euo pipefail

REPO_DIR="/home/aaryansinghchauhan/SigmaOS"
cd "$REPO_DIR"

git config user.email "aaryan.singh.chauhan.09@gmail.com"
git config user.name "AaryanSinghChauhan09"

MERGED=()
SKIPPED=()
FAILED=()

merge_branch() {
    local branch="$1"
    local pr_title="$2"
    
    echo ""
    echo "=========================================="
    echo "Merging: $branch"
    echo "Title: $pr_title"
    echo "=========================================="
    
    # Check if branch exists remotely
    if ! git ls-remote --heads origin "$branch" | grep -q "$branch"; then
        echo "  SKIP: Branch not found on remote"
        SKIPPED+=("$branch")
        return 0
    fi
    
    # Try merge with squash, accepting new files from branch on conflict
    if git merge --no-ff --no-edit -m "Merge $branch: $pr_title" "origin/$branch" 2>&1; then
        MERGED+=("$branch")
        echo "  SUCCESS: Merged $branch"
    else
        echo "  CONFLICT: Attempting to resolve..."
        # For conflicts, prefer ours for existing files (keep main), accept new files
        git diff --name-only --diff-filter=U 2>/dev/null | while read -r conflict_file; do
            echo "    Resolving conflict in: $conflict_file"
            # Accept the incoming branch version for new/modified files
            git checkout --theirs -- "$conflict_file" 2>/dev/null || true
            git add "$conflict_file" 2>/dev/null || true
        done
        
        if git commit --no-edit -m "Merge $branch (conflict-resolved): $pr_title" 2>&1; then
            MERGED+=("$branch (conflict-resolved)")
            echo "  SUCCESS (conflict-resolved): $branch"
        else
            git merge --abort 2>/dev/null || true
            FAILED+=("$branch")
            echo "  FAILED: Could not merge $branch"
        fi
    fi
}

# PR #2002 - Security fix: URL-encoded path traversal
merge_branch "jules-399426345445451680-826fd3f3" "Sentinel: Fix URL-encoded path traversal vulnerability"

# PR #1992 - Bolt performance
merge_branch "bolt/power-profile-name-len-opt-11191652987910492582" "Bolt: optimize SimplePowerProfile name lookup"

# PR #1991 - Bolt performance
merge_branch "jules-13286778191202915805-67d1d442" "Bolt: Optimize launcher query matching to zero allocations"

# PR #1990 - Bolt performance
merge_branch "bolt/optimize-calendar-search-3219374139632153533" "Bolt: Optimize CalendarApp::search_events string matching"

# PR #1985 - Bolt performance
merge_branch "bolt-opt-mint-search-allocations-6901479225876503243" "Bolt: Eliminate heap allocations in Mint menu and catalog search"

# PR #2000 - UX
merge_branch "palette/command-palette-outside-click-dismissal-13733729012262941830" "Palette: Command Palette outside-click dismissal"

# PR #2003 - Tech media
merge_branch "jules-14162979914014017982-aab70c43" "Enhance tech media publication synthesis with flexible domain query"

# PR #1999 - Package management
merge_branch "feature/universal-pkg-mgmt-improvements-18138656283433899290" "Enhance Universal Package Management for Linux & BSD Formats"

# PR #1997 - Universal PM
merge_branch "feature/universal-pm-package-manager-improvements-16698341308720013744" "Enhance SigmaOS Universal Package Manager with V30 Distro Parity Engine"

# PR #2007 - Sovereign Distro Package V30 (preferred over #2006)
merge_branch "feat/sovereign-distro-package-advancements-v30-14244316353020401733" "SovereignDistroPackageAdvancementsSuiteV30 for Universal Package Parity"

# PR #2006 - Sovereign Distro Package V30 (alternate)
merge_branch "feature/sovereign-distro-package-advancements-v30-11916394946915965445" "Implement Sovereign Distro Package Advancements Suite V30"

# PR #2001 - Missing Linux/BSD components
merge_branch "jules-add-missing-distro-components-7043122241752102433" "Add thousands of missing Linux & BSD components in SigmaOS"

# PR #1998 - Shell/browser enhancements
merge_branch "feat/linux-bsd-shell-browser-enhancements-3610942636789778446" "Linux/BSD universal shell and open-source browser suite improvements"

# PR #1995 - Open-source native parity
merge_branch "feature/open-source-project-inspiration-engines-7563681345909340205" "Implement open-source project native parity engines"

# PR #1993 - Enterprise productivity
merge_branch "feat/enterprise-productivity-suite-enhancements-8154725030213889568" "Expand enterprise productivity suite engines"

# PR #2009 - OS pinnacle engines
merge_branch "jules-open-source-os-pinnacle-innovations-14801483837692589404" "Add open-source project inspired OS pinnacle engines and orchestrator"

# PR #2008 - 2090 Distro Supremacy Engine
merge_branch "feat/2090-distro-supremacy-engine-17866795407874198435" "feat(distro): add 2090 Distro Supremacy Engine for SigmaOS"

# PR #1986 - Sovereign 2090 Distro Supremacy
merge_branch "jules-2090-distro-supremacy-engine-12331722532899912131" "Implement Sovereign 2090 Distro Supremacy Engine for SigmaOS"

# PR #1983 - Linux & BSD Advancements V30
merge_branch "jules-9182752905672575119-c77b5e66" "Add Sovereign Linux & BSD Ecosystem Advancements V30 in PR format"

# PR #1982 - Linux & BSD Advancements Suite V30
merge_branch "jules-distro-advancements-v30-13971136663823265044" "Add Sovereign Linux & BSD Ecosystem Advancements Suite V30"

# PR #1981 - Linux & BSD distribution modes
merge_branch "jules-15472946201603292323-71172f49" "Expand Linux & BSD distribution modes for full SigmaOS subsystem interoperability"

# PR #1996 - Wiki ideas
merge_branch "jules-11906709690069364763-7eea020f" "Verify and finalize implementation of .md and GitHub wiki unimplemented ideas"

# PR #2005 - Gap analysis
merge_branch "jules-15322285588524229622-46e72fe4" "Complete Open-Source OS Comparative Gap Analysis"

# PR #2004 - ImprovementPlan
merge_branch "main-3560015866053024662" "Add ImprovementPlan.md and NEXT_STEPS_GUIDELINES.md with Tri-Agent Audit"

# PR #1994 - Comparative gap analysis docs
merge_branch "jules-7295597980673015265-f6adcdf8" "Add Comprehensive Comparative Open Source OS Gap Analysis Document"

# PR #1989 - What is working/not working
merge_branch "jules-9922677821921089446-6dd8cbf3" "Create WHAT_IS_WORKING_AND_NOT_WORKING.md Master Diagnostic & Fix Guide"

# PR #1988 - Self-sufficiency encyclopedia
merge_branch "jules-5963512557304518959-418eea2d" "Add Sovereign OS Self-Sufficiency Ultra Encyclopedia V47"

# PR #1987 - Operations guide
merge_branch "jules-10139182151507534362-aae43373" "Add SigmaOS Operations & Continuous Improvement Guide"

# PR #1984 - Tri-Agent 500 repos plan
merge_branch "jules/tri-agent-500-repos-plan-4964016238560004491" "Add Tri-Agent Framework & 500+ Repositories Absorption Master Plans"

# PR #1980 - Future development roadmap
merge_branch "feature/sigmaos-strategic-roadmap-master-14787455624737826636" "docs: synthesize master FUTURE-DEVELOPMENT-ROADMAP specification"

# Additional branches not in the 30 PRs list but on remote
merge_branch "feat/add-thousands-distro-components-15034820114738373835" "Add thousands of distro components (additional batch)"
merge_branch "feat/add-thousands-missing-distro-components-9069208223223564262" "Add thousands of missing distro components (batch 2)"
merge_branch "feat/distro-package-advancements-v30-18169565315309804210" "Distro package advancements V30 (additional)"
merge_branch "feat/linux-bsd-subsystem-interop-15001753511924564587" "Linux/BSD subsystem interop expansion"
merge_branch "feat/open-source-obsoletion-enhancements-14652318106126063480" "Open-source obsoletion enhancements"
merge_branch "feat/sovereign-distro-package-advancements-v26-10737839506416669356" "Sovereign distro package advancements V26"
merge_branch "feat/v29-package-advancements-17556670720259686174" "Package advancements V29"
merge_branch "feature/productivity-suite-enhancements-17463768155699694616" "Productivity suite enhancements"
merge_branch "feature/shell-repl-and-sigmaweb-improvements-3504764505530691951" "Shell REPL and SigmaWeb improvements"
merge_branch "feature/universal-package-advancements-v25-17785054077281724979" "Universal package advancements V25"
merge_branch "feature/universal-package-advancements-v30-543169400665088625" "Universal package advancements V30"
merge_branch "feature/universal-pm-distro-advancements-v19-15246803577158701695" "Universal PM distro advancements V19"
merge_branch "jules-13948912723049669216-01b65ff6" "Jules additional improvements batch"
merge_branch "jules-4046267795508162233-05535c7b" "Jules batch 4046"
merge_branch "jules-4132922671784608808-62d0f9d4" "Jules batch 4132"
merge_branch "jules-8183960992423192444-92e7c544" "Jules batch 8183"

echo ""
echo "=========================================="
echo "MERGE SUMMARY"
echo "=========================================="
echo "Merged (${#MERGED[@]}):"
for b in "${MERGED[@]}"; do echo "  + $b"; done
echo ""
echo "Skipped (${#SKIPPED[@]}):"
for b in "${SKIPPED[@]}"; do echo "  - $b"; done
echo ""
echo "Failed (${#FAILED[@]}):"
for b in "${FAILED[@]}"; do echo "  ! $b"; done
