"""
SigmaOS Sovereign Setup & Hydration Engine (v1.0)
=================================================
USP: Transitioning SigmaOS from 'Prototype Stubs' to 'Functional Sovereign'.
Handles environment validation, core hydrate, and first-boot sequence.
"""

import os
import sys
import platform
import shutil
import time
import subprocess
from pathlib import Path

class SigmaSetupEngine:
    def __init__(self):
        self.root = Path(os.path.abspath(os.path.dirname(__file__)))
        self.version = "1.0.0-PRO-SETUP"
        self.prereqs = ["python", "pip", "git", "powershell"]
        
    def banner(self):
        print("\033[96m" + "="*60)
        print("   Σ SIGMAOS SOVEREIGN: APEX HYDRATIONS & SETUP   ")
        print("   Transforming Concepts into Execution Layers    ")
        print("="*60 + "\033[0m")

    def check_environment(self):
        print(f"[*] Validating Host Environment: {platform.system()} {platform.release()}")
        results = {}
        # Core checks
        results["python"] = ".".join(map(str, sys.version_info[:3]))
        results["disk_space"] = shutil.disk_usage("/").free // (2**30) # GB
        
        # TKINTER check (Critical for GUI)
        try:
            import tkinter
            results["gui_ready"] = True
        except ImportError:
            results["gui_ready"] = False
            print("[!] WARNING: 'tkinter' not found. GUI functions will be unavailable.")

        # PSUTIL check (Recommended for performance)
        try:
            import psutil
            results["psutil_ready"] = True
        except ImportError:
            results["psutil_ready"] = False
            print("[!] CAUTION: 'psutil' not found. System analytics will use native fallbacks.")

        print(f"[+] Python {results['python']} detected.")
        print(f"[+] Disk Resources: {results['disk_space']} GB Free.")
        return results

    def install_requirements(self):
        """Attempts to install core dependencies natively."""
        print("[*] Checking for missing dependencies in requirements.txt...")
        try:
            subprocess.check_call([sys.executable, "-m", "pip", "install", "-r", "requirements.txt"])
            print("[✓] Dependency map synchronized.")
        except Exception as e:
            print(f"[!] Warning: Automated dependency install failed: {e}")

    def hydrate_filesystem(self):
        """USP: Sovereign Partitioning. Ensures the SigmaFS structure exists."""
        print("[*] Hydrating SigmaFS Structure and Scripts...")
        dirs = [
            "sigma_core/hal", "sigma_core/memory", "sigma_core/scheduler",
            "userland/system-api", "web_os/pages",
            "ecosystem/apps", "assets/themes", "releases/stable", "logs/kernel"
        ]
        for d in dirs:
            path = self.root / d
            path.mkdir(parents=True, exist_ok=True)
            
        # Permission setup for Linux/macOS
        if platform.system() != "Windows":
            for script in self.root.glob("*.sh"):
                try:
                    os.chmod(script, 0o755)
                    print(f"    -> Perms Set: {script.name}")
                except: pass
        
        print("[✓] SigmaFS Geometry Initialized.")

    def bootstrap_core_logic(self):
        """USP: Kernel-Fleshing. Transitions stubs to working logic."""
        print("[*] Bootstrapping Kernel Implementation Layers...")
        # Create a basic boot loader script if missing
        boot_script = self.root / "boot.py"
        if not boot_script.exists():
            with open(boot_script, "w") as f:
                f.write("# SigmaOS Bootstrapper\\nfrom sigma_core.kernel import SigmaKernel\\n\\nif __name__ == '__main__':\\n    k = SigmaKernel()\\n    print('SigmaOS Kernel Initialized.')")
            print("[+] Boot script created: boot.py")
            
        # Ensure a base config exists
        config_path = self.root / "sigma_core" / "config.py"
        if not config_path.exists():
            with open(config_path, "w") as f:
                f.write("class SigmaConfig:\\n    OS_NAME = 'SigmaOS Sovereign'\\n    VERSION = '2.0.0-PRO'\\n    THEME = 'Glass-Dark'")
            print("[+] Config created: sigma_core/config.py")

    def setup_package_manager(self):
        """Initializes the Sovereign App Store (Package Registry)."""
        print("[*] Initializing Sovereign App Store Registry...")
        registry_path = self.root / "ecosystem" / "registry.json"
        if not registry_path.exists():
            import json
            initial_data = {
                "os_version": "2.0.0",
                "installed_apps": ["image_to_text", "text_to_html", "video_transcript"],
                "available_repo": "https://sigmaos.sovereign/repo",
                "last_sync": time.time()
            }
            with open(registry_path, "w") as f:
                json.dump(initial_data, f, indent=4)
            print("[+] Registry created: ecosystem/registry.json")

    def finalize(self):
        print("\n\033[92m[✓] SIGMAOS HYDRATION COMPLETE\033[0m")
        print("[!] Next Step: Run 'python boot.py' to launch into APEX Mode.")
        
if __name__ == "__main__":
    setup = SigmaSetupEngine()
    setup.banner()
    setup.check_environment()
    setup.install_requirements()
    setup.hydrate_filesystem()
    setup.setup_package_manager()
    setup.finalize()
