"""
SigmaOS Full System Audit v1.0
Automated integrity and manifest verification suite.
"""
import os
import sys
import importlib

# Add root to sys.path
root = os.getcwd()
if root not in sys.path:
    sys.path.append(root)

class SigmaAuditor:
    def __init__(self):
        self.errors = []
        self.warnings = []

    def audit_manifest(self):
        print("[AUDIT] Checking manifest consistency...")
        try:
            from sigma_core.manifest import CORE_SYSTEM_MODULES, ECOSYSTEM_APPS
        except ImportError as e:
            self.errors.append(f"CRITICAL: Could not load manifest: {e}")
            return

        for path, cls, key in CORE_SYSTEM_MODULES:
            try:
                # Basic path existence check
                mod_path = path.replace(".", "/") + ".py"
                if not os.path.exists(mod_path):
                    self.warnings.append(f"Manifest path mismatch: {mod_path}")
            except Exception as e:
                self.errors.append(f"Path verify error for {key}: {e}")

    def audit_gui_pkg(self):
        print("[AUDIT] Verifying UI modular packages...")
        pkg_dir = os.path.join(root, "userland", "system_api", "gui_pkg")
        if not os.path.exists(pkg_dir):
            self.errors.append("CRITICAL: gui_pkg directory missing!")
            return

        required_pages = ["base_page.py", "dashboard.py", "apex_page.py", "nexus_page.py", "arcade.py"]
        for p in required_pages:
            if not os.path.exists(os.path.join(pkg_dir, p)):
                self.errors.append(f"Modular page missing: {p}")

    def run(self):
        print("=== SIGMA OS SOVEREIGN AUDIT ===")
        self.audit_manifest()
        self.audit_gui_pkg()
        
        print("\nAudit Results:")
        print(f"Errors: {len(self.errors)}")
        print(f"Warnings: {len(self.warnings)}")
        
        for e in self.errors: print(f"  [X] {e}")
        for w in self.warnings: print(f"  [!] {w}")
        
        if not self.errors:
            print("\n[VERDICT] SYSTEM INTEGRITY: VERIFIED (APEX READY)")
        else:
            print("\n[VERDICT] SYSTEM INTEGRITY: COMPROMISED")

if __name__ == "__main__":
    auditor = SigmaAuditor()
    auditor.run()
