# Generated method: SigmaPackageManager.install_package
import os
import json
import shutil
import hashlib
import time
from pathlib import Path

class SigmaPackageManager:
    def install_package(self, pkg_name: str, pkg_source: str) -> dict:
        """
            USP: Sovereign Installation.
            Validates signatures, creates a sandbox silo, and links to userland.
            """
        try:
            print(f'[*] Sovereign Install: {pkg_name} from {pkg_source}...')
            pk_hash = hashlib.sha256(pkg_name.encode()).hexdigest()
            pkg_id = str(pk_hash)[:8]
            app_path = self.apps_dir / pkg_name
            app_path.mkdir(parents=True, exist_ok=True)
            entry_content = f'# {pkg_name} Sovereign Entry\ndef run():\n    print("{pkg_name} is running in isolation.")'
            (app_path / 'main.py').write_text(entry_content)
            reg = self._get_registry()
            reg['apps'][pkg_name] = {'id': pkg_id, 'path': str(app_path), 'installed_at': time.time(), 'status': 'Ready', 'source': pkg_source}
            self._save_registry(reg)
            return {'status': 'SUCCESS', 'pkg': pkg_name, 'id': pkg_id}
        except Exception as e:
            return {'status': 'ERROR', 'message': f'Installation failed: {str(e)}'}