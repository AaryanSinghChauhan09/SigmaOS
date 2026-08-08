#!/bin/bash
# SPDX-License-Identifier: MIT
# SigmaOS Memory and Binary Size Regression Auditor
# Audits codebase for potential performance bottlenecks, static arrays, and executable sizes.

set -eo pipefail

echo "=== SigmaOS Static Memory and Binary Size Regression Auditor ==="

# 1. Audit binary size bounds
log_size_check() {
    local label="$1"
    local path="$2"
    local max_bytes="$3"
    if [ -f "$path" ]; then
        local bytes
        bytes=$(stat -c%s "$path" 2>/dev/null || stat -f%z "$path" 2>/dev/null || echo 0)
        echo "[INFO] $label size: $bytes bytes (Limit: $max_bytes bytes)"
        if [ "$bytes" -gt "$max_bytes" ]; then
            echo "[WARN] $label size exceeds the recommended footprint budget of $max_bytes bytes!"
        else
            echo "[PASS] $label is within budget."
        fi
    else
        echo "[INFO] $label ($path) not compiled yet. Skipping size check."
    fi
}

log_size_check "Kernel binary (target/release/sigma_kernel)" "target/release/sigma_kernel" 10485760 # 10MB limit
log_size_check "Kernel debug binary (target/debug/sigma_kernel)" "target/debug/sigma_kernel" 52428800 # 50MB limit
log_size_check "Generated ISO Image" "build/sigmaos.iso" 104857600 # 100MB limit

# 2. Audit potential stack overflows or huge static allocations
echo "[INFO] Scanning for potential static memory bottlenecks..."
HUGE_ALLOCS=0
# Search for huge arrays like [u8; 102400] or static arrays over 10KB
if grep -rnE "\[(u8|i8|u16|i16|u32|i32|u64|i64|f32|f64); [1-9][0-9]{4,}\]" src/ 2>/dev/null; then
    echo "[WARN] Found static allocations that might cause stack overflows or high memory usage."
    HUGE_ALLOCS=$((HUGE_ALLOCS + 1))
else
    echo "[PASS] No excessively large static array allocations (>10KB) found."
fi

# 3. Check for memory leak risk patterns (e.g., Box::leak)
echo "[INFO] Auditing codebase for intentional memory leaks (Box::leak)..."
LEAK_PATTERNS=$(grep -rn "Box::leak" src/ 2>/dev/null | wc -l || echo 0)
if [ "$LEAK_PATTERNS" -gt 0 ]; then
    echo "[WARN] Found $LEAK_PATTERNS instances of Box::leak. Ensure these are intended global singletons."
else
    echo "[PASS] No memory leak risk patterns detected."
fi

echo "[PASS] No blocking memory or size regressions detected."
exit 0
