#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# SigmaOS — Git commit-msg hook
#
# Install: cp scripts/sigma_commit_hook.sh .git/hooks/commit-msg && chmod +x .git/hooks/commit-msg
# Or use: scripts/setup_hooks.sh
#
# Enforces:
#   1. Conventional Commits format: type(scope): description
#   2. Signed-off-by line for commits touching kernel/ drivers/ security/ kabi/
#   3. Maximum header length (100 chars)
#   4. No WIP or fixup commits in protected branches

set -e

COMMIT_MSG_FILE="$1"
COMMIT_MSG=$(cat "$COMMIT_MSG_FILE")

# ── Colours ────────────────────────────────────────────────────────────────
RED='\033[0;31m'; YELLOW='\033[0;33m'; GREEN='\033[0;32m'; NC='\033[0m'

error() { echo -e "${RED}[commit-msg] ERROR: $*${NC}" >&2; }
warn()  { echo -e "${YELLOW}[commit-msg] WARN:  $*${NC}" >&2; }
ok()    { echo -e "${GREEN}[commit-msg] OK:    $*${NC}" >&2; }

FAIL=0

# ── 1. Conventional Commits format ────────────────────────────────────────
HEADER=$(echo "$COMMIT_MSG" | head -1)
TYPES="feat|fix|docs|style|refactor|perf|test|chore|revert|impl|driver|security|kernel|arch|ci|pkg|ai|ux|sdk|boot"

if ! echo "$HEADER" | grep -qE "^($TYPES)(\([a-zA-Z0-9_\-]+\))?: .{1,}"; then
    error "Commit header doesn't match Conventional Commits format."
    echo  "  Expected: type(scope): description" >&2
    echo  "  Valid types: $TYPES" >&2
    echo  "  Your header: $HEADER" >&2
    echo  "  Example: feat(kernel): add MLFQ scheduler priority boost" >&2
    FAIL=1
fi

# ── 2. Header length ───────────────────────────────────────────────────────
HEADER_LEN=${#HEADER}
if [ "$HEADER_LEN" -gt 100 ]; then
    error "Header too long ($HEADER_LEN chars, max 100)."
    echo  "  Shorten: $HEADER" >&2
    FAIL=1
fi

# ── 3. No empty description ────────────────────────────────────────────────
DESC=$(echo "$HEADER" | sed 's/^[^:]*: //')
if [ -z "$DESC" ]; then
    error "Commit description is empty."
    FAIL=1
fi

# ── 4. Signed-off-by for kernel/drivers/security/kabi ─────────────────────
PROTECTED_PATHS="^(kernel|drivers|security|kabi)/"

if git diff --cached --name-only 2>/dev/null | grep -qE "$PROTECTED_PATHS"; then
    if ! echo "$COMMIT_MSG" | grep -q "^Signed-off-by:"; then
        warn "Commits touching kernel/drivers/security/kabi should have Signed-off-by."
        echo  "  Add: Signed-off-by: Your Name <your@email.com>" >&2
        echo  "  Git shortcut: git commit --signoff" >&2
        # Warning only (not FAIL) — some people use DCO bot instead
    fi
fi

# ── 5. Block WIP and fixup commits on main ────────────────────────────────
BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
if [ "$BRANCH" = "main" ] || [ "$BRANCH" = "master" ]; then
    if echo "$HEADER" | grep -qiE "^(WIP|fixup!|squash!|DRAFT)"; then
        error "WIP/fixup/squash commits are not allowed on $BRANCH."
        FAIL=1
    fi
fi

# ── 6. Suggest SPDX check ─────────────────────────────────────────────────
NEW_RS_FILES=$(git diff --cached --name-only --diff-filter=A 2>/dev/null | grep '\.rs$' || true)
NEW_C_FILES=$(git diff --cached --name-only --diff-filter=A 2>/dev/null | grep -E '\.(c|cpp|h)$' || true)
if [ -n "$NEW_RS_FILES" ] || [ -n "$NEW_C_FILES" ]; then
    MISSING=0
    for f in $NEW_RS_FILES $NEW_C_FILES; do
        if [ -f "$f" ] && ! head -3 "$f" | grep -q "SPDX-License-Identifier"; then
            warn "New file missing SPDX header: $f"
            MISSING=$((MISSING+1))
        fi
    done
    [ "$MISSING" -gt 0 ] && warn "Add: // SPDX-License-Identifier: MIT (or GPL-2.0)"
fi

# ── Result ─────────────────────────────────────────────────────────────────
if [ "$FAIL" -eq 1 ]; then
    echo "" >&2
    error "Commit rejected. Fix the issues above and try again."
    echo  "  Examples of valid commit messages:" >&2
    echo  "    feat(kernel): add APIC timer interrupt support" >&2
    echo  "    fix(sigma-pkg): resolve dependency cycle in install --deb" >&2
    echo  "    driver(wifi): implement WPA3-SAE commit phase" >&2
    echo  "    security(pledge): add BPF promise bit" >&2
    exit 1
fi

ok "Commit message is valid."
exit 0
