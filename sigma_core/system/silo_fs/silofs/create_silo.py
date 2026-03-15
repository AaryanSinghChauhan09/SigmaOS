# Generated method: SiloFS.create_silo
import os
import shutil
from typing import List, Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SiloFS:
    def create_silo(self, app_name: str, base_dir: str) -> str:
        """USP: Create a layered sandbox for an untrusted application."""
        silo_id = f'silo_{app_name}_{os.urandom(4).hex()}'
        silo_path = os.path.join(self.sandbox_root, silo_id)
        os.makedirs(silo_path)
        with open(os.path.join(silo_path, 'silo_manifest.json'), 'w') as f:
            f.write(f'{{"app": "{app_name}", "base": "{base_dir}", "mode": "EPHEMERAL"}}')
        self.active_silos[silo_id] = {'app': app_name, 'path': silo_path}
        self.log_event('silo_created', {'id': silo_id, 'app': app_name})
        return silo_id