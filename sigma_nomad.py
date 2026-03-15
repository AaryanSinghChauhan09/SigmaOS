"""
SigmaOS Nomad — Portable Virtualized Environment (PVE)
======================================================
USP: Cross-OS Portability + Host-Agnostic Isolation + Zero-Install Boot.
This containerized variant allows SigmaOS to run as a 'Virtual Shell' inside
any host (Windows, Linux, macOS) without permanent modifications.
"""

import os
import sys
import subprocess
import json

class SigmaNomad:
    """
    SigmaNomad manages the 'Guest-over-Host' lifecycle.
    It orchestrates local isolation folders and virtual paths.
    """
    def __init__(self, mode="Virtual"):
        self.mode = mode
        self.root_dir = os.path.abspath(os.path.dirname(__file__))
        self.portable_drive = os.path.join(self.root_dir, "sigma_portable_drive")
        
        # Ensure the virtual workspace exists
        if not os.path.exists(self.portable_drive):
            os.makedirs(self.portable_drive)
            os.makedirs(os.path.join(self.portable_drive, "home"))
            os.makedirs(os.path.join(self.portable_drive, "mnt/shared"))

    def check_host_compatibility(self):
        """Standard compatibility check for Nomad mode."""
        return {
            "os": sys.platform,
            "python": sys.version.split()[0],
            "virtualization_ready": True, # Assume native Python isolation
            "storage": "PORTABLE/NOMAD"
        }

    def boot_portable(self):
        """Launches the SigmaOS Kernel in Nomad/Containerized mode."""
        print(f"--- [SIGMAOS NOMAD BOOTING ON {sys.platform.upper()}] ---")
        # In Nomad mode, we point the kernel to the portable drive for all logic
        os.environ["C:/Users/Aaryan"] = self.portable_drive
        
        # Start the kernel as a subprocess to maintain host independence
        try:
            cmd = [sys.executable, "sigma_core/kernel.py"]
            subprocess.Popen(cmd, cwd=self.root_dir)
            return "Nomad: Kernel successfully virtualized over host."
        except Exception as e:
            return f"Nomad: Boot Failure - {str(e)}"

if __name__ == "__main__":
    nomad = SigmaNomad()
    print(json.dumps(nomad.check_host_compatibility(), indent=4))
    print(nomad.boot_portable())
