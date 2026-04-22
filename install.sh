#!/usr/bin/env bash
# SigmaOS One-Command Install Script (Linux / macOS)
# Usage: curl -fsSL https://raw.githubusercontent.com/AaryanSinghChauhan09/SigmaOS/main/install.sh | bash
set -e

SIGMA_REPO="https://github.com/AaryanSinghChauhan09/SigmaOS"
INSTALL_DIR="$HOME/.local/share/sigmaos"
BIN_DIR="$HOME/.local/bin"

info()  { echo -e "\033[36m[INFO]\033[0m  $*"; }
ok()    { echo -e "\033[32m[OK]\033[0m    $*"; }
warn()  { echo -e "\033[33m[WARN]\033[0m  $*"; }

echo -e "\033[36m  SigmaOS — Sovereign Lattice Installer v1.0\033[0m\n"

for dep in git node python3; do
    command -v $dep &>/dev/null && ok "$dep found" || warn "$dep not found — install manually"
done

if [ -d "$INSTALL_DIR" ]; then
    info "Updating $INSTALL_DIR..."
    git -C "$INSTALL_DIR" pull --rebase || warn "Update failed"
else
    info "Cloning to $INSTALL_DIR..."
    git clone --recurse-submodules "$SIGMA_REPO" "$INSTALL_DIR"
fi

cd "$INSTALL_DIR" && npm install --silent

mkdir -p "$BIN_DIR"
cat > "$BIN_DIR/sigmactl" << EOF
#!/usr/bin/env bash
exec python3 "$INSTALL_DIR/sigmactl.py" "\$@"
EOF
chmod +x "$BIN_DIR/sigmactl"
ok "sigmactl installed to $BIN_DIR/sigmactl"

python3 "$INSTALL_DIR/sigmactl.py" wizard

ok "SigmaOS installed!"
info "Start Zenith: cd $INSTALL_DIR && node server.js"
info "Use CLI:      sigmactl --help"
info "Add to PATH:  export PATH=\"\$HOME/.local/bin:\$PATH\""
