"""
Sigma Sovereign ISO Distro Assembler
====================================
USP: This module assembles the 'Sovereign Distro' which mimics the 
     structure of a professional OS ISO (Windows/Linnux).
     It creates the folders and manifests required for a 'Zero-Interference' boot.
"""

import os
import json

class SigmaISOAssembler:
    def __init__(self, target_dir):
        self.target_dir = target_dir
        self.iso_root = os.path.join(self.target_dir, "SOVEREIGN_DISTRO_IMG")

    def assemble_distro(self):
        """Creates the ISO-like file structure."""
        print(f"🚀 ASSEMBLING SIGMAOS SOVEREIGN DISTRO: {self.iso_root}")
        
        # 1. Create Folders
        folders = [
            "BOOT",          # Bootloader manifests
            "KERNEL",        # Sigma Core 
            "APPS",          # Browser, Data Matrix, Forge
            "DRIVERS",       # VBox Guest Additions bridge
            "SYSTEM_RECOVERY" 
        ]
        
        for f in folders:
            os.makedirs(os.path.join(self.iso_root, f), exist_ok=True)

        # 2. Generate Boot Manifest
        manifest = {
            "OS_NAME": "SigmaOS Sovereign",
            "VERSION": "2.0.0",
            "ARCH": "x64",
            "BOOT_ENTRY": "sigma_direct_boot.py",
            "NATIVE_INJECTOR": "SET_AS_NATIVE_BOOT.bat",
            "LICENSE": "Zero-Trust Sovereign License"
        }
        
        with open(os.path.join(self.iso_root, "BOOT", "manifest.json"), 'w') as f:
            json.dump(manifest, f, indent=4)

        # 3. Create 'OS_README' for the Virtual DVD
        with open(os.path.join(self.iso_root, "SOVEREIGN_README.txt"), 'w') as f:
            f.write("============================================================\n")
            f.write(" SIGMA OS SOVEREIGN v2.0 - MASTER DISTRIBUTION IMAGE\n")
            f.write("============================================================\n\n")
            f.write("This directory contains the full Sovereign OS Distribution.\n")
            f.write("To run this inside VirtualBox as a 'Zero-Interference' system:\n\n")
            f.write("1. Mount this folder as a Shared Folder in VirtualBox.\n")
            f.write("2. Boot your Foundation Silo.\n")
            f.write("3. Run 'SET_AS_NATIVE_BOOT.bat' from the root of this disk.\n\n")
            f.write("Sovereignty is yours.")

if __name__ == "__main__":
    root = "C:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS"
    assembler = SigmaISOAssembler(root)
    assembler.assemble_distro()
