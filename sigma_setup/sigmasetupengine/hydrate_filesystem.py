# Generated method: SigmaSetupEngine.hydrate_filesystem
import os
import sys
import platform
import shutil
import time
import subprocess
from pathlib import Path

class SigmaSetupEngine:
    def hydrate_filesystem(self):
        """USP: Sovereign Partitioning. Ensures the SigmaFS structure exists."""
        print('[*] Hydrating SigmaFS Structure and Scripts...')
        dirs = ['sigma_core/hal', 'sigma_core/memory', 'sigma_core/scheduler', 'userland/system_api', 'web_os/pages', 'ecosystem/apps', 'assets/themes', 'releases/stable', 'logs/kernel']
        for d in dirs:
            path = self.root / d
            path.mkdir(parents=True, exist_ok=True)
        if platform.system() != 'Windows':
            for script in self.root.glob('*.sh'):
                try:
                    os.chmod(script, 493)
                    print(f'    -> Perms Set: {script.name}')
                except:
                    pass
        print('[✓] SigmaFS Geometry Initialized.')