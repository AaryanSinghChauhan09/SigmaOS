#!/usr/bin/env bash
# =============================================================================
# SigmaOS Quality Gate Script
# Run before any PR merge to main. Checks stubs, SPDX, credentials, problems.
# Usage: ./scripts/sigma_quality_check.sh [--strict]
# =============================================================================
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
STRICT=0
[[ "${1:-}" == "--strict" ]] && STRICT=1

FAIL=0
WARN=0
log()  { echo "[sigma-quality] $*"; }
fail() { echo "[sigma-quality] FAIL: $*"; FAIL=1; }
warn() { echo "[sigma-quality] WARN: $*"; WARN=1; }

log "========================================"
log "SigmaOS Quality Gate — $(date '+%Y-%m-%d %H:%M:%S')"
log "Root: $ROOT"
log "========================================"

# ── 1. Stub / TODO count ──────────────────────────────────────────────────────
log "[1/8] Counting open stubs..."
STUBS=0
for pattern in "return 0; // stub" "// TODO" "// FIXME" "return -1; // NYI" \
               "SIGMA_NULL; /\* TODO" "/* TODO:" "// NYI:"; do
  COUNT=$(grep -r --include="*.cpp" --include="*.c" --include="*.h" \
    "$pattern" "$ROOT/kernel" "$ROOT/net" "$ROOT/drivers" "$ROOT/userland" \
    "$ROOT/crypto" 2>/dev/null | wc -l || true)
  STUBS=$((STUBS + COUNT))
done
log "  Open stubs/TODOs: $STUBS"
if [ "$STUBS" -gt 500 ]; then
  warn "$STUBS stubs found (threshold: 500)"
  [[ "$STRICT" -eq 1 ]] && fail "Strict mode: too many stubs"
fi

# ── 2. SPDX licence headers ───────────────────────────────────────────────────
log "[2/8] Checking SPDX licence headers..."
MISSING_SPDX=0
while IFS= read -r -d '' f; do
  grep -q "SPDX-License-Identifier" "$f" 2>/dev/null || MISSING_SPDX=$((MISSING_SPDX+1))
done < <(find "$ROOT/kernel" "$ROOT/userland" "$ROOT/crypto" "$ROOT/net" \
  -name "*.cpp" -o -name "*.h" -o -name "*.c" 2>/dev/null | head -200 | tr '\n' '\0')
log "  Files missing SPDX: $MISSING_SPDX"
[ "$MISSING_SPDX" -eq 0 ] || warn "$MISSING_SPDX files missing SPDX-License-Identifier"

# ── 3. Hardcoded credentials ──────────────────────────────────────────────────
log "[3/8] Scanning for hardcoded credentials..."
CRED_HITS=$(grep -rn \
  -E '(password|secret|api_key|private_key)\s*=\s*"[^"]{4,}"' \
  "$ROOT/kernel" "$ROOT/userland" "$ROOT/crypto" \
  --include="*.cpp" --include="*.h" --include="*.go" 2>/dev/null \
  | grep -v "test\|example\|stub\|mock\|TODO" | wc -l || true)
log "  Credential pattern hits: $CRED_HITS"
[ "$CRED_HITS" -eq 0 ] || fail "Hardcoded credentials found ($CRED_HITS hits)"

# ── 4. CURRENT_PROBLEMS critical item count ───────────────────────────────────
log "[4/8] Checking open critical problems..."
CRITICAL=0
if [ -f "$ROOT/CURRENT_PROBLEMS_MANIFEST.md" ]; then
  CRITICAL=$(grep -c "🔴" "$ROOT/CURRENT_PROBLEMS_MANIFEST.md" 2>/dev/null || echo 0)
fi
log "  Open 🔴 critical problems: $CRITICAL"
if [ "$CRITICAL" -gt 10 ]; then
  warn "$CRITICAL critical problems open (threshold: 10)"
  [[ "$STRICT" -eq 1 ]] && fail "Strict: too many critical problems open"
fi

# ── 5. Branch parity check ────────────────────────────────────────────────────
log "[5/8] Running branch parity check..."
"$ROOT/scripts/ci_branch_check.sh" 2>/dev/null || warn "Branch parity check failed"

# ── 6. Wiki sync status ───────────────────────────────────────────────────────
log "[6/8] Checking wiki_repo sync..."
WIKI_DIR="$ROOT/wiki_repo"
if [ -d "$WIKI_DIR/.git" ]; then
  BEHIND=$(git -C "$WIKI_DIR" rev-list HEAD..origin/main --count 2>/dev/null || echo 0)
  UNCOMMITTED=$(git -C "$WIKI_DIR" status --porcelain 2>/dev/null | wc -l)
  log "  Wiki commits behind origin: $BEHIND"
  log "  Wiki uncommitted changes: $UNCOMMITTED"
  [ "$BEHIND" -eq 0 ] || warn "wiki_repo is $BEHIND commits behind origin/main"
  [ "$UNCOMMITTED" -eq 0 ] || warn "wiki_repo has $UNCOMMITTED uncommitted changes"
fi

# ── 7. Recovery readiness ─────────────────────────────────────────────────────
log "[7/8] Recovery readiness scan..."
"$ROOT/scripts/sigma_automation.sh" recovery-check 2>/dev/null || \
  warn "Recovery readiness check failed"

# ── 8. Documentation freshness ───────────────────────────────────────────────
log "[8/8] Checking doc freshness..."
if [ -f "$ROOT/CURRENT_PROBLEMS_MANIFEST.md" ]; then
  LAST_MOD=$(git -C "$ROOT" log -1 --format="%ar" \
    -- CURRENT_PROBLEMS_MANIFEST.md 2>/dev/null || echo "unknown")
  log "  CURRENT_PROBLEMS_MANIFEST.md last updated: $LAST_MOD"
fi
if [ -f "$ROOT/FEATURE_MATRIX.md" ]; then
  LAST_MOD=$(git -C "$ROOT" log -1 --format="%ar" \
    -- FEATURE_MATRIX.md 2>/dev/null || echo "unknown")
  log "  FEATURE_MATRIX.md last updated: $LAST_MOD"
fi

# ── Summary ───────────────────────────────────────────────────────────────────
log "========================================"
if [ "$FAIL" -eq 1 ]; then
  log "RESULT: ❌ QUALITY GATE FAILED"
  log "Fix the failures above before merging to main."
  exit 1
elif [ "$WARN" -eq 1 ]; then
  log "RESULT: ⚠️  QUALITY GATE PASSED WITH WARNINGS"
  log "Address warnings before release/* promotion."
  exit 0
else
  log "RESULT: ✅ QUALITY GATE PASSED"
  exit 0
fi
