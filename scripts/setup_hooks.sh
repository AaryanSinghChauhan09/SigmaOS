#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# SigmaOS — Developer environment setup script
# Sets up git hooks, checks toolchain versions, and configures the repo.
#
# Usage: ./scripts/setup_hooks.sh
# Re-run whenever you update the toolchain or pull major changes.

set -e
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

RED='\033[0;31m'; YELLOW='\033[0;33m'; GREEN='\033[0;32m'; CYAN='\033[0;36m'; NC='\033[0m'

ok()   { echo -e "${GREEN}  ✓${NC} $*"; }
warn() { echo -e "${YELLOW}  ⚠${NC} $*"; }
fail() { echo -e "${RED}  ✗${NC} $*"; FAILED=$((FAILED+1)); }
info() { echo -e "${CYAN}  →${NC} $*"; }

FAILED=0

echo ""
echo "  SigmaOS Developer Environment Setup"
echo "  $(printf '─%.0s' {1..50})"
echo ""

# ── 1. Git hooks ──────────────────────────────────────────────────────────
info "Installing git hooks..."
mkdir -p .git/hooks

cp scripts/sigma_commit_hook.sh .git/hooks/commit-msg
chmod +x .git/hooks/commit-msg
ok "commit-msg hook installed"

# pre-commit: run rustfmt check + clippy on staged Rust files
cat > .git/hooks/pre-commit << 'PRECOMMIT_EOF'
#!/usr/bin/env bash
# SigmaOS pre-commit: fast lint on staged files only
set +e
STAGED_RS=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$')
if [ -n "$STAGED_RS" ]; then
    # Run rustfmt in check mode on staged .rs files
    for f in $STAGED_RS; do
        [ -f "$f" ] && rustfmt +nightly --check --edition 2021 "$f" 2>/dev/null && \
            echo "  fmt ok: $f" || echo "  fmt warn: $f (run rustfmt to fix)"
    done
fi
exit 0
PRECOMMIT_EOF
chmod +x .git/hooks/pre-commit
ok "pre-commit hook installed"

# prepare-commit-msg: auto-fill Signed-off-by
cat > .git/hooks/prepare-commit-msg << 'SOB_EOF'
#!/usr/bin/env bash
COMMIT_MSG_FILE=$1
COMMIT_SOURCE=$2
# Only add SOB for normal commits (not merges/squashes)
if [ -z "$COMMIT_SOURCE" ]; then
    NAME=$(git config user.name)
    EMAIL=$(git config user.email)
    if [ -n "$NAME" ] && [ -n "$EMAIL" ]; then
        # Add SOB if not already present
        if ! grep -q "Signed-off-by:" "$COMMIT_MSG_FILE"; then
            echo "" >> "$COMMIT_MSG_FILE"
            echo "Signed-off-by: $NAME <$EMAIL>" >> "$COMMIT_MSG_FILE"
        fi
    fi
fi
SOB_EOF
chmod +x .git/hooks/prepare-commit-msg
ok "prepare-commit-msg hook installed (auto Signed-off-by)"

echo ""

# ── 2. Toolchain checks ───────────────────────────────────────────────────
info "Checking toolchains..."

# Rust
if command -v rustc &>/dev/null; then
    RUST_VER=$(rustc --version | awk '{print $2}')
    ok "Rust $RUST_VER"
else
    fail "Rust not found. Install: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
fi

# Rust nightly + components
if command -v rustup &>/dev/null; then
    CHANNEL="nightly-2026-06-01"
    if rustup toolchain list 2>/dev/null | grep -q "$CHANNEL"; then
        ok "Rust toolchain $CHANNEL"
    else
        warn "Rust toolchain $CHANNEL not installed. Installing..."
        rustup toolchain install "$CHANNEL" \
            --component clippy rustfmt rust-src \
            --target x86_64-unknown-none 2>/dev/null || \
        warn "Install failed — run manually: rustup toolchain install $CHANNEL"
    fi
    # Add no_std targets
    for target in x86_64-unknown-none aarch64-unknown-none; do
        rustup target add "$target" --toolchain "$CHANNEL" 2>/dev/null || true
    done
    ok "no_std targets: x86_64-unknown-none, aarch64-unknown-none"
fi

# Nim
if command -v nim &>/dev/null; then
    NIM_VER=$(nim --version | head -1 | awk '{print $4}')
    ok "Nim $NIM_VER"
else
    warn "Nim not found. Install: https://nim-lang.org/install.html"
fi

# Zig
if command -v zig &>/dev/null; then
    ZIG_VER=$(zig version)
    ok "Zig $ZIG_VER"
else
    warn "Zig not found. Install: https://ziglang.org/download/"
fi

# QEMU
if command -v qemu-system-x86_64 &>/dev/null; then
    QEMU_VER=$(qemu-system-x86_64 --version | head -1 | awk '{print $4}')
    ok "QEMU $QEMU_VER"
else
    warn "QEMU not found. Install: sudo apt install qemu-system-x86"
fi

# NASM
if command -v nasm &>/dev/null; then
    NASM_VER=$(nasm --version | head -1 | awk '{print $3}')
    ok "NASM $NASM_VER"
else
    warn "NASM not found. Install: sudo apt install nasm"
fi

# cargo-audit
if command -v cargo-audit &>/dev/null; then
    ok "cargo-audit"
else
    info "Installing cargo-audit..."
    cargo install cargo-audit --quiet && ok "cargo-audit installed" || \
        warn "cargo-audit install failed — run: cargo install cargo-audit"
fi

# GDB
if command -v gdb &>/dev/null; then
    GDB_VER=$(gdb --version | head -1)
    ok "$GDB_VER"
else
    warn "GDB not found (needed for kernel debugging). Install: sudo apt install gdb"
fi

echo ""

# ── 3. Git config ─────────────────────────────────────────────────────────
info "Configuring git..."
git config --local commit.gpgsign false 2>/dev/null || true
git config --local pull.rebase true 2>/dev/null && ok "pull.rebase = true"
git config --local push.default current 2>/dev/null && ok "push.default = current"

echo ""

# ── 4. Python dev deps ────────────────────────────────────────────────────
info "Checking Python environment..."
if command -v python3 &>/dev/null; then
    PY_VER=$(python3 --version | awk '{print $2}')
    ok "Python $PY_VER"
else
    warn "Python 3 not found — sigma-pkg and kabi/check.py won't work"
fi

echo ""

# ── 5. Pre-commit framework (optional) ────────────────────────────────────
if command -v pre-commit &>/dev/null && [ -f .pre-commit-config.yaml ]; then
    info "Installing pre-commit hooks..."
    pre-commit install --install-hooks 2>/dev/null && ok "pre-commit hooks installed" || \
        warn "pre-commit install failed"
fi

# ── Summary ───────────────────────────────────────────────────────────────
echo ""
echo "  $(printf '─%.0s' {1..50})"
if [ "$FAILED" -eq 0 ]; then
    echo -e "  ${GREEN}Setup complete!${NC} You're ready to contribute to SigmaOS."
else
    echo -e "  ${YELLOW}Setup complete with $FAILED warning(s).${NC}"
    echo    "  Fix the warnings above before submitting PRs."
fi
echo ""
echo "  Quick start:"
echo "    cargo build --manifest-path sigma-sh/Cargo.toml"
echo "    python sigma-pkg/sigma_pkg_install.py list"
echo "    make qemu   # boot in QEMU (when kernel is ready)"
echo ""
