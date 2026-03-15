# Generated method: SigmaScrubber.scrub_file
import os
import re
from pathlib import Path

class SigmaScrubber:
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
                print(f'[SCRUBBED] {file_path}')
                return True
        except Exception as e:
            print(f'[ERROR] Could not scrub {file_path}: {e}')
        return False