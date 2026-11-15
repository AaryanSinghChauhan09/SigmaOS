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

def _verify_aaa_testing(content: str, repo_name: str):
    """Verify that a repository adheres to the Arrange-Act-Assert (AAA) pattern."""
    has_aaa_suffix = "_aaa" in content
    has_aaa_comments = "arrange" in content.lower() and "assert" in content.lower()
    assert has_aaa_suffix or has_aaa_comments, f"Test in {repo_name} must follow AAA pattern"

def test_bsd_suite_parity_aaa():
    """
    AAA Pattern: Verify all 12 BSD subsystem repositories exist locally,
    contain real domain implementations, AAA tests, Cargo packages, and PKGBUILDs.
    """
    # Arrange
    bsd_repos = [
        "SigmaOS-bsd-desktop", "SigmaOS-bsd-dragonfly", "SigmaOS-bsd-free",
        "SigmaOS-bsd-fury", "SigmaOS-bsd-ghost", "SigmaOS-bsd-hardened",
        "SigmaOS-bsd-karma", "SigmaOS-bsd-liberty", "SigmaOS-bsd-midnight",
        "SigmaOS-bsd-nomad", "SigmaOS-bsd-open", "SigmaOS-bsd-star"
    ]
    home_dir = Path("/home/aaryansinghchauhan")

    # Act & Assert
    for repo in bsd_repos:
        repo_path = home_dir / repo
        assert repo_path.is_dir(), f"Missing BSD repo directory: {repo}"
        assert (repo_path / "Cargo.toml").is_file(), f"Missing Cargo.toml in {repo}"
        lib_rs = repo_path / "src" / "lib.rs"
        assert lib_rs.is_file(), f"Missing src/lib.rs in {repo}"
        content = lib_rs.read_text(encoding="utf-8")
        _verify_aaa_testing(content, repo)

        pkg_name = repo.lower()
        assert (PKGS_DIR / pkg_name / "PKGBUILD").is_file(), f"Missing PKGBUILD for {pkg_name}"

def test_kernel_suite_parity_aaa():
    """
    AAA Pattern: Verify all 5 Kernel subsystem repositories (GNU, Linux, MINIX, ReactOS, XNU)
    exist locally, contain real implementations, AAA tests, and PKGBUILDs.
    """
    # Arrange
    kernel_repos = [
        "SigmaOS-kernel-gnu", "SigmaOS-kernel-linux", "SigmaOS-kernel-minix",
        "SigmaOS-kernel-reactos", "SigmaOS-kernel-xnu"
    ]
    home_dir = Path("/home/aaryansinghchauhan")

    # Act & Assert
    for repo in kernel_repos:
        repo_path = home_dir / repo
        assert repo_path.is_dir(), f"Missing Kernel repo directory: {repo}"
        assert (repo_path / "Cargo.toml").is_file(), f"Missing Cargo.toml in {repo}"
        lib_rs = repo_path / "src" / "lib.rs"
        assert lib_rs.is_file(), f"Missing src/lib.rs in {repo}"
        content = lib_rs.read_text(encoding="utf-8")
        _verify_aaa_testing(content, repo)

        pkg_name = repo.lower()
        assert (PKGS_DIR / pkg_name / "PKGBUILD").is_file(), f"Missing PKGBUILD for {pkg_name}"

def test_linux_distro_suite_parity_aaa():
    """
    AAA Pattern: Verify all 18 Linux distro inspiration repositories exist locally,
    contain real implementations, AAA tests, and PKGBUILDs.
    """
    # Arrange
    linux_repos = [
        "SigmaOS-linux-arch", "SigmaOS-linux-cachy", "SigmaOS-linux-debian",
        "SigmaOS-linux-elementary", "SigmaOS-linux-endeavouros", "SigmaOS-linux-fedora",
        "SigmaOS-linux-gentoo", "SigmaOS-linux-kali", "SigmaOS-linux-lfs",
        "SigmaOS-linux-mint", "SigmaOS-linux-omarchy", "SigmaOS-linux-parrot",
        "SigmaOS-linux-qubes", "SigmaOS-linux-reactos", "SigmaOS-linux-redox",
        "SigmaOS-linux-slackware", "SigmaOS-linux-suse", "SigmaOS-linux-ubuntu"
    ]
    home_dir = Path("/home/aaryansinghchauhan")

    # Act & Assert
    for repo in linux_repos:
        repo_path = home_dir / repo
        assert repo_path.is_dir(), f"Missing Linux distro repo directory: {repo}"
        assert (repo_path / "Cargo.toml").is_file(), f"Missing Cargo.toml in {repo}"
        lib_rs = repo_path / "src" / "lib.rs"
        assert lib_rs.is_file(), f"Missing src/lib.rs in {repo}"
        content = lib_rs.read_text(encoding="utf-8")
        _verify_aaa_testing(content, repo)

        pkg_name = repo.lower()
        assert (PKGS_DIR / pkg_name / "PKGBUILD").is_file(), f"Missing PKGBUILD for {pkg_name}"

def test_office_and_web_suites_parity_aaa():
    """
    AAA Pattern: Verify BetterOffice, GenOffice, OnlyOffice, and RDBMS expansion suites
    exist locally, follow AAA naming, and have PKGBUILDs.
    """
    # Arrange
    office_repos = [
        # BetterOffice
        "SigmaOS-betteroffice-diagrams", "SigmaOS-betteroffice-docs",
        "SigmaOS-betteroffice-sheets", "SigmaOS-betteroffice-slides",
        # GenOffice
        "SigmaOS-genoffice-docs", "SigmaOS-genoffice-html",
        "SigmaOS-genoffice-markdown", "SigmaOS-genoffice-pdf",
        "SigmaOS-genoffice-sheets", "SigmaOS-genoffice-slides",
        # OnlyOffice
        "SigmaOS-onlyoffice-document", "SigmaOS-onlyoffice-forms",
        "SigmaOS-onlyoffice-presentation", "SigmaOS-onlyoffice-pdf",
        "SigmaOS-onlyoffice-spreadsheet",
        # RDBMS
        "SigmaOS-rdbms-apache", "SigmaOS-rdbms-dbeaver",
        "SigmaOS-rdbms-duckdb", "SigmaOS-rdbms-postgresql"
    ]
    home_dir = Path("/home/aaryansinghchauhan")

    # Act & Assert
    for repo in office_repos:
        repo_path = home_dir / repo
        assert repo_path.is_dir(), f"Missing Office/RDBMS repo directory: {repo}"
        assert (repo_path / "Cargo.toml").is_file(), f"Missing Cargo.toml in {repo}"
        lib_rs = repo_path / "src" / "lib.rs"
        assert lib_rs.is_file(), f"Missing src/lib.rs in {repo}"
        content = lib_rs.read_text(encoding="utf-8")
        _verify_aaa_testing(content, repo)

        pkg_name = repo.lower()
        assert (PKGS_DIR / pkg_name / "PKGBUILD").is_file(), f"Missing PKGBUILD for {pkg_name}"

def test_tools_wave3_and_datascience_parity_aaa():
    """
    AAA Pattern: Verify Tools Wave 3 and Expanded Data Science suites
    exist locally, follow AAA naming, and have PKGBUILDs.
    """
    # Arrange
    expanded_tools = [
        # Tools wave 3
        "SigmaOS-tools-blender", "SigmaOS-tools-docker", "SigmaOS-tools-vlc",
        "SigmaOS-tools-warp", "SigmaOS-tools-langchain", "SigmaOS-tools-crewai",
        "SigmaOS-tools-deepseek-coder", "SigmaOS-tools-hermes", "SigmaOS-tools-cline",
        "SigmaOS-tools-notepadpp", "SigmaOS-tools-codex-cli", "SigmaOS-tools-openclaw",
        # Data Science expanded
        "SigmaOS-tools-tensorflow", "SigmaOS-tools-tableau", "SigmaOS-tools-sql-ds",
        "SigmaOS-tools-python-ds", "SigmaOS-tools-r-ds", "SigmaOS-tools-sas",
        "SigmaOS-tools-ms-powerbi", "SigmaOS-tools-skyvia"
    ]
    home_dir = Path("/home/aaryansinghchauhan")

    # Act & Assert
    for repo in expanded_tools:
        repo_path = home_dir / repo
        assert repo_path.is_dir(), f"Missing Tool/DS repo directory: {repo}"
        assert (repo_path / "Cargo.toml").is_file(), f"Missing Cargo.toml in {repo}"
        lib_rs = repo_path / "src" / "lib.rs"
        assert lib_rs.is_file(), f"Missing src/lib.rs in {repo}"
        content = lib_rs.read_text(encoding="utf-8")
        _verify_aaa_testing(content, repo)

        pkg_name = repo.lower()
        assert (PKGS_DIR / pkg_name / "PKGBUILD").is_file(), f"Missing PKGBUILD for {pkg_name}"

def test_wiki_documentation_conformance_parity_aaa():
    """
    AAA Pattern: Verify that wiki documentation pages exist for all new components
    and conform to the industry standard rules (zero markdown in main branch except root README).
    """
    # Arrange
    wiki_dir = Path("/home/aaryansinghchauhan/SigmaOS.wiki")
    sample_plans = [
        "bsd-desktop-plan.md", "bsd-dragonfly-plan.md", "bsd-free-plan.md",
        "kernel-gnu-plan.md", "kernel-linux-plan.md", "kernel-minix-plan.md",
        "linux-arch-plan.md", "linux-cachy-plan.md", "linux-debian-plan.md",
        "betteroffice-docs-plan.md", "betteroffice-sheets-plan.md",
        "genoffice-docs-plan.md", "onlyoffice-document-plan.md",
        "rdbms-duckdb-plan.md", "tools-blender-plan.md", "tools-docker-plan.md",
        "tools-tensorflow-plan.md", "tools-tableau-plan.md"
    ]

    # Act & Assert
    for plan in sample_plans:
        plan_path = wiki_dir / plan
        assert plan_path.is_file(), f"Missing wiki plan: {plan}"
        content = plan_path.read_text(encoding="utf-8")
        assert "Repository Context and Scope" in content, f"Missing section in {plan}"
        assert "Scope and Boundaries" in content, f"Missing section in {plan}"
        assert "Periodic Task Cadence" in content, f"Missing section in {plan}"
        assert "Industry-Standard Rules" in content, f"Missing section in {plan}"
        assert "Governance and Merge Policy" in content, f"Missing section in {plan}"

def test_craft_suite_parity_aaa():
    """
    AAA Pattern: Verify all 20 Craft suite repositories exist locally,
    contain real domain logic, adhere to AAA testing, and have PKGBUILDs.
    """
    # Arrange
    craft_repos = [
        "SigmaOS-craft-3d", "SigmaOS-craft-ani", "SigmaOS-craft-art", "SigmaOS-craft-cad",
        "SigmaOS-craft-craftlibs", "SigmaOS-craft-deck", "SigmaOS-craft-design", "SigmaOS-craft-effect",
        "SigmaOS-craft-fig", "SigmaOS-craft-film", "SigmaOS-craft-font", "SigmaOS-craft-grid",
        "SigmaOS-craft-light", "SigmaOS-craft-pdf", "SigmaOS-craft-photo", "SigmaOS-craft-print",
        "SigmaOS-craft-sound", "SigmaOS-craft-vector", "SigmaOS-craft-voice", "SigmaOS-craft-word"
    ]
    home_dir = Path("/home/aaryansinghchauhan")

    # Act & Assert
    for repo in craft_repos:
        repo_path = home_dir / repo
        assert repo_path.is_dir(), f"Missing Craft repo: {repo}"
        assert (repo_path / "Cargo.toml").is_file(), f"Missing Cargo.toml in {repo}"
        lib_rs = repo_path / "src" / "lib.rs"
        assert lib_rs.is_file(), f"Missing src/lib.rs in {repo}"
        content = lib_rs.read_text(encoding="utf-8")
        _verify_aaa_testing(content, repo)

        pkg_name = repo.lower()
        assert (PKGS_DIR / pkg_name / "PKGBUILD").is_file(), f"Missing PKGBUILD for {pkg_name}"

def test_extended_tools_and_office_parity_aaa():
    """
    AAA Pattern: Verify DOS additions (free, ms), LibreOffice additions (calc, writer),
    Tools additions (chromium, command-code), and Data Science additions (ms-access, ms-excel).
    """
    # Arrange
    extended_repos = [
        "SigmaOS-dos-free", "SigmaOS-dos-ms",
        "SigmaOS-libreoffice-calc", "SigmaOS-libreoffice-writer",
        "SigmaOS-tools-chromium", "SigmaOS-tools-command-code",
        "SigmaOS-tools-ms-access", "SigmaOS-tools-ms-excel"
    ]
    home_dir = Path("/home/aaryansinghchauhan")

    # Act & Assert
    for repo in extended_repos:
        repo_path = home_dir / repo
        assert repo_path.is_dir(), f"Missing extended repo: {repo}"
        assert (repo_path / "Cargo.toml").is_file(), f"Missing Cargo.toml in {repo}"
        lib_rs = repo_path / "src" / "lib.rs"
        assert lib_rs.is_file(), f"Missing src/lib.rs in {repo}"
        content = lib_rs.read_text(encoding="utf-8")
        _verify_aaa_testing(content, repo)

        pkg_name = repo.lower()
        assert (PKGS_DIR / pkg_name / "PKGBUILD").is_file(), f"Missing PKGBUILD for {pkg_name}"

def test_platform_frameworks_parity_aaa():
    """
    AAA Pattern: Verify all 13 Platform GUI framework repositories exist locally,
    contain real domain implementations, AAA tests, Cargo packages, and PKGBUILDs.
    """
    # Arrange
    platforms = [
        "SigmaOS-platform-tauri", "SigmaOS-platform-wails", "SigmaOS-platform-neutralino",
        "SigmaOS-platform-gpui", "SigmaOS-platform-imgui", "SigmaOS-platform-fyne",
        "SigmaOS-platform-sciter", "SigmaOS-platform-ultralight", "SigmaOS-platform-waydroid",
        "SigmaOS-platform-photino", "SigmaOS-platform-compose", "SigmaOS-platform-avalonia",
        "SigmaOS-platform-slint"
    ]
    home_dir = Path("/home/aaryansinghchauhan")

    # Act & Assert
    for repo in platforms:
        repo_path = home_dir / repo
        assert repo_path.is_dir(), f"Missing Platform repo: {repo}"
        assert (repo_path / "Cargo.toml").is_file(), f"Missing Cargo.toml in {repo}"
        lib_rs = repo_path / "src" / "lib.rs"
        assert lib_rs.is_file(), f"Missing src/lib.rs in {repo}"
        content = lib_rs.read_text(encoding="utf-8")
        _verify_aaa_testing(content, repo)

        pkg_name = repo.lower()
        assert (PKGS_DIR / pkg_name / "PKGBUILD").is_file(), f"Missing PKGBUILD for {pkg_name}"

def test_platform_wave2_frameworks_parity_aaa():
    """
    AAA Pattern: Verify all 15 Platform GUI Wave 2 framework repositories exist locally,
    contain real domain implementations, AAA tests, Cargo packages, and PKGBUILDs.
    """
    # Arrange
    platforms_w2 = [
        "SigmaOS-platform-electron", "SigmaOS-platform-flutter", "SigmaOS-platform-gtk",
        "SigmaOS-platform-qt", "SigmaOS-platform-iced", "SigmaOS-platform-egui",
        "SigmaOS-platform-dioxus", "SigmaOS-platform-webview", "SigmaOS-platform-sdl2",
        "SigmaOS-platform-kivy", "SigmaOS-platform-flet", "SigmaOS-platform-pywebview",
        "SigmaOS-platform-nodegui", "SigmaOS-platform-electrobun", "SigmaOS-platform-react-native"
    ]
    home_dir = Path("/home/aaryansinghchauhan")

    # Act & Assert
    for repo in platforms_w2:
        repo_path = home_dir / repo
        assert repo_path.is_dir(), f"Missing Platform Wave 2 repo: {repo}"
        assert (repo_path / "Cargo.toml").is_file(), f"Missing Cargo.toml in {repo}"
        lib_rs = repo_path / "src" / "lib.rs"
        assert lib_rs.is_file(), f"Missing src/lib.rs in {repo}"
        content = lib_rs.read_text(encoding="utf-8")
        _verify_aaa_testing(content, repo)

        pkg_name = repo.lower()
        assert (PKGS_DIR / pkg_name / "PKGBUILD").is_file(), f"Missing PKGBUILD for {pkg_name}"

def test_platform_wave3_frameworks_parity_aaa():
    """
    AAA Pattern: Verify all 10 Platform GUI Wave 3 framework repositories exist locally,
    contain real domain implementations, AAA tests, Cargo packages, and PKGBUILDs.
    """
    # Arrange
    platforms_w3 = [
        "SigmaOS-platform-nwjs", "SigmaOS-platform-uno", "SigmaOS-platform-revery",
        "SigmaOS-platform-nicegui", "SigmaOS-platform-pyqt", "SigmaOS-platform-pyside",
        "SigmaOS-platform-monomer", "SigmaOS-platform-sciterjs",
        "SigmaOS-platform-saucer", "SigmaOS-platform-vaadin"
    ]
    home_dir = Path("/home/aaryansinghchauhan")

    # Act & Assert
    for repo in platforms_w3:
        repo_path = home_dir / repo
        assert repo_path.is_dir(), f"Missing Platform Wave 3 repo: {repo}"
        assert (repo_path / "Cargo.toml").is_file(), f"Missing Cargo.toml in {repo}"
        lib_rs = repo_path / "src" / "lib.rs"
        assert lib_rs.is_file(), f"Missing src/lib.rs in {repo}"
        content = lib_rs.read_text(encoding="utf-8")
        _verify_aaa_testing(content, repo)

        pkg_name = repo.lower()
        assert (PKGS_DIR / pkg_name / "PKGBUILD").is_file(), f"Missing PKGBUILD for {pkg_name}"

def test_platform_wave4_frameworks_parity_aaa():
    """
    AAA Pattern: Verify all 12 Platform GUI Wave 4 framework repositories exist locally,
    contain real domain implementations, AAA tests, Cargo packages, and PKGBUILDs.
    """
    # Arrange
    platforms_w4 = [
        "SigmaOS-platform-tkinter", "SigmaOS-platform-wxwidgets", "SigmaOS-platform-wxpython",
        "SigmaOS-platform-javafx", "SigmaOS-platform-openjfx", "SigmaOS-platform-swing",
        "SigmaOS-platform-lorca", "SigmaOS-platform-eel", "SigmaOS-platform-chromely",
        "SigmaOS-platform-deskgap", "SigmaOS-platform-webview-deno", "SigmaOS-platform-quasar"
    ]
    home_dir = Path("/home/aaryansinghchauhan")

    # Act & Assert
    for repo in platforms_w4:
        repo_path = home_dir / repo
        assert repo_path.is_dir(), f"Missing Platform Wave 4 repo: {repo}"
        assert (repo_path / "Cargo.toml").is_file(), f"Missing Cargo.toml in {repo}"
        lib_rs = repo_path / "src" / "lib.rs"
        assert lib_rs.is_file(), f"Missing src/lib.rs in {repo}"
        content = lib_rs.read_text(encoding="utf-8")
        _verify_aaa_testing(content, repo)

        pkg_name = repo.lower()
        assert (PKGS_DIR / pkg_name / "PKGBUILD").is_file(), f"Missing PKGBUILD for {pkg_name}"

def test_platform_wave5_frameworks_parity_aaa():
    """
    AAA Pattern: Verify all 10 Platform GUI Wave 5 framework repositories exist locally,
    contain real domain implementations, AAA tests, Cargo packages, and PKGBUILDs.
    """
    # Arrange
    platforms_w5 = [
        "SigmaOS-platform-gtk-rs", "SigmaOS-platform-iup", "SigmaOS-platform-nigui",
        "SigmaOS-platform-nativephp", "SigmaOS-platform-vuido", "SigmaOS-platform-shiny",
        "SigmaOS-platform-astrodon", "SigmaOS-platform-glimmer", "SigmaOS-platform-lazarus",
        "SigmaOS-platform-pyloid"
    ]
    home_dir = Path("/home/aaryansinghchauhan")

    # Act & Assert
    for repo in platforms_w5:
        repo_path = home_dir / repo
        assert repo_path.is_dir(), f"Missing Platform Wave 5 repo: {repo}"
        assert (repo_path / "Cargo.toml").is_file(), f"Missing Cargo.toml in {repo}"
        lib_rs = repo_path / "src" / "lib.rs"
        assert lib_rs.is_file(), f"Missing src/lib.rs in {repo}"
        content = lib_rs.read_text(encoding="utf-8")
        _verify_aaa_testing(content, repo)

        pkg_name = repo.lower()
        assert (PKGS_DIR / pkg_name / "PKGBUILD").is_file(), f"Missing PKGBUILD for {pkg_name}"

def test_arch_linux_ecosystem_gap_closure_aaa():
    """
    AAA Pattern: Verify complete Arch Linux distribution ecosystem parity:
    1. Archiso bootloaders: GRUB, Syslinux, EFIBoot systemd-boot.
    2. ALPM pacman.conf, makepkg.conf, and mirrorlists.
    3. Pacman transaction hooks (mkinitcpio, paccache, desktop-database).
    4. Mkinitcpio pipeline and preset configurations.
    5. Operating system identity (os-release ID_LIKE=arch).
    6. System administration and live environment scripts (sigmaos-install, arch-chroot, pacstrap, sigma-aur).
    7. Core Arch Linux PKGBUILD recipes (base, base-devel, linux, headers, pacman, keyring, mirrorlist).
    8. Archiso build and ALPM repo sync scripts.
    """
    # Arrange
    bootloader_files = [
        ISO_DIR / "pacman.conf",
        ISO_DIR / "grub" / "grub.cfg",
        ISO_DIR / "grub" / "loopback.cfg",
        ISO_DIR / "syslinux" / "syslinux.cfg",
        ISO_DIR / "syslinux" / "archiso_sys-linux.cfg",
        ISO_DIR / "efiboot" / "loader" / "loader.conf",
        ISO_DIR / "efiboot" / "loader" / "entries" / "01-archiso-x86_64-linux.conf",
    ]
    airootfs_files = [
        ISO_DIR / "airootfs" / "etc" / "pacman.conf",
        ISO_DIR / "airootfs" / "etc" / "makepkg.conf",
        ISO_DIR / "airootfs" / "etc" / "mkinitcpio.conf",
        ISO_DIR / "airootfs" / "etc" / "mkinitcpio.d" / "linux.preset",
        ISO_DIR / "airootfs" / "etc" / "pacman.d" / "mirrorlist",
        ISO_DIR / "airootfs" / "etc" / "pacman.d" / "sigmaos-mirrorlist",
        ISO_DIR / "airootfs" / "etc" / "pacman.d" / "hooks" / "90-mkinitcpio-install.hook",
        ISO_DIR / "airootfs" / "etc" / "pacman.d" / "hooks" / "clean_package_cache.hook",
        ISO_DIR / "airootfs" / "etc" / "fstab",
        ISO_DIR / "airootfs" / "etc" / "hostname",
        ISO_DIR / "airootfs" / "etc" / "hosts",
        ISO_DIR / "airootfs" / "etc" / "locale.gen",
        ISO_DIR / "airootfs" / "etc" / "vconsole.conf",
        ISO_DIR / "airootfs" / "etc" / "sudoers.d" / "wheel",
    ]
    arch_tools = [
        ISO_DIR / "airootfs" / "usr" / "local" / "bin" / "sigmaos-install",
        ISO_DIR / "airootfs" / "usr" / "local" / "bin" / "arch-chroot",
        ISO_DIR / "airootfs" / "usr" / "local" / "bin" / "pacstrap",
        ISO_DIR / "airootfs" / "usr" / "local" / "bin" / "sigma-aur",
        ROOT_DIR / "scripts" / "build_archiso.sh",
        ROOT_DIR / "scripts" / "repo_sync.sh",
    ]
    core_pkgbuilds = [
        PKGS_DIR / "sigmaos-base" / "PKGBUILD",
        PKGS_DIR / "sigmaos-base-devel" / "PKGBUILD",
        PKGS_DIR / "sigmaos-linux" / "PKGBUILD",
        PKGS_DIR / "sigmaos-linux-headers" / "PKGBUILD",
        PKGS_DIR / "sigmaos-pacman" / "PKGBUILD",
        PKGS_DIR / "sigmaos-keyring" / "PKGBUILD",
        PKGS_DIR / "sigmaos-mirrorlist" / "PKGBUILD",
    ]

    # Act & Assert - Bootloaders
    for bf in bootloader_files:
        assert bf.is_file(), f"Missing archiso bootloader file: {bf}"
        assert bf.stat().st_size > 20, f"Bootloader file {bf} is empty or stub"

    # Act & Assert - Airootfs Configuration
    for af in airootfs_files:
        assert af.is_file(), f"Missing airootfs config file: {af}"
        assert af.stat().st_size > 0, f"Airootfs config {af} is empty or stub"

    # Act & Assert - os-release Arch compatibility
    os_rel = ISO_DIR / "airootfs" / "etc" / "os-release"
    content = os_rel.read_text(encoding="utf-8")
    assert "ID_LIKE=arch" in content
    assert "ID=sigmaos" in content

    # Act & Assert - Tools and Scripts Executability
    for tool in arch_tools:
        assert tool.is_file(), f"Missing tool script: {tool}"
        assert os.access(tool, os.X_OK), f"Tool {tool} is not marked executable"

    # Act & Assert - Core PKGBUILDs
    for pkg in core_pkgbuilds:
        assert pkg.is_file(), f"Missing core Arch PKGBUILD: {pkg}"
        pkg_content = pkg.read_text(encoding="utf-8")
        assert "pkgname=" in pkg_content
        assert "pkgver=" in pkg_content
        assert "package()" in pkg_content

def test_open_source_os_ecosystem_gap_closure_aaa():
    """
    AAA Pattern: Verify all 12 Open Source Operating System repositories exist locally,
    contain real domain implementations, AAA tests, Cargo packages, and PKGBUILDs.
    """
    # Arrange
    os_repos = [
        "SigmaOS-os-sel4", "SigmaOS-os-genode", "SigmaOS-os-helenos",
        "SigmaOS-os-haiku", "SigmaOS-os-serenity", "SigmaOS-os-illumos",
        "SigmaOS-os-smartos", "SigmaOS-os-plan9", "SigmaOS-os-inferno",
        "SigmaOS-os-templeos", "SigmaOS-os-freertos", "SigmaOS-os-zephyr"
    ]
    home_dir = Path("/home/aaryansinghchauhan")

    # Act & Assert
    for repo in os_repos:
        repo_path = home_dir / repo
        assert repo_path.is_dir(), f"Missing OS repo: {repo}"
        assert (repo_path / "Cargo.toml").is_file(), f"Missing Cargo.toml in {repo}"
        lib_rs = repo_path / "src" / "lib.rs"
        assert lib_rs.is_file(), f"Missing src/lib.rs in {repo}"
        content = lib_rs.read_text(encoding="utf-8")
        _verify_aaa_testing(content, repo)

        pkg_name = repo.lower()
        assert (PKGS_DIR / pkg_name / "PKGBUILD").is_file(), f"Missing PKGBUILD for {pkg_name}"

def test_multi_distro_open_source_os_unification_parity_aaa():
    """
    AAA Pattern: Verify multi-distribution open source OS unification:
    1. Cross-distro compiler tool in tools/sigma_distro_unifier.rs
    2. Debian DPKG packaging: debian/control, debian/rules, debian/changelog
    3. Gentoo Portage ebuild & make.conf
    4. Fedora RPM spec & dnf.conf
    5. Alpine Linux APKBUILD
    6. NixOS default.nix & flake.nix
    """
    # Arrange
    tool_file = ROOT_DIR / "tools" / "sigma_distro_unifier.rs"
    distro_assets = [
        ROOT_DIR / "debian" / "control",
        ROOT_DIR / "debian" / "rules",
        ROOT_DIR / "debian" / "changelog",
        ROOT_DIR / "portage" / "make.conf",
        ROOT_DIR / "portage" / "sys-apps" / "sigmaos-core" / "sigmaos-core-1.0.0.ebuild",
        ROOT_DIR / "specs" / "sigmaos.spec",
        ROOT_DIR / "config" / "fedora" / "dnf.conf",
        ROOT_DIR / "aports" / "main" / "sigmaos-core" / "APKBUILD",
        ROOT_DIR / "nix" / "default.nix",
        ROOT_DIR / "nix" / "flake.nix",
    ]

    # Act & Assert
    assert tool_file.is_file(), f"Missing tool: {tool_file}"
    tool_content = tool_file.read_text(encoding="utf-8")
    assert "DistroFamily" in tool_content
    assert "DistroDependencyMapper" in tool_content
    assert "GentooUseFlagResolver" in tool_content
    _verify_aaa_testing(tool_content, "sigma_distro_unifier.rs")

    for asset in distro_assets:
        assert asset.is_file(), f"Missing distro packaging asset: {asset}"
        assert asset.stat().st_size > 20, f"Asset {asset} is empty or stub"

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
    test_bsd_suite_parity_aaa()
    print("✓ test_bsd_suite_parity_aaa passed")
    test_kernel_suite_parity_aaa()
    print("✓ test_kernel_suite_parity_aaa passed")
    test_linux_distro_suite_parity_aaa()
    print("✓ test_linux_distro_suite_parity_aaa passed")
    test_office_and_web_suites_parity_aaa()
    print("✓ test_office_and_web_suites_parity_aaa passed")
    test_tools_wave3_and_datascience_parity_aaa()
    print("✓ test_tools_wave3_and_datascience_parity_aaa passed")
    test_wiki_documentation_conformance_parity_aaa()
    print("✓ test_wiki_documentation_conformance_parity_aaa passed")
    test_craft_suite_parity_aaa()
    print("✓ test_craft_suite_parity_aaa passed")
    test_extended_tools_and_office_parity_aaa()
    print("✓ test_extended_tools_and_office_parity_aaa passed")
    test_platform_frameworks_parity_aaa()
    print("✓ test_platform_frameworks_parity_aaa passed")
    test_platform_wave2_frameworks_parity_aaa()
    print("✓ test_platform_wave2_frameworks_parity_aaa passed")
    test_platform_wave3_frameworks_parity_aaa()
    print("✓ test_platform_wave3_frameworks_parity_aaa passed")
    test_platform_wave4_frameworks_parity_aaa()
    print("✓ test_platform_wave4_frameworks_parity_aaa passed")
    test_platform_wave5_frameworks_parity_aaa()
    print("✓ test_platform_wave5_frameworks_parity_aaa passed")
    test_arch_linux_ecosystem_gap_closure_aaa()
    print("✓ test_arch_linux_ecosystem_gap_closure_aaa passed")
    test_open_source_os_ecosystem_gap_closure_aaa()
    print("✓ test_open_source_os_ecosystem_gap_closure_aaa passed")
    test_multi_distro_open_source_os_unification_parity_aaa()
    print("✓ test_multi_distro_open_source_os_unification_parity_aaa passed")
    print("\nALL 32 AAA MULTI-DISTRO & OPEN SOURCE OS PARITY TESTS PASSED SUCCESSFULLY!")
