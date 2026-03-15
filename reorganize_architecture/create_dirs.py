# Generated file: create_dirs
import os
import shutil
from pathlib import Path

def create_dirs():
    print('[*] Creating OS Geometry...')
    dirs = ['bootloader', 'kernel', 'libc', 'userland', 'userland/system_api', 'web_os', 'apps']
    for d in dirs:
        (ROOT / d).mkdir(parents=True, exist_ok=True)