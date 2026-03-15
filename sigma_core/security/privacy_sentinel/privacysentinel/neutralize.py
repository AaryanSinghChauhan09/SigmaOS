# Generated method: PrivacySentinel.neutralize
import os
import re

class PrivacySentinel:
    @staticmethod
    def neutralize(fpath, replacement='[REDACTED]'):
        """Neutralize PII in a file."""
        try:
            with open(fpath, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
            new_content = content
            for pattern in PrivacySentinel.BLACKLIST:
                new_content = re.sub(pattern, replacement, new_content)
            if new_content != content:
                with open(fpath, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                return True
        except:
            pass
        return False