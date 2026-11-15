#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""
test_omarchy_parity.py: AAA Automated Test Suite for Omarchy & Omacom Parity in SigmaOS.
Ensures full parity between SigmaOS and Omarchy Linux desktop configurations, ISO pipelines,
PKGBUILD packaging, Gliff symbol picker, Self-Healing engine, and dynamic graphical UI suites.
"""

import os
import json
import subprocess
from pathlib import Path

ROOT_DIR = Path(__file__).resolve().parent.parent
CONFIG_DIR = ROOT_DIR / "config" / "omarchy"
SCRIPTS_DIR = ROOT_DIR / "scripts"
UI_INDEX = ROOT_DIR / "ui" / "desktop" / "index.html"
ISO_DIR = ROOT_DIR / "iso" / "archiso"
PKGS_DIR = ROOT_DIR / "pkgs"

def test_hyprland_configs_exist_and_nonempty():
    """Arrange-Act-Assert: Verify all Hyprland suite configs exist and are non-empty."""
    expected_files = [
        CONFIG_DIR / "hyprland" / "hyprland.conf",
        CONFIG_DIR / "hyprland" / "hyprland-binds.conf",
        CONFIG_DIR / "hyprland" / "hypridle.conf",
        CONFIG_DIR / "hyprland" / "hyprlock.conf",
    ]
    for p in expected_files:
        assert p.is_file(), f"Missing config: {p}"
        assert p.stat().st_size > 50, f"Config {p} is too small / stub"

def test_waybar_config_valid_json():
    """Arrange-Act-Assert: Verify Waybar config is valid JSON/JSONC."""
    wb_cfg = CONFIG_DIR / "waybar" / "config.jsonc"
    assert wb_cfg.is_file(), f"Missing waybar config: {wb_cfg}"
    with open(wb_cfg, "r", encoding="utf-8") as f:
        content = f.read()
    lines = [line for line in content.splitlines() if not line.strip().startswith("//")]
    parsed = json.loads("\n".join(lines))
    assert "layer" in parsed
    assert "modules-left" in parsed
    assert "modules-right" in parsed

def test_wallust_presets_valid():
    """Arrange-Act-Assert: Verify Wallust theme presets exist and have 16 colors."""
    preset_dir = CONFIG_DIR / "wallust" / "presets"
    assert preset_dir.is_dir()
    presets = list(preset_dir.glob("*.json"))
    assert len(presets) >= 5, f"Expected at least 5 presets, found {len(presets)}"
    for p in presets:
        with open(p, "r", encoding="utf-8") as f:
            data = json.load(f)
            assert "name" in data
            assert "background" in data
            assert "foreground" in data
            assert len(data.get("colors", [])) == 16, f"Preset {p.name} must define 16 ANSI colors"

def test_terminal_configs_exist():
    """Arrange-Act-Assert: Verify Foot, Alacritty, and Kitty configs exist and are non-empty."""
    term_configs = [
        CONFIG_DIR / "foot" / "foot.ini",
        CONFIG_DIR / "alacritty" / "alacritty.toml",
        CONFIG_DIR / "kitty" / "kitty.conf",
    ]
    for p in term_configs:
        assert p.is_file(), f"Missing terminal config: {p}"
        assert p.stat().st_size > 100, f"Terminal config {p} is too small"

def test_notification_and_shell_configs():
    """Arrange-Act-Assert: Verify Dunst, Mako, Wlogout, Starship, and Neovim configs."""
    configs = [
        CONFIG_DIR / "dunst" / "dunstrc",
        CONFIG_DIR / "mako" / "config",
        CONFIG_DIR / "wlogout" / "layout",
        CONFIG_DIR / "wlogout" / "style.css",
        CONFIG_DIR / "starship" / "starship.toml",
        CONFIG_DIR / "nvim" / "init.lua",
    ]
    for p in configs:
        assert p.is_file(), f"Missing config: {p}"
        assert p.stat().st_size > 50, f"Config {p} is too small"

def test_omarchy_shell_cli_execution():
    """Arrange-Act-Assert: Verify omarchy-shell binary executes and outputs binds."""
    binary = ROOT_DIR / "bin" / "omarchy-shell"
    assert binary.is_file(), f"Missing binary: {binary}"
    res = subprocess.run([str(binary), "binds"], capture_output=True, text=True)
    assert res.returncode == 0
    assert "bind = SUPER+SHIFT, Return, exec, omarchy terminal" in res.stdout

def test_apply_theme_orchestrator():
    """Arrange-Act-Assert: Verify apply_theme.py executes without error."""
    script = CONFIG_DIR / "scripts" / "apply_theme.py"
    assert script.is_file()
    res = subprocess.run([str(script), "list"], capture_output=True, text=True)
    assert res.returncode == 0
    assert "catppuccin-mocha" in res.stdout

def test_desktop_gui_parity():
    """Arrange-Act-Assert: Verify all 20 Craft modules, GW-BASIC, Tools suite, Folio, Omarchy shell, Plain editor, Marketplace, and Notification Drawer."""
    assert UI_INDEX.is_file(), "Missing ui/desktop/index.html"
    with open(UI_INDEX, "r", encoding="utf-8") as f:
        html = f.read()

    # All 20 Craft modules
    craft_modules = [
        "3DCraft", "AniCraft", "ArtCraft", "CADCraft", "CraftLibs",
        "DeckCraft", "DesignCraft", "EffectCraft", "FigCraft", "FilmCraft",
        "FontCraft", "GridCraft", "LightCraft", "PDFCraft", "PhotoCraft",
        "PrintCraft", "SoundCraft", "VectorCraft", "VoiceCraft", "WordCraft"
    ]
    for mod in craft_modules:
        assert mod in html, f"Missing Craft module: {mod}"

    # Essential Sovereign Windows & Drawers
    expected_windows = [
        'id="win-term"',
        'id="win-wallust"',
        'id="win-betteroffice"',
        'id="win-dos"',
        'id="win-basic"',
        'id="win-tools"',
        'id="win-folio"',
        'id="win-omarchy-term"',
        'id="win-plain"',
        'id="win-marketplace"',
        'id="win-datascience"',
        'id="notif-drawer"',
        'id="gliff-modal"',
        'id="desktop-context-menu"'
    ]
    for win in expected_windows:
        assert win in html, f"Missing desktop window or component: {win}"

def test_plugins_and_omacom_parity():
    """Arrange-Act-Assert: Verify omacom-inspired plugins, shell configs, and Neovim themes."""
    plugin_files = [
        CONFIG_DIR / "plugins" / "theme-sync.js",
        CONFIG_DIR / "plugins" / "local-ai-catalog.json",
        CONFIG_DIR / "plugins" / "port-forward.json",
        CONFIG_DIR / "zsh" / ".zshrc",
        CONFIG_DIR / "fish" / "config.fish",
        CONFIG_DIR / "nvim" / "colors" / "aether.lua",
        CONFIG_DIR / "nvim" / "colors" / "lumon.lua",
    ]
    for p in plugin_files:
        assert p.is_file(), f"Missing Omacom parity file: {p}"
        assert p.stat().st_size > 20, f"File {p} is empty or stub"

def test_archiso_and_pkgs_parity():
    """Arrange-Act-Assert: Verify archiso live ISO pipeline and PKGBUILD package definitions (omacom/omarchy-iso & omacom/omarchy-pkgs)."""
    assert (ISO_DIR / "profiledef.sh").is_file()
    assert (ISO_DIR / "packages.x86_64").is_file()
    assert (ISO_DIR / "airootfs" / "etc" / "os-release").is_file()
    assert (ISO_DIR / "airootfs" / "etc" / "motd").is_file()

    expected_pkgs = [
        PKGS_DIR / "sigmaos-omarchy-shell" / "PKGBUILD",
        PKGS_DIR / "sigmaos-desktop-meta" / "PKGBUILD",
        PKGS_DIR / "sigmaos-wallust-themes" / "PKGBUILD",
        PKGS_DIR / "sigmaos-tools-ttfx" / "PKGBUILD",
        PKGS_DIR / "sigmaos-tools-folio" / "PKGBUILD",
    ]
    for pkg in expected_pkgs:
        assert pkg.is_file(), f"Missing PKGBUILD: {pkg}"
        with open(pkg) as f:
            c = f.read()
            assert "pkgname=" in c
            assert "pkgver=" in c
            assert "package()" in c

def test_gliff_and_self_healing_parity():
    """Arrange-Act-Assert: Verify Gliff symbol modal, Wallust theme registry, and WeCanFixEverything self-healing engine."""
    # 1. Gliff modal in HTML
    with open(UI_INDEX, "r", encoding="utf-8") as f:
        html = f.read()
    assert "openGliffModal" in html
    assert "gliff-modal" in html

    # 2. Theme Registry JSON
    registry_file = CONFIG_DIR / "wallust" / "registry.json"
    assert registry_file.is_file()
    with open(registry_file) as f:
        reg = json.load(f)
        assert len(reg.get("themes", [])) >= 8

    # 3. WeCanFixEverything script
    fix_script = SCRIPTS_DIR / "wecanfixeverything.py"
    assert fix_script.is_file()
    res = subprocess.run(["python3", str(fix_script)], capture_output=True, text=True)
    assert res.returncode == 0
    assert "ALL CHECKS PASSED" in res.stdout

def test_lazyvim_and_voice_dictation_parity():
    """Arrange-Act-Assert: Verify Pixel Neovim theme, LazyVim plugins, WKD signature validator, and voice dictation."""
    # 1. Neovim colorschemes & plugins
    assert (CONFIG_DIR / "nvim" / "colors" / "pixel.lua").is_file()
    for plugin in ["lsp.lua", "treesitter.lua", "telescope.lua", "lualine.lua"]:
        assert (CONFIG_DIR / "nvim" / "lua" / "plugins" / plugin).is_file()

    # 2. PGP / WKD signature verification script
    wkd_script = SCRIPTS_DIR / "verify_signatures.py"
    assert wkd_script.is_file()
    res_wkd = subprocess.run(["python3", str(wkd_script)], capture_output=True, text=True)
    assert res_wkd.returncode == 0
    assert "WKD (Web Key Directory)" in res_wkd.stdout

    # 3. Voice dictation engine
    voice_script = SCRIPTS_DIR / "voice_dictation.py"
    assert voice_script.is_file()
    res_voice = subprocess.run(["python3", str(voice_script)], capture_output=True, text=True)
    assert res_voice.returncode == 0
    assert "Whisper ASR" in res_voice.stdout

    # 4. Voice dictation button in HTML
    with open(UI_INDEX, "r", encoding="utf-8") as f:
        html = f.read()
    assert "btn-dictate" in html
    assert "toggleDictation" in html

if __name__ == "__main__":
    test_hyprland_configs_exist_and_nonempty()
    print("✓ test_hyprland_configs_exist_and_nonempty passed")
    test_waybar_config_valid_json()
    print("✓ test_waybar_config_valid_json passed")
    test_wallust_presets_valid()
    print("✓ test_wallust_presets_valid passed")
    test_terminal_configs_exist()
    print("✓ test_terminal_configs_exist passed")
    test_notification_and_shell_configs()
    print("✓ test_notification_and_shell_configs passed")
    test_omarchy_shell_cli_execution()
    print("✓ test_omarchy_shell_cli_execution passed")
    test_apply_theme_orchestrator()
    print("✓ test_apply_theme_orchestrator passed")
    test_desktop_gui_parity()
    print("✓ test_desktop_gui_parity passed")
    test_plugins_and_omacom_parity()
    print("✓ test_plugins_and_omacom_parity passed")
    test_archiso_and_pkgs_parity()
    print("✓ test_archiso_and_pkgs_parity passed")
    test_gliff_and_self_healing_parity()
    print("✓ test_gliff_and_self_healing_parity passed")
    test_lazyvim_and_voice_dictation_parity()
    print("✓ test_lazyvim_and_voice_dictation_parity passed")
    print("\nALL 12 AAA OMARCHY & OMACOM PARITY TESTS PASSED SUCCESSFULLY!")
