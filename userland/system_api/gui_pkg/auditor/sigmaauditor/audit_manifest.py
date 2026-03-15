# Generated method: SigmaAuditor.audit_manifest
import os
import sys
import importlib

class SigmaAuditor:
    def audit_manifest(self):
        print('[AUDIT] Checking manifest consistency...')
        try:
            from sigma_core.manifest import CORE_SYSTEM_MODULES, ECOSYSTEM_APPS
        except ImportError as e:
            self.errors.append(f'CRITICAL: Could not load manifest: {e}')
            return
        for path, cls, key in CORE_SYSTEM_MODULES:
            try:
                mod_path = path.replace('.', '/') + '.py'
                if not os.path.exists(mod_path):
                    self.warnings.append(f'Manifest path mismatch: {mod_path}')
            except Exception as e:
                self.errors.append(f'Path verify error for {key}: {e}')