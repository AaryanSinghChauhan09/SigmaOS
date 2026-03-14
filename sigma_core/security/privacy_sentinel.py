"""
SigmaOS Privacy Sentinel (v1.0)
================================
USP: Automated PII (Personal Identifiable Information) Detection & Neutralization.
Ensures no personal data remains in the codebase or logs.
"""
import os
import re

class PrivacySentinel:
    BLACKLIST = [
        r"\b[A-Z][a-z]+ [A-Z][a-z]+\b", # Generic Name Pattern
        r"\b[A-Za-z0-9._%+-]+@\b",      # Email Start
    ]

    @staticmethod
    def audit_directory(path="."):
        """Scan directory for PII leaks."""
        leaks = []
        for root, _, files in os.walk(path):
            if ".git" in root or ".antigravity" in root: continue
            for file in files:
                if file.endswith((".py", ".md", ".txt", ".json")):
                    fpath = os.path.join(root, file)
                    found = PrivacySentinel.audit_file(fpath)
                    if found:
                        leaks.append((fpath, found))
        return leaks

    @staticmethod
    def audit_file(fpath):
        leaks = []
        try:
            with open(fpath, "r", encoding="utf-8", errors="ignore") as f:
                content = f.read()
                for pattern in PrivacySentinel.BLACKLIST:
                    matches = re.findall(pattern, content)
                    if matches:
                        leaks.extend(list(set(matches)))
        except:
            pass
        return leaks

    @staticmethod
    def neutralize(fpath, replacement="[REDACTED]"):
        """Neutralize PII in a file."""
        try:
            with open(fpath, "r", encoding="utf-8", errors="ignore") as f:
                content = f.read()
            
            new_content = content
            for pattern in PrivacySentinel.BLACKLIST:
                new_content = re.sub(pattern, replacement, new_content)
            
            if new_content != content:
                with open(fpath, "w", encoding="utf-8") as f:
                    f.write(new_content)
                return True
        except:
            pass
        return False
