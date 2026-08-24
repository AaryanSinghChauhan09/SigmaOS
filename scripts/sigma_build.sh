#!/usr/bin/env sh
# SigmaOS build orchestrator — replaces all PowerShell build scripts
# Pure POSIX sh — zero runtime dependency beyond a POSIX shell.
# Inspired by FreeBSD/NetBSD/OpenBSD build system primitives and Linux distro packaging tools.

set -e

# ── Configuration & Defaults ──────────────────────────────────────────────
ARCH="${ARCH:-x86_64}"
BUILD_DIR="./build/${ARCH}"
KERNEL_SRC="./kernel"
SIGPKG_SRC="./userland/sigpkg"
TOOLS_SRC="./tools"
LOG_FILE="${BUILD_DIR}/build.log"
VERBOSE=0
DRY_RUN=0

# Reproducible: zero out timestamps that leak into binaries
SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-1704067200}"
export SOURCE_DATE_EPOCH

# ── OS & Platform Detection (Linux/BSD/Darwin) ─────────────────────────────
HOST_OS="$(uname -s 2>/dev/null || echo "Unknown")"
HOST_ARCH="$(uname -m 2>/dev/null || echo "Unknown")"

# ANSI Color Support Detection (inspired by BSD rc.subr / Linux init-functions)
if [ -t 1 ] && [ "${TERM:-dumb}" != "dumb" ]; then
    COLOR_BLUE='\033[1;34m'
    COLOR_GREEN='\033[1;32m'
    COLOR_RED='\033[1;31m'
    COLOR_YELLOW='\033[1;33m'
    COLOR_RESET='\033[0m'
else
    COLOR_BLUE=''
    COLOR_GREEN=''
    COLOR_RED=''
    COLOR_YELLOW=''
    COLOR_RESET=''
fi

# ── Utility & Logging ──────────────────────────────────────────────────────
setup_dirs() {
    mkdir -p "${BUILD_DIR}"
}

log() {
    setup_dirs
    printf "${COLOR_BLUE}[sigma-build]${COLOR_RESET} %s\n" "$*" | tee -a "${LOG_FILE}"
}

warn() {
    setup_dirs
    printf "${COLOR_YELLOW}[sigma-build WARN]${COLOR_RESET} %s\n" "$*" | tee -a "${LOG_FILE}"
}

die() {
    setup_dirs
    printf "${COLOR_RED}[sigma-build ERROR]${COLOR_RESET} %s\n" "$*" | tee -a "${LOG_FILE}" >&2
    exit 1
}

step() {
    log "── $* ──"
}

# Cleanup Signal Trap
cleanup() {
    status=$?
    if [ $status -ne 0 ]; then
        printf "${COLOR_RED}[sigma-build] Build process terminated unexpectedly with exit code %d${COLOR_RESET}\n" "$status" >&2
    fi
}
trap cleanup EXIT INT TERM

# ── Toolchain Detection ───────────────────────────────────────────────────
detect_toolchain() {
    step "Detecting toolchain on ${HOST_OS} (${HOST_ARCH})"
    CC="${CC:-cc}"
    RUSTC="${RUSTC:-rustc}"
    NIM="${NIM:-nim}"
    ZIG="${ZIG:-zig}"

    if command -v "${CC}" >/dev/null 2>&1; then
        log "  C Compiler (${CC}): OK"
    else
        die "C compiler (${CC}) not found"
    fi

    if command -v "${RUSTC}" >/dev/null 2>&1; then
        log "  Rust (${RUSTC}): $(${RUSTC} --version)"
    else
        warn "  Rust compiler not found — skipping Rust core targets"
    fi

    if command -v "${NIM}" >/dev/null 2>&1; then
        log "  Nim (${NIM}): $(${NIM} --version | head -1)"
    else
        log "  Nim not found — skipping Nim targets"
    fi

    if command -v "${ZIG}" >/dev/null 2>&1; then
        log "  Zig (${ZIG}): $(${ZIG} version)"
    else
        log "  Zig not found — skipping Zig targets"
    fi
}

# ── Build C Tools ─────────────────────────────────────────────────────────
build_c_tools() {
    step "Building C tools"

    if [ -f "${TOOLS_SRC}/sigma_nl_cli.c" ]; then
        if [ "$DRY_RUN" -eq 1 ]; then
            log "  [DRY-RUN] ${CC} -O2 -std=c11 -Wall -Wextra ${TOOLS_SRC}/sigma_nl_cli.c -o ${BUILD_DIR}/sigma_nl_cli"
        else
            "${CC}" -O2 -std=c11 -Wall -Wextra "${TOOLS_SRC}/sigma_nl_cli.c" -o "${BUILD_DIR}/sigma_nl_cli" && log "  sigma_nl_cli: OK"
        fi
    fi

    if [ -f "${TOOLS_SRC}/sigma_mkinitfs.c" ]; then
        if [ "$DRY_RUN" -eq 1 ]; then
            log "  [DRY-RUN] ${CC} -O2 -std=c11 -Wall -Wextra ${TOOLS_SRC}/sigma_mkinitfs.c -o ${BUILD_DIR}/sigma_mkinitfs"
        else
            "${CC}" -O2 -std=c11 -Wall -Wextra "${TOOLS_SRC}/sigma_mkinitfs.c" -o "${BUILD_DIR}/sigma_mkinitfs" && log "  sigma_mkinitfs: OK"
        fi
    fi

    if [ -f "./kabi/sigma_kabi.c" ]; then
        if [ "$DRY_RUN" -eq 1 ]; then
            log "  [DRY-RUN] ${CC} -O2 -std=c11 -Wall -Wextra ./kabi/sigma_kabi.c -o ${BUILD_DIR}/sigma_kabi"
        else
            "${CC}" -O2 -std=c11 -Wall -Wextra "./kabi/sigma_kabi.c" -o "${BUILD_DIR}/sigma_kabi" && log "  sigma_kabi: OK"
        fi
    fi
}

# ── Build Nim Tools ───────────────────────────────────────────────────────
build_nim_tools() {
    if ! command -v "${NIM}" >/dev/null 2>&1; then
        return
    fi
    step "Building Nim tools"

    if [ -f "${SIGPKG_SRC}/sigpkg_cli.nim" ]; then
        if [ "$DRY_RUN" -eq 1 ]; then
            log "  [DRY-RUN] ${NIM} c --gc:none --opt:speed -o:${BUILD_DIR}/sigpkg ${SIGPKG_SRC}/sigpkg_cli.nim"
        else
            "${NIM}" c --gc:none --opt:speed -o:"${BUILD_DIR}/sigpkg" "${SIGPKG_SRC}/sigpkg_cli.nim" && log "  sigpkg CLI: OK"
        fi
    fi

    if [ -f "./scripts/sync_wiki.nim" ]; then
        if [ "$DRY_RUN" -eq 1 ]; then
            log "  [DRY-RUN] ${NIM} c --gc:none --opt:speed -o:${BUILD_DIR}/sync_wiki ./scripts/sync_wiki.nim"
        else
            "${NIM}" c --gc:none --opt:speed -o:"${BUILD_DIR}/sync_wiki" "./scripts/sync_wiki.nim" && log "  sync_wiki: OK"
        fi
    fi

    if [ -f "${TOOLS_SRC}/syscall_profiler.nim" ]; then
        if [ "$DRY_RUN" -eq 1 ]; then
            log "  [DRY-RUN] ${NIM} c --gc:none --opt:speed -o:${BUILD_DIR}/syscall_profiler ${TOOLS_SRC}/syscall_profiler.nim"
        else
            "${NIM}" c --gc:none --opt:speed -o:"${BUILD_DIR}/syscall_profiler" "${TOOLS_SRC}/syscall_profiler.nim" && log "  syscall_profiler: OK"
        fi
    fi
}

# ── Build Zig Apps ────────────────────────────────────────────────────────
build_zig_apps() {
    if ! command -v "${ZIG}" >/dev/null 2>&1; then
        return
    fi
    step "Building Zig apps"

    if [ -f "./apps/sigmaTerm.zig" ]; then
        if [ "$DRY_RUN" -eq 1 ]; then
            log "  [DRY-RUN] ${ZIG} build-exe ./apps/sigmaTerm.zig -femit-bin=${BUILD_DIR}/sigmaTerm"
        else
            "${ZIG}" build-exe "./apps/sigmaTerm.zig" -femit-bin="${BUILD_DIR}/sigmaTerm" && log "  sigmaTerm: OK"
        fi
    fi

    if [ -f "./apps/sigmaNotes.zig" ]; then
        if [ "$DRY_RUN" -eq 1 ]; then
            log "  [DRY-RUN] ${ZIG} build-exe ./apps/sigmaNotes.zig -femit-bin=${BUILD_DIR}/sigmaNotes"
        else
            "${ZIG}" build-exe "./apps/sigmaNotes.zig" -femit-bin="${BUILD_DIR}/sigmaNotes" && log "  sigmaNotes: OK"
        fi
    fi
}

# ── Reproducibility Check ─────────────────────────────────────────────────
check_reproducibility() {
    step "Reproducibility check (SOURCE_DATE_EPOCH=${SOURCE_DATE_EPOCH})"
    log "  Build artifacts in ${BUILD_DIR}"
    log "  Re-run with same SOURCE_DATE_EPOCH to verify bit-identical output"
}

# ── Usage & Help ──────────────────────────────────────────────────────────
usage() {
    cat <<EOF
SigmaOS Build Orchestrator (POSIX Shell Interface)

Usage:
  $(basename "$0") [options]

Options:
  -a <arch>   Target architecture (x86_64, aarch64, riscv64). Default: ${ARCH}
  -d          Dry-run mode (print commands without executing)
  -v          Verbose output mode
  -h          Display this help message and exit
EOF
}

# ── Main ──────────────────────────────────────────────────────────────────
main() {
    while getopts "a:dvh" opt; do
        case "$opt" in
            a) ARCH="$OPTARG"; BUILD_DIR="./build/${ARCH}"; LOG_FILE="${BUILD_DIR}/build.log" ;;
            d) DRY_RUN=1 ;;
            v) VERBOSE=1 ;;
            h) usage; exit 0 ;;
            *) usage; exit 1 ;;
        esac
    done

    setup_dirs
    log "SigmaOS Build System v0.2 (${HOST_OS}/${HOST_ARCH})"
    log "Target architecture: ${ARCH}"
    log "Build directory: ${BUILD_DIR}"

    detect_toolchain
    build_c_tools
    build_nim_tools
    build_zig_apps
    check_reproducibility

    log "${COLOR_GREEN}Build COMPLETE.${COLOR_RESET}"
}

main "$@"
