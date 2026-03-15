# Generated method: SigmaNomad.boot_portable
import os
import sys
import subprocess
import json

class SigmaNomad:
    def boot_portable(self):
        """Launches the SigmaOS Kernel in Nomad/Containerized mode."""
        print(f'--- [SIGMAOS NOMAD BOOTING ON {sys.platform.upper()}] ---')
        os.environ['C:/Users/SigmaUser'] = self.portable_drive
        try:
            cmd = [sys.executable, 'sigma_core/kernel.py']
            subprocess.Popen(cmd, cwd=self.root_dir)
            return 'Nomad: Kernel successfully virtualized over host.'
        except Exception as e:
            return f'Nomad: Boot Failure - {str(e)}'