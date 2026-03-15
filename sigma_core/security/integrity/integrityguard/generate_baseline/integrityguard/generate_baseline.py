# Generated method: IntegrityGuard.generate_baseline
import hashlib
import os
import sys
import json
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase

class IntegrityGuard:
    def generate_baseline(self, directories: List[str], create_backups: bool=True):
        """Generates a signed manifest of the current OS state and optional backups."""
        manifest = {}
        import shutil
        for directory in directories:
            abs_dir = os.path.abspath(directory)
            if not os.path.exists(abs_dir):
                continue
            for root, _, files in os.walk(abs_dir):
                for file in files:
                    if file.endswith(('.py', '.ps1', '.sh', '.json')):
                        path = os.path.join(root, file)
                        h = self._hash_file(path)
                        rel_path = os.path.relpath(path, os.path.dirname(__file__))
                        manifest[rel_path] = h
                        if create_backups:
                            bak_path = path + '.bak'
                            try:
                                shutil.copy2(path, bak_path)
                            except:
                                pass
        with open(self.manifest_path, 'w') as f:
            json.dump(manifest, f, indent=4)
        return f'Integrity: Baseline generated for {len(manifest)} shards. Backups: {create_backups}'