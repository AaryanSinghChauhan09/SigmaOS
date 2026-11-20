#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# build_archiso.sh — Build the SigmaOS Arch-based live ISO
#
# Works on any Linux host with Docker OR on a native Arch Linux system.
# Without Docker or mkarchiso, prints a validation summary and instructions.
#
# Usage:
#   scripts/build_archiso.sh [--docker] [--native] [--validate-only] [-h]
#
# Output: out/sigmaos-desktop-<date>-x86_64.iso
# Logs:   out/build.log

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
PROFILE_DIR="$ROOT_DIR/iso/archiso"
OUT_DIR="$ROOT_DIR/out"
WORK_DIR="$ROOT_DIR/work"
DATE=$(date +%Y%m%d)
ISO_NAME="sigmaos-desktop-${DATE}-x86_64.iso"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; CYAN='\033[0;36m'
BOLD='\033[1m'; RESET='\033[0m'

info()    { echo -e "${CYAN}[build-iso]${RESET} $*"; }
success() { echo -e "${GREEN}[build-iso]${RESET} $*"; }
warn()    { echo -e "${YELLOW}[build-iso]${RESET} $*"; }
err()     { echo -e "${RED}[build-iso]${RESET} $*" >&2; }

usage() {
  cat <<'EOF'
build_archiso.sh — SigmaOS live ISO build tool

USAGE
  scripts/build_archiso.sh [OPTIONS]

OPTIONS
  --docker          Build using Docker + archlinux image (works on any Linux host)
  --native          Build using mkarchiso directly (requires Arch Linux host)
  --validate-only   Validate the ISO profile only; do not build
  -h, --help        Show this help

EXAMPLES
  # Validate profile on any host:
  scripts/build_archiso.sh --validate-only

  # Build via Docker (recommended for Ubuntu/Debian hosts):
  scripts/build_archiso.sh --docker

  # Build natively on an Arch Linux host:
  sudo scripts/build_archiso.sh --native
EOF
}

# ── Mode flags ───────────────────────────────────────────────────────────────
MODE="auto"
for arg in "$@"; do
  case "$arg" in
    --docker)        MODE="docker" ;;
    --native)        MODE="native" ;;
    --validate-only) MODE="validate" ;;
    -h|--help)       usage; exit 0 ;;
    *) err "Unknown option: $arg"; usage; exit 1 ;;
  esac
done

# ── Validate ISO profile ──────────────────────────────────────────────────────
validate_profile() {
  local ok=1
  info "Validating ISO profile at: $PROFILE_DIR"
  echo ""

  local required_files=(
    "$PROFILE_DIR/profiledef.sh"
    "$PROFILE_DIR/packages.x86_64"
    "$PROFILE_DIR/airootfs/usr/local/bin/sigmaos-install"
    "$PROFILE_DIR/airootfs/etc/hostname"
    "$PROFILE_DIR/airootfs/etc/locale.conf"
  )

  for f in "${required_files[@]}"; do
    if [ -f "$f" ]; then
      echo -e "  ${GREEN}✔${RESET}  $(basename "$f")"
    else
      echo -e "  ${RED}✘${RESET}  MISSING: $f"
      ok=0
    fi
  done

  # Validate profiledef.sh fields
  if [ -f "$PROFILE_DIR/profiledef.sh" ]; then
    if grep -q 'iso_name="sigmaos-desktop"' "$PROFILE_DIR/profiledef.sh"; then
      echo -e "  ${GREEN}✔${RESET}  iso_name=sigmaos-desktop"
    else
      echo -e "  ${YELLOW}⚠${RESET}  iso_name may not be 'sigmaos-desktop'"
    fi
    if grep -q 'install_dir=' "$PROFILE_DIR/profiledef.sh"; then
      echo -e "  ${GREEN}✔${RESET}  install_dir defined"
    fi
  fi

  # Validate package list
  if [ -f "$PROFILE_DIR/packages.x86_64" ]; then
    local pkg_count
    pkg_count=$(grep -v '^#' "$PROFILE_DIR/packages.x86_64" | grep -v '^$' | wc -l)
    echo -e "  ${GREEN}✔${RESET}  packages.x86_64 — $pkg_count packages listed"
  fi

  echo ""
  if [ "$ok" = "1" ]; then
    success "[OK] Profile validation successful."
    return 0
  else
    err "[FAIL] Profile has missing required files."
    return 1
  fi
}

# ── Docker build ──────────────────────────────────────────────────────────────
build_docker() {
  info "Building SigmaOS ISO via Docker (archlinux image)..."

  if ! command -v docker &>/dev/null; then
    err "Docker is not installed. Install it with:"
    err "  sudo apt-get install -y docker.io && sudo systemctl enable --now docker"
    exit 1
  fi

  if ! docker info &>/dev/null; then
    err "Docker daemon is not running or current user lacks access."
    err "  sudo systemctl start docker"
    err "  sudo usermod -aG docker \$USER  # then log out and in"
    exit 1
  fi

  mkdir -p "$OUT_DIR" "$WORK_DIR" "$WORK_DIR/pacman-cache"

  # archlinux:latest ships openssl 4.x which breaks libssl.so=3 consumers and
  # the erofs-utils dep chain. Pin openssl 3.x + archiso-91 (pre-erofs) from
  # the Arch archive so the build is reproducible on a fresh clone.
  ensure_pkg() {
    local file="$1" url="$2"
    [ -s "$WORK_DIR/$file" ] && return 0
    info "Downloading $file ..."
    curl -fL --retry 3 -o "$WORK_DIR/$file" "$url" \
      || { err "Failed to download $file"; exit 1; }
  }
  ARCH_SNAP="https://archive.archlinux.org/repos/2025/11/20/extra/os/x86_64"
  ensure_pkg "openssl-3.6.5-1-x86_64.pkg.tar.zst"  "$ARCH_SNAP/openssl-3.6.5-1-x86_64.pkg.tar.zst"
  ensure_pkg "archiso-91-1-any.pkg.tar.zst"       "$ARCH_SNAP/archiso-91-1-any.pkg.tar.zst"

  local abs_profile
  abs_profile="$(realpath "$PROFILE_DIR")"
  local abs_out
  abs_out="$(realpath "$OUT_DIR")"
  local abs_work
  abs_work="$(realpath "$WORK_DIR")"
  local abs_cache
  abs_cache="$(realpath "$WORK_DIR/pacman-cache")"

  info "Profile:   $abs_profile"
  info "Output:    $abs_out/$ISO_NAME"
  info "Work dir:  $abs_work"
  echo ""

  docker run --rm \
    --privileged \
    -v "$abs_profile:/profile:ro" \
    -v "$abs_out:/out" \
    -v "$abs_work:/work" \
    -v "$abs_cache:/var/cache/pacman/pkg" \
    archlinux:latest \
    bash -c "
      set -euo pipefail
      echo '[docker] Installing archiso...'
      pacman -Sy --noconfirm
      # Pin openssl 3.x: archlinux:latest ships openssl 4.x which breaks
      # libssl.so=3-64 consumers (curl, pacman deps) needed by archiso.
      if [ -f /work/openssl-3.6.5-1-x86_64.pkg.tar.zst ]; then
        pacman -U --noconfirm /work/openssl-3.6.5-1-x86_64.pkg.tar.zst
        grep -q '^IgnorePkg.*openssl' /etc/pacman.conf || echo 'IgnorePkg = openssl' >> /etc/pacman.conf
      fi
      # Current archiso requires erofs-utils -> openssl 4 (conflicts with pin).
      # Use archived archiso-91 and skip erofs-utils: our profile is squashfs,
      # so mkarchiso never invokes erofs tools.
      if [ -f /work/archiso-91-1-any.pkg.tar.zst ]; then
        pacman -U --noconfirm --assume-installed erofs-utils /work/archiso-91-1-any.pkg.tar.zst
      else
        pacman -Sy --noconfirm archiso
      fi
      # pacman.conf references /etc/pacman.d/*. Include files that live in
      # airootfs — copy them into the build container's pacman.d so mkarchiso
      # can resolve the [sigmaos] repo during package installation.
      mkdir -p /etc/pacman.d
      cp -f /profile/airootfs/etc/pacman.d/* /etc/pacman.d/ 2>/dev/null || true
      # mkarchiso's _run_once markers persist in the mounted work dir across
      # runs; a stale marker would skip pacman.conf regeneration and reuse an
      # empty/broken iso.pacman.conf. Always start from a clean work dir.
      rm -rf /work/iso-work
      echo '[docker] Running mkarchiso...'
      mkarchiso -v -w /work/iso-work -o /out /profile
      echo '[docker] Build complete.'
    " 2>&1 | tee "$OUT_DIR/build.log"

  if ls "$OUT_DIR"/*.iso &>/dev/null; then
    success "ISO built successfully: $OUT_DIR/$(ls "$OUT_DIR"/*.iso | head -1 | xargs basename)"
  else
    err "Build completed but no ISO found in $OUT_DIR"
    exit 1
  fi
}

# ── Native build ──────────────────────────────────────────────────────────────
build_native() {
  info "Building SigmaOS ISO natively with mkarchiso..."

  if [[ "$(id -u)" != "0" ]]; then
    err "Native mkarchiso build requires root. Run: sudo $0 --native"
    exit 1
  fi

  if ! command -v mkarchiso &>/dev/null; then
    err "mkarchiso not found. This requires Arch Linux."
    err "  Install: pacman -S archiso"
    exit 1
  fi

  mkdir -p "$OUT_DIR" "$WORK_DIR"
  info "Profile:  $PROFILE_DIR"
  info "Output:   $OUT_DIR"

  mkarchiso -v -w "$WORK_DIR" -o "$OUT_DIR" "$PROFILE_DIR" 2>&1 | tee "$OUT_DIR/build.log"

  if ls "$OUT_DIR"/*.iso &>/dev/null; then
    success "ISO built: $(ls "$OUT_DIR"/*.iso)"
  else
    err "Build failed — no ISO in $OUT_DIR"
    exit 1
  fi
}

# ── Auto-detect mode ──────────────────────────────────────────────────────────
auto_detect() {
  if command -v mkarchiso &>/dev/null; then
    info "Detected mkarchiso. Using native build mode."
    build_native
  elif command -v docker &>/dev/null && docker info &>/dev/null 2>&1; then
    info "Detected Docker. Using Docker build mode."
    build_docker
  else
    warn "Neither mkarchiso nor Docker available on this system."
    warn "Running profile validation only."
    echo ""
    validate_profile
    echo ""
    warn "To build the ISO, choose one of:"
    warn "  Option A — Docker (any Linux host):"
    warn "    sudo apt install -y docker.io && sudo systemctl start docker"
    warn "    scripts/build_archiso.sh --docker"
    warn ""
    warn "  Option B — Arch Linux host:"
    warn "    pacman -S archiso"
    warn "    sudo scripts/build_archiso.sh --native"
    warn ""
    warn "  Option C — QEMU test without ISO:"
    warn "    scripts/qemu_smoke_test.sh"
  fi
}

# ── Dispatch ──────────────────────────────────────────────────────────────────
echo -e "${BOLD}${CYAN}╔════════════════════════════════════════════╗${RESET}"
echo -e "${BOLD}${CYAN}║   SigmaOS Arch-based Live ISO Builder      ║${RESET}"
echo -e "${BOLD}${CYAN}╚════════════════════════════════════════════╝${RESET}"
echo ""

case "$MODE" in
  validate) validate_profile ;;
  docker)   validate_profile && build_docker ;;
  native)   validate_profile && build_native ;;
  auto)     auto_detect ;;
esac
