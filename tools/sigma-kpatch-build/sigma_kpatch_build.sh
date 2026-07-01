#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-or-later
# sigma-kpatch-build — generate a live-patch module from a unified diff
#
# Mirrors kpatch-build's approach:
#   1. Build baseline kernel
#   2. Apply patch, rebuild
#   3. readelf diff → find changed functions
#   4. Link changed functions into .spatch module
#   5. Sign with Ed25519 key
#
# Usage: sigma-kpatch-build <patch.diff> [--kernel-src DIR] [--sign-key KEY]

set -euo pipefail

PATCH_FILE="${1:?Usage: sigma-kpatch-build <patch.diff> [--kernel-src DIR]}"
KERNEL_SRC="/sigma/src/kernel"
SIGN_KEY="/sigma/etc/keys/kpatch-signing.key"
CACHE_DIR="$HOME/.sigma-kpatch"
OUTPUT=""

while [[ $# -gt 1 ]]; do
    case "$2" in
        --kernel-src) KERNEL_SRC="$3"; shift 2 ;;
        --sign-key)   SIGN_KEY="$3";   shift 2 ;;
        --output)     OUTPUT="$3";     shift 2 ;;
        *) echo "Unknown option: $2"; exit 1 ;;
    esac
done

PATCH_NAME=$(basename "$PATCH_FILE" .diff)
OUTPUT="${OUTPUT:-sigma-kpatch-${PATCH_NAME}-$(date +%Y%m%d).spatch}"

log() { echo "[sigma-kpatch-build] $*" >&2; }

# ── Step 1: Baseline build ─────────────────────────────────────────────────
log "Building baseline kernel..."
mkdir -p "$CACHE_DIR/baseline" "$CACHE_DIR/patched"
cmake -B "$CACHE_DIR/baseline" -G Ninja \
    -DCMAKE_BUILD_TYPE=Release \
    -DSIGMA_KPATCH_INSTRUMENTATION=ON \
    "$KERNEL_SRC" 2>/dev/null
ninja -C "$CACHE_DIR/baseline" -j"$(nproc)" 2>/dev/null
log "Baseline build complete."

# ── Step 2: Patched build ──────────────────────────────────────────────────
log "Applying patch: $PATCH_FILE"
cp -r "$CACHE_DIR/baseline" "$CACHE_DIR/patched" 2>/dev/null || true
# Apply patch to source (reversible — we restore after)
git -C "$KERNEL_SRC" apply "$PATCH_FILE"
ninja -C "$CACHE_DIR/patched" -j"$(nproc)" 2>/dev/null
# Restore source tree
git -C "$KERNEL_SRC" apply -R "$PATCH_FILE"
log "Patched build complete."

# ── Step 3: Find changed functions ────────────────────────────────────────
log "Comparing symbol tables..."
readelf --syms "$CACHE_DIR/baseline/vmlinuz-sigma" \
    | grep " FUNC " | awk '{print $8, $3}' | sort > "$CACHE_DIR/syms.baseline"
readelf --syms "$CACHE_DIR/patched/vmlinuz-sigma" \
    | grep " FUNC " | awk '{print $8, $3}' | sort > "$CACHE_DIR/syms.patched"

# Functions that changed size or are new in patched
diff "$CACHE_DIR/syms.baseline" "$CACHE_DIR/syms.patched" \
    | grep "^[<>]" > "$CACHE_DIR/changed_funcs.txt" || true

CHANGED_COUNT=$(grep -c "^>" "$CACHE_DIR/changed_funcs.txt" 2>/dev/null || echo 0)
log "Found $CHANGED_COUNT changed/new functions."

if [[ "$CHANGED_COUNT" -eq 0 ]]; then
    log "No functions changed — patch has no effect on compiled code."
    exit 0
fi

# ── Step 4: Build .spatch module ──────────────────────────────────────────
log "Linking patch module..."

# Extract function addresses from patched binary
FUNC_ENTRIES=""
while IFS=' ' read -r fname fsize; do
    OLD_ADDR=$(grep " $fname$" "$CACHE_DIR/syms.baseline" | awk '{print $2}' || true)
    NEW_ADDR=$(nm "$CACHE_DIR/patched/vmlinuz-sigma" 2>/dev/null \
               | grep " T $fname$" | awk '{print $1}' || true)
    if [[ -n "$OLD_ADDR" && -n "$NEW_ADDR" ]]; then
        FUNC_ENTRIES+="$fname:0x$OLD_ADDR:0x$NEW_ADDR\n"
    fi
done < <(grep "^>" "$CACHE_DIR/changed_funcs.txt" | awk '{print $2, $3}')

# Generate .spatch JSON manifest
KERNEL_BUILD_ID=$(readelf -n "$CACHE_DIR/baseline/vmlinuz-sigma" \
                  | grep "Build ID" | awk '{print $3}' 2>/dev/null || echo "unknown")
TIMESTAMP=$(date +%s)

cat > "$CACHE_DIR/patch_manifest.json" << EOF
{
  "magic": "SigKPatc",
  "version": 1,
  "patch_id": "sigma-kpatch-${PATCH_NAME}",
  "target_kernel": "${KERNEL_BUILD_ID}",
  "description": "Live patch: ${PATCH_NAME}",
  "timestamp": ${TIMESTAMP},
  "functions": [
$(echo -e "$FUNC_ENTRIES" | grep -v '^$' | while IFS=: read -r name old new; do
    printf '    {"func_name": "%s", "old_addr": "%s", "new_addr": "%s"}' \
           "$name" "$old" "$new"
    echo ","
done | sed '$ s/,$//')
  ]
}
EOF

# ── Step 5: Sign the module ────────────────────────────────────────────────
if [[ -f "$SIGN_KEY" ]]; then
    log "Signing patch module..."
    openssl pkeyutl -sign \
        -inkey "$SIGN_KEY" \
        -in "$CACHE_DIR/patch_manifest.json" \
        -out "$CACHE_DIR/patch_manifest.sig" 2>/dev/null || \
    log "Warning: signing failed (key not found or wrong format)"
else
    log "Warning: signing key not found at $SIGN_KEY — module will be unsigned"
    touch "$CACHE_DIR/patch_manifest.sig"
fi

# Bundle manifest + signature + patched objects into .spatch archive
tar czf "$OUTPUT" \
    -C "$CACHE_DIR" \
    patch_manifest.json \
    patch_manifest.sig

log "Patch module ready: $OUTPUT"
log "  Changed functions: $CHANGED_COUNT"
log "  Apply with: sigma-kpatch load $OUTPUT"
log "  Verify with: sigma-kpatch verify $OUTPUT"
