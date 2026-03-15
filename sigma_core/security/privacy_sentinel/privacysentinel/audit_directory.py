# Generated method: PrivacySentinel.audit_directory
import os
import re

class PrivacySentinel:
    @staticmethod
    def audit_directory(path='.'):
        """Scan directory for PII leaks."""
        leaks = []
        for root, _, files in os.walk(path):
            if '.git' in root or '.antigravity' in root:
                continue
            for file in files:
                if file.endswith(('.py', '.md', '.txt', '.json')):
                    fpath = os.path.join(root, file)
                    found = PrivacySentinel.audit_file(fpath)
                    if found:
                        leaks.append((fpath, found))
        return leaks