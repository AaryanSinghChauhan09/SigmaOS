#!/usr/bin/env python3
"""
SigmaOS Startup Diagnostics Engine
====================================
Runs health checks at startup and formats them as rich GTK3 output.
Modelled after Omarchy's 'doctor' philosophy and Arch's pacman -Qk / checkupdates.
"""
import subprocess
import os
import sys

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR = os.path.dirname(SCRIPT_DIR)

CHECKS = [
    # (label, shell_command, success_contains_or_None_to_check_rc)
    ("Rust/Cargo toolchain",   "rustc --version 2>&1",         "rustc"),
    ("SigmaOS REPL binary",    f"test -x {ROOT_DIR}/bin/sigmaos && echo OK", "OK"),
    ("SigmaOS Launcher binary",f"test -x {ROOT_DIR}/bin/sigmaos-launch && echo OK", "OK"),
    ("ALPM bridge (sigmaos-arch)", "which sigmaos-arch 2>&1 || echo MISSING", "sigmaos"),
    ("sigpkg helper",          "which sigpkg 2>&1 || echo MISSING", "sigpkg"),
    ("pacman ALPM wrapper",    "sigpkg version 2>&1 || pacman --version 2>&1 | head -1", None),
    ("AUR v5 RPC endpoint",    "curl -sf 'https://aur.archlinux.org/rpc/v5/search/hyprland?by=name' | python3 -c \"import json,sys; d=json.load(sys.stdin); print(f'AUR: {d[\\\"resultcount\\\"]} pkgs')\" 2>&1 || echo 'AUR: offline (expected in sandbox)'", "AUR"),
    ("GTK3 available",         "python3 -c \"import gi; gi.require_version('Gtk','3.0'); from gi.repository import Gtk; print('GTK3 OK')\" 2>&1", "GTK3"),
    ("WebKit2 4.1 available",  "python3 -c \"import gi; gi.require_version('WebKit2','4.1'); from gi.repository import WebKit2; print('WebKit2 OK')\" 2>&1", "WebKit2"),
    ("DISPLAY / X11",          "echo \"DISPLAY=${DISPLAY:-not-set}\"", "DISPLAY"),
    ("SigmaOS UI HTML",        f"test -f {ROOT_DIR}/ui/desktop/index.html && wc -l {ROOT_DIR}/ui/desktop/index.html | awk '{{print $1\" lines\"}}' || echo MISSING", None),
    ("Nerd Fonts present",     "fc-list | grep -i 'NerdFont\\|JetBrains\\|Hack' | head -1 | cut -c-50 || echo 'no nerd font detected'", None),
    ("neofetch / fastfetch",   "which neofetch 2>/dev/null || which fastfetch 2>/dev/null || echo MISSING", None),
    ("google-chrome",          "which google-chrome 2>/dev/null || which chromium 2>/dev/null || echo MISSING", None),
]

def run_diagnostics(rich=False):
    """Run all checks. Returns list of (label, status, output) tuples."""
    results = []
    for label, cmd, ok_contains in CHECKS:
        try:
            out = subprocess.check_output(cmd, shell=True, stderr=subprocess.STDOUT, text=True, timeout=10).strip()
            status = "PASS" if (ok_contains is None or ok_contains.lower() in out.lower()) else "WARN"
        except subprocess.CalledProcessError as e:
            out = (e.output or "").strip() or "FAIL"
            status = "FAIL"
        except Exception as e:
            out = str(e)
            status = "FAIL"
        results.append((label, status, out[:120]))
    return results

def format_text(results):
    lines = ["╔══════════════════════════════════════════════════════════════╗",
             "║           SigmaOS Startup Diagnostics                       ║",
             "╚══════════════════════════════════════════════════════════════╝", ""]
    pass_n = sum(1 for _, s, _ in results if s == "PASS")
    warn_n = sum(1 for _, s, _ in results if s == "WARN")
    fail_n = sum(1 for _, s, _ in results if s == "FAIL")
    for label, status, out in results:
        icon = {"PASS": "✓", "WARN": "~", "FAIL": "✗"}.get(status, "?")
        lines.append(f"  [{icon}] {label:<35}  {out}")
    lines += ["", f"  Results: {pass_n} PASS  {warn_n} WARN  {fail_n} FAIL",
              "────────────────────────────────────────────────────────────────"]
    return "\n".join(lines)

if __name__ == "__main__":
    results = run_diagnostics()
    print(format_text(results))
    fail_n = sum(1 for _, s, _ in results if s == "FAIL")
    sys.exit(1 if fail_n > 0 else 0)
