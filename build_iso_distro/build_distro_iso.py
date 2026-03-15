# Generated file: build_distro_iso
import os
import shutil
import json
from pathlib import Path

def build_distro_iso():
    print('[*] Initiating SigmaOS Sovereign Distro Build...')
    dist_img = ROOT / 'SOVEREIGN_DISTRO_IMG'
    if dist_img.exists():
        try:
            shutil.rmtree(dist_img)
        except Exception as e:
            print(f'[!] Cleanup failed (likely open file): {e}')
    dirs = ['BOOT', 'KERNEL', 'LIBC', 'USERLAND/API', 'USERLAND/UI', 'USERLAND/APPS', 'RECOVERY']
    for d in dirs:
        (dist_img / d).mkdir(parents=True, exist_ok=True)
    for src_dir, dest_dir in [('sigma_core', 'KERNEL'), ('bootloader', 'BOOT'), ('libc', 'LIBC')]:
        src = ROOT / src_dir
        if src.exists():
            for f in src.iterdir():
                if f.is_file():
                    shutil.copy2(f, dist_img / dest_dir / f.name)
    mapping = [('userland/system_api', 'USERLAND/API'), ('web_os', 'USERLAND/UI'), ('apps', 'USERLAND/APPS')]
    for src_sub, dest_sub in mapping:
        src = ROOT / src_sub
        if src.exists():
            for f in src.iterdir():
                if f.is_file():
                    shutil.copy2(f, dist_img / dest_sub / f.name)
    top_files = ['boot.py', 'sigma_setup.py', 'sigma_gui.py', 'sigma_cli.py', 'sigma_core', 'ecosystem', 'requirements.txt', 'README.md']
    for item in top_files:
        src = ROOT / item
        if src.exists():
            if src.is_dir():
                shutil.copytree(src, dist_img / item, dirs_exist_ok=True)
            else:
                shutil.copy2(src, dist_img / item)
    manifest = {'OS_NAME': 'SigmaOS Sovereign', 'VERSION': '2.0.0-APEX', 'ARCHITECTURE': 'Universal Python-Orchestrated x86/ARM', 'BOOT_ENTRY': 'boot.py', 'SECURITY_GRADE': 'TITAN_ZERO_TRUST'}
    with open(dist_img / 'BOOT' / 'manifest.json', 'w') as f:
        json.dump(manifest, f, indent=4)
    print(f'DONE - SIGMAOS ISO DISTRO ASSEMBLED AT: {dist_img}')