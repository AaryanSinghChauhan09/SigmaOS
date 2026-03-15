# Generated method: IntegrityGuard.verify_system_integrity
import hashlib
import os
import sys
import json
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase

class IntegrityGuard:
    def verify_system_integrity(self) -> Dict[str, Any]:
        """USP: Deep Forensic Audit. Compares system shards against the manifest."""
        if not os.path.exists(self.manifest_path):
            return {'status': 'UNCONFIGURED', 'message': 'No integrity manifest found.'}
        with open(self.manifest_path, 'r') as f:
            manifest = json.load(f)
        tampered = []
        for rel_path, expected_hash in manifest.items():
            abs_path = os.path.abspath(os.path.join(os.path.dirname(__file__), rel_path))
            if not os.path.exists(abs_path):
                tampered.append({'path': rel_path, 'reason': 'DELETED'})
                continue
            current_hash = self._hash_file(abs_path)
            if current_hash != expected_hash:
                tampered.append({'path': rel_path, 'reason': 'MODIFIED'})
                self._preserve_evidence(abs_path, current_hash)
        self.stats['verifications'] += 1
        self.stats['shards_verified'] = len(manifest)
        if tampered:
            self.stats['tamper_events'] += len(tampered)
            if self.kernel and hasattr(self.kernel, 'bus'):
                self.kernel.bus.emit('integrity.tamper', {'violations': tampered})
            return {'status': 'TAMPERED', 'violations': tampered}
        return {'status': 'PURE', 'shards': len(manifest)}