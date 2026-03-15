import os
import re
from pathlib import Path

class SigmaScrubber:
    """
    Forensic Identity Scrubbing Engine for SigmaOS.
    Ensures no personal paths (C:/Users/Aaryan) or dev-keys leak to GitHub.
    """
    def __init__(self):
        self.root = Path(os.getcwd())
        # Patterns to scrub
        self.sensitive_patterns = [
            (r"C:[\\/]Users[\\/][a-zA-Z0-9\-_]+", "C:/Users/Aaryan"),
            (r"c:[\\/]Users[\\/][a-zA-Z0-9\-_]+", "C:/Users/Aaryan"),
            (r"Sovereign-User", "Sovereign-User"), # Keep this for generic identity
            (r"api_key\s*=\s*['\"][a-zA-Z0-9_\-]+['\"]", "api_key = 'REDACTED_BY_SOVEREIGN'"),
        ]

    def scrub_file(self, file_path):
        try:
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
            
            new_content = content
            for pattern, replacement in self.sensitive_patterns:
                new_content = re.sub(pattern, replacement, new_content)
            
            if new_content != content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                print(f"[SCRUBBED] {file_path}")
                return True
        except Exception as e:
            print(f"[ERROR] Could not scrub {file_path}: {e}")
        return False

    def run(self):
        print("--- SIGMA IDENTITY SCRUB BATTLE-READY ---")
        scrubbed_count = 0
        for root, dirs, files in os.walk(self.root):
            if ".git" in root or "__pycache__" in root or "evidence_vault" in root:
                continue
            for f in files:
                if f.endswith(('.py', '.json', '.sh', '.ps1', '.txt', '.md')):
                    if self.scrub_file(os.path.join(root, f)):
                        scrubbed_count += 1
                        
        # USP: Bit-Shredding of temp logs (Competitor Absorption: Forensic wiping)
        from userland.system_api.sigma_std import SigmaCrypto
        for tmp_f in ["_errors.txt", "_errors2.txt", "system_audit.sigma.tmp"]:
            if os.path.exists(tmp_f):
                SigmaCrypto.secure_shred(tmp_f)
                print(f"[SHREDDED] Trace file: {tmp_f}")

        print(f"--- SCRUB COMPLETE. {scrubbed_count} files sanitized. ---")

def scrub_all():
    """Global entry point for direct module execution."""
    scrubber = SigmaScrubber()
    scrubber.run()

if __name__ == "__main__":
    scrub_all()
