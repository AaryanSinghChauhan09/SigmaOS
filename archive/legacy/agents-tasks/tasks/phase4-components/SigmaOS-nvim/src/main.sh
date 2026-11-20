#!/usr/bin/env bash
# SigmaOS Neovim — SigmaOS opinionated Neovim distribution setup scaffold
# Upstream: LazyVim (https://www.lazyvim.org/), AstroNvim (https://astronvim.com/)
set -euo pipefail

SIGMA_NVIM_DIR="${SIGMA_NVIM_DIR:-$HOME/.config/nvim-sigmaos}"
SIGMA_NVIM_DATA="${SIGMA_NVIM_DATA:-$HOME/.local/share/nvim-sigmaos}"
NVIM_MIN_VERSION="0.9"

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") <subcommand> [options]

SigmaOS Neovim distribution manager

Subcommands:
  install     Install SigmaOS Neovim config and plugins
  update      Update plugins and config to latest
  reset       Reset to clean SigmaOS default config
  status      Show install and plugin status
  plugins     List configured plugins
  launch      Launch Neovim with SigmaOS config
  doctor      Run health checks
  backup      Backup existing Neovim config before install
  version     Print manager version

EOF
}

check_nvim() {
    if ! command -v nvim &>/dev/null; then
        echo "[nvim] nvim not found — install with: sigmaos-pkg install neovim" >&2
        exit 1
    fi
}

cmd_install() {
    echo "[nvim] Installing SigmaOS Neovim distribution..."
    echo "  Config dir : $SIGMA_NVIM_DIR"
    echo "  Data dir   : $SIGMA_NVIM_DATA"
    mkdir -p "$SIGMA_NVIM_DIR/lua/sigmaos" "$SIGMA_NVIM_DATA"
    # Write a minimal init.lua scaffold
    cat > "$SIGMA_NVIM_DIR/init.lua" <<'LUAEOF'
-- SigmaOS Neovim distribution init.lua
-- Upstream: LazyVim, AstroNvim
vim.g.mapleader = " "
vim.g.maplocalleader = "\\"

-- Bootstrap lazy.nvim
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
  vim.fn.system({ "git", "clone", "--filter=blob:none",
    "https://github.com/folke/lazy.nvim.git", "--branch=stable", lazypath })
end
vim.opt.rtp:prepend(lazypath)

-- Core settings
vim.opt.number         = true
vim.opt.relativenumber = true
vim.opt.expandtab      = true
vim.opt.tabstop        = 4
vim.opt.shiftwidth     = 4
vim.opt.termguicolors  = true
vim.opt.signcolumn     = "yes"

require("lazy").setup({
  { "folke/tokyonight.nvim",    priority = 1000 },  -- SigmaOS default theme
  { "nvim-telescope/telescope.nvim", tag = "0.1.5", dependencies = { "nvim-lua/plenary.nvim" } },
  { "nvim-treesitter/nvim-treesitter", build = ":TSUpdate" },
  { "neovim/nvim-lspconfig" },
  { "hrsh7th/nvim-cmp" },
  { "lewis6991/gitsigns.nvim" },
  { "nvim-lualine/lualine.nvim" },
})

vim.cmd.colorscheme("tokyonight-night")
LUAEOF
    echo "[nvim] init.lua written"
    echo "[nvim] Launch Neovim to install plugins: $(basename "$0") launch"
}

cmd_update() {
    check_nvim
    echo "[nvim] Updating SigmaOS Neovim plugins..."
    NVIM_APPNAME="nvim-sigmaos" nvim --headless "+Lazy update" +qa 2>/dev/null || \
        echo "[nvim] (headless update ran — open Neovim to confirm)"
}

cmd_reset() {
    echo "[nvim] Resetting SigmaOS Neovim config..."
    rm -rf "$SIGMA_NVIM_DIR" "$SIGMA_NVIM_DATA"
    echo "[nvim] Config cleared. Run: $(basename "$0") install"
}

cmd_status() {
    echo "Config dir : $SIGMA_NVIM_DIR ($([ -d "$SIGMA_NVIM_DIR" ] && echo exists || echo missing))"
    echo "Data dir   : $SIGMA_NVIM_DATA ($([ -d "$SIGMA_NVIM_DATA" ] && echo exists || echo missing))"
    if command -v nvim &>/dev/null; then
        echo "Neovim     : $(nvim --version | head -1)"
    else
        echo "Neovim     : not installed"
    fi
    local plugins=0
    [ -d "$SIGMA_NVIM_DATA/lazy" ] && plugins=$(ls "$SIGMA_NVIM_DATA/lazy" 2>/dev/null | wc -l)
    echo "Plugins    : $plugins installed"
}

cmd_plugins() {
    cat <<'EOF'
SigmaOS Neovim curated plugins:
  folke/tokyonight.nvim         — SigmaOS default colorscheme
  nvim-telescope/telescope.nvim — fuzzy finder (file/grep/lsp)
  nvim-treesitter/nvim-treesitter — syntax highlighting
  neovim/nvim-lspconfig         — LSP client configuration
  hrsh7th/nvim-cmp              — completion engine
  lewis6991/gitsigns.nvim       — git signs in gutter
  nvim-lualine/lualine.nvim     — status line
EOF
}

cmd_launch() {
    check_nvim
    echo "[nvim] Launching SigmaOS Neovim..."
    exec env NVIM_APPNAME="nvim-sigmaos" nvim "$@"
}

cmd_doctor() {
    check_nvim
    echo "[nvim] Running Neovim health checks..."
    NVIM_APPNAME="nvim-sigmaos" nvim --headless "+checkhealth" +qa 2>&1 | head -40 || \
        echo "[nvim] Health check complete (see above for warnings)"
}

cmd_backup() {
    local ts
    ts=$(date +%Y%m%d_%H%M%S)
    local bak="${SIGMA_NVIM_DIR}.bak-$ts"
    if [ -d "$SIGMA_NVIM_DIR" ]; then
        cp -r "$SIGMA_NVIM_DIR" "$bak"
        echo "[nvim] Backed up: $bak"
    else
        echo "[nvim] Nothing to backup (config dir does not exist)"
    fi
}

case "${1:-}" in
    install) cmd_install ;;
    update)  cmd_update ;;
    reset)   cmd_reset ;;
    status)  cmd_status ;;
    plugins) cmd_plugins ;;
    launch)  shift; cmd_launch "$@" ;;
    doctor)  cmd_doctor ;;
    backup)  cmd_backup ;;
    version) echo "SigmaOS Neovim distribution manager v0.5" ;;
    -h|--help|"") usage ;;
    *) echo "Unknown subcommand: $1" >&2; usage; exit 1 ;;
esac
