# Generated method: PrivacySentinel.audit_file
import os
import re

class PrivacySentinel:
    @staticmethod
    def audit_file(fpath):
        leaks = []
        try:
            with open(fpath, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
                for pattern in PrivacySentinel.BLACKLIST:
                    matches = re.findall(pattern, content)
                    if matches:
                        leaks.extend(list(set(matches)))
        except:
            pass
        return leaks