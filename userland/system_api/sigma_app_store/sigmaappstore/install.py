"""
Auto-split from userland\system_api\sigma_app_store.py — SigmaAppStore.install
"""

from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import time
import hashlib
import json



class SigmaAppStore:
    def install(self, app_id: str) -> Dict[str, Any]:
        """
            Installs an application into the sovereign environment.
            Verifies checksum, runs sandbox init, logs to ledger.
            """
        if app_id not in self._catalog:
            return {'success': False, 'error': f"App '{app_id}' not found in Sovereign Catalog."}
        app = self._catalog[app_id]
        if app.installed:
            return {'success': False, 'error': f"'{app.name}' is already installed."}
        expected_checksum = self._compute_checksum(app.app_id, app.version)
        if app.checksum != expected_checksum:
            self._ledger_write(f'SECURITY: Checksum mismatch for {app_id} — INSTALL BLOCKED')
            return {'success': False, 'error': 'Integrity check FAILED. Install blocked by Sovereign Shield.'}
        sandbox_result = self._init_sandbox(app)
        if not sandbox_result['ok']:
            return {'success': False, 'error': f"Sandbox setup failed: {sandbox_result['reason']}"}
        app.installed = True
        app.install_path = f"/sigma/apps/{app.app_id.replace('.', '/')}"
        app.downloads += 1
        self._installed[app_id] = app
        self._reviews[app_id] = []
        entry = f"[{time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())}] INSTALL OK | {app.name} v{app.version} | sandbox={app.sandbox_level} | checksum={app.checksum[:12]}..."
        self._ledger_write(entry)
        return {'success': True, 'app_id': app_id, 'name': app.name, 'version': app.version, 'path': app.install_path, 'sandbox': app.sandbox_level, 'message': f"✅ '{app.name} v{app.version}' installed successfully in {app.sandbox_level} sandbox."}
