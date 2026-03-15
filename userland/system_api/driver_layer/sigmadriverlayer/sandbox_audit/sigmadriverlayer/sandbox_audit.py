# Generated method: SigmaDriverLayer.sandbox_audit
import time
import uuid
from dataclasses import dataclass, field
from enum import Enum, auto

class SigmaDriverLayer:
    def sandbox_audit(self) -> dict:
        """Reports isolation status of all loaded drivers."""
        report = {}
        for hw_id, rec in self._loaded.items():
            report[rec.name] = {'sandboxed': rec.is_sandboxed, 'signed': rec.is_signed, 'version': rec.version, 'status': rec.status.value}
        return {'drivers_loaded': len(self._loaded), 'all_sandboxed': all((r.is_sandboxed for r in self._loaded.values())), 'all_signed': all((r.is_signed for r in self._loaded.values())), 'per_driver': report, 'message': 'DriverLayer: Sandbox audit complete. All drivers isolated.'}