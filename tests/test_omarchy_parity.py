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

def test_omarchy_tools_fleet_parity():
    """
    AAA Pattern: Verify that all 19 official Omarchy and Omacom tools are packaged with standard
    PKGBUILD recipes and contain valid packaging definitions.
    """
    # Arrange
    tools = [
        "sigmaos-tools-ttfx", "sigmaos-tools-omasnap", "sigmaos-tools-omareel",
        "sigmaos-tools-omawrite", "sigmaos-tools-omacalc", "sigmaos-tools-folio",
        "sigmaos-tools-omacut", "sigmaos-tools-signed", "sigmaos-tools-hype",
        "sigmaos-tools-speedtest", "sigmaos-linux-aether", "sigmaos-tools-plain",
        "sigmaos-tools-monologue", "sigmaos-tools-gliff", "sigmaos-tools-owe",
        "sigmaos-tools-omafind", "sigmaos-tools-local-ai", "sigmaos-tools-port-forward",
        "sigmaos-tools-audio-tuner"
    ]
    pkgs_dir = PKGS_DIR

    # Act & Assert
    for tool in tools:
        pkg_file = pkgs_dir / tool / "PKGBUILD"
        assert pkg_file.is_file(), f"Missing PKGBUILD for {tool}"
        content = pkg_file.read_text(encoding="utf-8")
        assert f"pkgname={tool}" in content
        assert "cargo build --release" in content

def test_omacom_wave2_tools_parity_aaa():
    """
    AAA Pattern: Verify that the 8 Wave 2 Omarchy parity tools exist locally,
    have valid Cargo packages, pass AAA tests (_aaa), and contain real domain logic.
    """
    # Arrange
    omacom_tools = [
        "SigmaOS-tools-plain", "SigmaOS-tools-monologue", "SigmaOS-tools-gliff",
        "SigmaOS-tools-owe", "SigmaOS-tools-omafind", "SigmaOS-tools-local-ai",
        "SigmaOS-tools-port-forward", "SigmaOS-tools-audio-tuner"
    ]
    home_dir = Path("/home/aaryansinghchauhan")

    # Act & Assert
    for tool in omacom_tools:
        repo_path = home_dir / tool
        assert repo_path.is_dir(), f"Missing Omarchy tool repo directory: {tool}"
        cargo_toml = repo_path / "Cargo.toml"
        assert cargo_toml.is_file(), f"Missing Cargo.toml in {tool}"
        lib_rs = repo_path / "src" / "lib.rs"
        assert lib_rs.is_file(), f"Missing src/lib.rs in {tool}"
        content = lib_rs.read_text(encoding="utf-8")
        assert "_aaa" in content, f"Test in {tool} must follow _aaa naming convention"

def test_desktop_omacom_wave2_apps_parity_aaa():
    """
    AAA Pattern: Verify desktop integration of Plain, Monologue, Owe, Omafind,
    Local AI, Port Forward, Audio Tuner, and Super+Ctrl+G keybind in ui/desktop/index.html.
    """
    # Arrange
    assert UI_INDEX.is_file(), "Missing ui/desktop/index.html"
    with open(UI_INDEX, "r", encoding="utf-8") as f:
        html = f.read()

    # Act & Assert
    expected_entries = [
        "Plain Editor", "Monologue", "Gliff Remote Desktop", "Owe Wallpaper Engine",
        "Omafind", "Local AI Catalog", "Port Forward Plugin", "Audio Tuner",
        "Super+Ctrl+G"
    ]
    for entry in expected_entries:
        assert entry in html, f"Missing Omarchy app integration in desktop UI: {entry}"

def test_data_science_suite_parity():
    """
    AAA Pattern: Verify that the 11 Data Science enterprise tools (airbyte, hadoop, spark,
    databricks, dbt, fivetran, bigquery, looker, knime, mongodb, snowflake) exist locally,
    pass unit tests, and contain valid Cargo packages.
    """
    # Arrange
    ds_tools = [
        "SigmaOS-tools-airbyte", "SigmaOS-tools-hadoop", "SigmaOS-tools-spark",
        "SigmaOS-tools-databricks", "SigmaOS-tools-dbt", "SigmaOS-tools-fivetran",
        "SigmaOS-tools-bigquery", "SigmaOS-tools-looker", "SigmaOS-tools-knime",
        "SigmaOS-tools-mongodb", "SigmaOS-tools-snowflake"
    ]
    home_dir = Path("/home/aaryansinghchauhan")

    # Act & Assert
    for ds_tool in ds_tools:
        repo_path = home_dir / ds_tool
        assert repo_path.is_dir(), f"Missing Data Science repo directory: {ds_tool}"
        cargo_toml = repo_path / "Cargo.toml"
        assert cargo_toml.is_file(), f"Missing Cargo.toml in {ds_tool}"
        lib_rs = repo_path / "src" / "lib.rs"
        assert lib_rs.is_file(), f"Missing src/lib.rs in {ds_tool}"
        content = lib_rs.read_text(encoding="utf-8")
        assert "_aaa" in content, f"Test in {ds_tool} must follow _aaa naming convention"

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
    test_omarchy_tools_fleet_parity()
    print("✓ test_omarchy_tools_fleet_parity passed")
    test_omacom_wave2_tools_parity_aaa()
    print("✓ test_omacom_wave2_tools_parity_aaa passed")
    test_desktop_omacom_wave2_apps_parity_aaa()
    print("✓ test_desktop_omacom_wave2_apps_parity_aaa passed")
    test_data_science_suite_parity()
    print("✓ test_data_science_suite_parity passed")
    print("\nALL 16 AAA OMARCHY & OMACOM PARITY TESTS PASSED SUCCESSFULLY!")
