# Generated file: update_imports
import os
import shutil
from pathlib import Path

def update_imports():
    print('[*] Refactoring Semantic Imports...')
    target_files = ['sigma.py', 'launcher.py', 'sigma_cli.py', 'sigma_gui.py']
    for m in target_files:
        p = ROOT / m
        if p.exists():
            content = p.read_text('utf-8', errors='ignore')
            content = content.replace('"kernel"', '"sigma_core"')
            content = content.replace("'kernel'", "'sigma_core'")
            p.write_text(content, 'utf-8')