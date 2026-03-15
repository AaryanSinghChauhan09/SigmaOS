# Generated file: sanitize_file
import os
import sys
from pathlib import Path

def sanitize_file(file_path):
    try:
        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
        modified = content
        for find_str, replace_str in REPLACEMENTS.items():
            if find_str in modified:
                modified = modified.replace(find_str, replace_str)
        if modified != content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(modified)
            print(f'[RECOVER] Sanitized: {os.path.basename(file_path)}')
            return True
    except Exception as e:
        pass
    return False