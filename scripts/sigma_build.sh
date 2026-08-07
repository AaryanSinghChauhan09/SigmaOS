#!/usr/bin/env sh
# SigmaOS build orchestrator — replaces all PowerShell build scripts
# (build_sovereign.ps1, industrial_build.ps1, reproducible_build.ps1, etc.)
# Pure POSIX sh — zero runtime dependency beyond a POSIX shell.

set -e

# ── Configuration ─────────────────────────────────────────────────────────
ARCH="${ARCH:-x86_64}"
BUILD_DIR="./build/${ARCH}"
KERNEL_SRC="./kernel"
SIGPKG_SRC="./userland/sigpkg"
TOOLS_SRC="./tools"
LOG_FILE="${BUILD_DIR}/build.log"

# Reproducible: zero out timestamps that leak into binaries
SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-1704067200}"
export SOURCE_DATE_EPOCH

# ── Utility ───────────────────────────────────────────────────────────────
log()  { printf '[sigma-build] %s\n' "$*" | tee -a "${LOG_FILE}"; }
die()  { log "ERROR: $*"; exit 1; }
step() { log "── $* ──"; }

# ── Toolchain Detection ───────────────────────────────────────────────────
detect_toolchain() {
    step "Detecting toolchain"
    CC="${CC:-cc}"
    RUSTC="${RUSTC:-rustc}"
    NIM="${NIM:-nim}"
    ZIG="${ZIG:-zig}"

    command -v "${CC}"    >/dev/null 2>&1 || die "C compiler not found"
    command -v "${RUSTC}" >/dev/null 2>&1 && log "  Rust: $(${RUSTC} --version)"
    command -v "${NIM}"   >/dev/null 2>&1 && log "  Nim: $(${NIM} --version | head -1)"
    command -v "${ZIG}"   >/dev/null 2>&1 && log "  Zig: $(${ZIG} version)"
}

# ── Directory Setup ───────────────────────────────────────────────────────
setup_dirs() {
    step "Creating build directories"
    mkdir -p "${BUILD_DIR}"
}

# ── Build C Tools ─────────────────────────────────────────────────────────
build_c_tools() {
    step "Building C tools"
    "${CC}" -O2 -std=c11 -Wall -Wextra \
        "${TOOLS_SRC}/sigma_nl_cli.c" \
        -o "${BUILD_DIR}/sigma_nl_cli" \
        && log "  sigma_nl_cli: OK"

    "${CC}" -O2 -std=c11 -Wall -Wextra \
        "${TOOLS_SRC}/sigma_mkinitfs.c" \
        -o "${BUILD_DIR}/sigma_mkinitfs" \
        && log "  sigma_mkinitfs: OK"

    "${CC}" -O2 -std=c11 -Wall -Wextra \
        "./kabi/sigma_kabi.c" \
        -o "${BUILD_DIR}/sigma_kabi" \
        && log "  sigma_kabi: OK"
}

# ── Build Nim Tools ───────────────────────────────────────────────────────
build_nim_tools() {
    if ! command -v "${NIM}" >/dev/null 2>&1; then
        log "  Nim not found — skipping Nim targets"
        return
    fi
    step "Building Nim tools"
    "${NIM}" c --gc:none --opt:speed \
        -o:"${BUILD_DIR}/sigpkg" \
        "${SIGPKG_SRC}/sigpkg_cli.nim" \
        && log "  sigpkg CLI: OK"

    "${NIM}" c --gc:none --opt:speed \
        -o:"${BUILD_DIR}/sync_wiki" \
        "./scripts/sync_wiki.nim" \
        && log "  sync_wiki: OK"

    "${NIM}" c --gc:none --opt:speed \
        -o:"${BUILD_DIR}/syscall_profiler" \
        "${TOOLS_SRC}/syscall_profiler.nim" \
        && log "  syscall_profiler: OK"
}

# ── Build Zig Apps ────────────────────────────────────────────────────────
build_zig_apps() {
    if ! command -v "${ZIG}" >/dev/null 2>&1; then
        log "  Zig not found — skipping Zig targets"
        return
    fi
    step "Building Zig apps"
    "${ZIG}" build-exe "./apps/sigmaTerm.zig" \
        -femit-bin="${BUILD_DIR}/sigmaTerm" \
        && log "  sigmaTerm: OK"

    "${ZIG}" build-exe "./apps/sigmaNotes.zig" \
        -femit-bin="${BUILD_DIR}/sigmaNotes" \
        && log "  sigmaNotes: OK"
}

# ── Reproducibility Check ─────────────────────────────────────────────────
check_reproducibility() {
    step "Reproducibility check (SOURCE_DATE_EPOCH=${SOURCE_DATE_EPOCH})"
    log "  Build artifacts in ${BUILD_DIR}"
    log "  Re-run with same SOURCE_DATE_EPOCH to verify bit-identical output"
}

# ── Main ──────────────────────────────────────────────────────────────────
main() {
    log "SigmaOS Build System v0.1"
    log "Target architecture: ${ARCH}"
    log "Build directory: ${BUILD_DIR}"

    setup_dirs
    detect_toolchain
    build_c_tools
    build_nim_tools
    build_zig_apps
    check_reproducibility

    log "Build COMPLETE."
}

main "$@"
