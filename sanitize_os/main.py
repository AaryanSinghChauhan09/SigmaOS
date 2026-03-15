# Generated file: main
import os
import sys
from pathlib import Path

def main():
    print(f'[*] Starting OS Sanitization loop in {SIGMA_ROOT}...')
    count = 0
    for root, dirs, files in os.walk(SIGMA_ROOT):
        if any((x in root for x in ('.git', '__pycache__', '.pytest_cache'))):
            continue
        for file in files:
            if file.endswith(('.py', '.md', '.txt', '.ps1', '.bat', '.json', '.xml')):
                if sanitize_file(os.path.join(root, file)):
                    count += 1
    print(f'DONE - Sanitization complete. {count} files processed.')