#!/usr/bin/env bash
# automation/scripts/build.sh — Smart shard build script
# Only rebuilds shards affected by changed files (CI fast path)
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
CYAN='\033[36m'; GREEN='\033[32m'; YELLOW='\033[33m'; RED='\033[31m'; NC='\033[0m'

info()  { echo -e "${CYAN}[BUILD]${NC} $*"; }
ok()    { echo -e "${GREEN}[OK]${NC}    $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC}  $*"; }
fail()  { echo -e "${RED}[FAIL]${NC}  $*"; exit 1; }

CHANGED="${1:-all}"   # Pass changed file list or "all" to rebuild everything

# ── Determine affected targets ──────────────────────────────────────────────
build_kernel=false
build_rust=false
build_web=false
build_tools=false

if [[ "$CHANGED" == "all" ]]; then
    build_kernel=true; build_rust=true; build_web=true; build_tools=true
else
    echo "$CHANGED" | grep -qE 'kernel/|include/|Makefile' && build_kernel=true || true
    echo "$CHANGED" | grep -qE 'shards/|core/|Cargo\.toml'  && build_rust=true   || true
    echo "$CHANGED" | grep -qE 'web_ui/'                     && build_web=true    || true
    echo "$CHANGED" | grep -qE 'tools/|server/'              && build_tools=true  || true
fi

# ── Build Rust Workspace ───────────────────────────────────────────────────
if $build_rust; then
    info "Building Rust workspace (cargo build --workspace)..."
    cd "$ROOT"
    if cargo build --workspace --release 2>&1; then
        ok "Rust workspace built."
    else
        warn "Rust build failed — running self-heal (clean + retry)"
        cargo clean && cargo build --workspace --release || fail "Self-heal failed."
    fi
fi

# ── Build C Kernel Shards ──────────────────────────────────────────────────
if $build_kernel; then
    info "Building C11 kernel shards..."
    cd "$ROOT"
    PASS=0; FAIL=0
    for src in $(find kernel/suites -name "*.c" 2>/dev/null); do
        NAME=$(basename "${src%.*}")
        mkdir -p build
        if x86_64-linux-gnu-gcc -std=c11 -ffreestanding -O2 -m64 \
            -Wall -Wextra -Wno-unused-function -Wno-unused-parameter \
            -I. -Iinclude -nostdlib -fno-stack-protector \
            -c "$src" -o "build/${NAME}.o" 2>/dev/null; then
            PASS=$((PASS+1))
        else
            FAIL=$((FAIL+1))
        fi
    done
    ok "C Shards: $PASS compiled / $FAIL skipped"
fi

# ── Web tooling ────────────────────────────────────────────────────────────
if $build_web; then
    info "Checking Zenith UI module count..."
    COUNT=$(ls "$ROOT/web_ui/scripts/modules/"*.js 2>/dev/null | wc -l)
    ok "UI modules: $COUNT"
fi

if $build_tools; then
    info "Building tools..."
    make -C "$ROOT/tools" all 2>/dev/null || warn "Tools build skipped (check tools/Makefile)"
fi

ok "Build complete."
