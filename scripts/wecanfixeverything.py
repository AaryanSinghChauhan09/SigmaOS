#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""
wecanfixeverything.py - Sovereign Self-Healing Diagnosis & Compatibility Repair Engine
Inspired by omacom/wecanfixeverything ("We can fix everything: every missing app, every incompatibility, every paper cut.")
"""

import os
import sys
import shutil
import subprocess
from pathlib import Path

HOME = Path.home()
CONFIG_DIR = HOME / ".config"
LOCAL_BIN = HOME / ".local" / "bin"
SIGMA_ROOT = Path(__file__).resolve().parent.parent

class SystemDoctor:
    def __init__(self):
        self.issues = []
        self.fixes_applied = []

    def check_pacman_lock(self):
        lock_file = Path("/var/lib/pacman/db.lck")
        if lock_file.exists():
            self.issues.append(("PACMAN_LOCK", "Stale pacman database lock file exists", str(lock_file)))
        else:
            print("  [✓] Pacman database lock: Clean")

    def check_bin_symlinks(self):
        missing_bins = []
        expected_bins = ["omarchy-shell", "omarchy"]
        for b in expected_bins:
            p = LOCAL_BIN / b
            if not p.exists():
                missing_bins.append(b)
        if missing_bins:
            self.issues.append(("MISSING_BINS", f"Missing binaries in ~/.local/bin: {missing_bins}", missing_bins))
        else:
            print("  [✓] Local binaries in ~/.local/bin: OK")

    def check_dotfiles(self):
        missing_configs = []
        expected_dirs = ["hypr", "waybar", "wallust", "foot", "alacritty", "kitty", "rofi", "mako", "nvim"]
        for d in expected_dirs:
            p = CONFIG_DIR / d
            if not p.is_dir():
                missing_configs.append(d)
        if missing_configs:
            self.issues.append(("MISSING_CONFIGS", f"Missing config directories: {missing_configs}", missing_configs))
        else:
            print("  [✓] Omarchy dotfiles in ~/.config: Complete")

    def check_audio_subsystem(self):
        # Verify pipewire or pulseaudio binary
        if not shutil.which("pipewire") and not shutil.which("pulseaudio"):
            self.issues.append(("AUDIO_DAEMON", "Neither pipewire nor pulseaudio binary found in PATH", None))
        else:
            print("  [✓] Audio subsystem (Pipewire/Pulse): Available")

    def check_theme_presets(self):
        preset_dir = SIGMA_ROOT / "config" / "omarchy" / "wallust" / "presets"
        if not preset_dir.is_dir() or len(list(preset_dir.glob("*.json"))) < 5:
            self.issues.append(("THEME_PRESETS", "Wallust theme presets are incomplete", str(preset_dir)))
        else:
            print("  [✓] Wallust 16-color theme presets: Complete (5+ loaded)")

    def diagnose(self):
        print("============================================================")
        print("  SigmaOS / Omarchy Diagnosis: 'We Can Fix Everything'")
        print("============================================================")
        self.check_pacman_lock()
        self.check_bin_symlinks()
        self.check_dotfiles()
        self.check_audio_subsystem()
        self.check_theme_presets()

        if not self.issues:
            print("\n🎉 ALL CHECKS PASSED: 0 ISSUES DETECTED. SYSTEM 100% HEALTHY.")
            return True
        else:
            print(f"\n⚠️  {len(self.issues)} issue(s) detected:")
            for code, msg, _ in self.issues:
                print(f"  • [{code}] {msg}")
            return False

    def fix(self):
        print("\n==> Initiating automated self-healing repairs...")
        for code, msg, data in self.issues:
            if code == "PACMAN_LOCK":
                try:
                    Path(data).unlink(missing_ok=True)
                    self.fixes_applied.append(f"Removed stale lock {data}")
                except Exception as e:
                    print(f"  [-] Failed to remove lock: {e}")
            elif code == "MISSING_BINS":
                src_bin = SIGMA_ROOT / "bin" / "omarchy-shell"
                if src_bin.is_file():
                    LOCAL_BIN.mkdir(parents=True, exist_ok=True)
                    target_bin = LOCAL_BIN / "omarchy-shell"
                    shutil.copy2(src_bin, target_bin)
                    target_bin.chmod(0o755)
                    sym = LOCAL_BIN / "omarchy"
                    sym.unlink(missing_ok=True)
                    sym.symlink_to(target_bin)
                    self.fixes_applied.append("Restored ~/.local/bin/omarchy-shell and omarchy symlink")
            elif code == "MISSING_CONFIGS":
                deploy_script = SIGMA_ROOT / "scripts" / "setup_omarchy_dotfiles.sh"
                if deploy_script.is_file():
                    subprocess.run(["bash", str(deploy_script)], stdout=subprocess.DEVNULL)
                    self.fixes_applied.append("Re-deployed Omarchy dotfiles via setup script")

        print("==> Self-healing completed.")
        for f in self.fixes_applied:
            print(f"  [✓] {f}")

def main():
    doctor = SystemDoctor()
    healthy = doctor.diagnose()
    if "--fix" in sys.argv and not healthy:
        doctor.fix()
        print("\nRe-verifying system health:")
        doctor.issues.clear()
        doctor.diagnose()

if __name__ == "__main__":
    main()
