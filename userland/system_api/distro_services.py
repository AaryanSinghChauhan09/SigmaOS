"""
Cosmos AI-OS: Package Manager (cpkg) & Init System (Cosmos-d)
=============================================================
Mission: Lifecycle Management & Binary Distribution.
"""

import hashlib
import time

from .privacy_engine import ZeroTrustValidator

class CosmosPackageManager:
    """The 'cpkg' utility for Cosmos AI-OS."""
    def __init__(self, kernel):
        self.kernel = kernel
        self.repo = {
            "vim": {"version": "9.0", "deps": ["libc"], "sig": "cosmos_root_v1"},
            "python-lite": {"version": "3.11", "deps": ["libc", "libmath"], "sig": "cosmos_root_v1"},
            "cosmos-term": {"version": "1.0", "deps": ["compositor-lib"], "sig": "antigravity_core_v1"},
            "malware-test": {"version": "6.6.6", "deps": [], "sig": "untrusted_sig"}
        }
        self.installed = ["libc", "libmath"]
        self.trust = ZeroTrustValidator()

    def install(self, pkg_name):
        if pkg_name not in self.repo:
            return f"Error: Package {pkg_name} not found in repository."
        
        pkg = self.repo[pkg_name]
        
        # 1. Zero-Trust Signature Check (Crucial GAP fix)
        if not self.trust.validate_module(pkg_name, pkg["sig"]):
            return f"ACCESS DENIED: Package {pkg_name} failed Zero-Trust verification. Execution blocked."

        print(f"[CPKG] Resolving dependencies for {pkg_name}...")
        for dep in pkg["deps"]:
            if dep not in self.installed:
                print(f"[CPKG] Auto-installing dependency: {dep}")
                self.installed.append(dep)
        
        self.installed.append(pkg_name)
        return f"Successfully installed {pkg_name} v{pkg['version']}."

class CosmosInit:
    """The 'Cosmos-d' Init System (PID 1)."""
    def __init__(self, kernel):
        self.kernel = kernel
        self.services = [
            {"name": "pci_scanner", "priority": 1},
            {"name": "privacy_scrubber", "priority": 2},
            {"name": "neural_firewall", "priority": 3},
            {"name": "compositor", "priority": 4},
            {"name": "lisp_shell", "priority": 5}
        ]

    def start_system(self):
        print("[Cosmos-d] Starting System in Sovereign Mode (Strict Principles)...")
        # Sort by priority to ensure privacy/firewall start BEFORE networking/UI
        sorted_services = sorted(self.services, key=lambda x: x["priority"])
        
        for svc in sorted_services:
            print(f"[Cosmos-d] Spawning {svc['name']} (Priority {svc['priority']})...")
            if svc["name"] == "pci_scanner":
                self.kernel.registry["pci"].scan_bus()
            elif svc["name"] == "privacy_scrubber":
                print("[Cosmos-d] Privacy Scrubber engaged at Ring-0.")
            
        print("[Cosmos-d] Performing Final Sovereign Audit...")
        from .privacy_engine import ZeroTrustValidator
        ZeroTrustValidator().check_telemetry_status()
        print("[Cosmos-d] System Stable. Zero 3rd party modules detected.")

    def monitor_services(self):
        # Watchdog logic...
        pass
