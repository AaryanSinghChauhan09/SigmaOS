# Generated file: move_files
import os
import shutil
from pathlib import Path

def move_files():
    print('[*] Reorganizing Core Components...')
    c_core = ROOT / 'kernel' / 'c_core'
    if c_core.exists():
        pass
    old_apps = ROOT / 'apps'
    new_apps = ROOT / 'userland' / 'apps'
    if old_apps.exists() and old_apps != new_apps:
        for f in old_apps.iterdir():
            if not (new_apps / f.name).exists():
                shutil.move(str(f), str(new_apps / f.name))
        print(f'[✓] Migrated apps to {new_apps}')