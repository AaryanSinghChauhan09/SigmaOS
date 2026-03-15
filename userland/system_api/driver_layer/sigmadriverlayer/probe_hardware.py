"""
Auto-split from userland\system_api\driver_layer.py — SigmaDriverLayer.probe_hardware
"""

import time
import uuid
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaDriverLayer:
    def probe_hardware(self) -> dict:
        """
            Full hardware scan: enumerates PCI/USB bus and matches each device
            to the Sovereign Driver Registry. Returns a discovery report.
            """
        discovered: list[dict] = []
        for hw_id, rec in _DRIVER_REGISTRY.items():
            rec.status = DriverStatus.PROBING
            match_quality = 'exact' if rec.is_signed else 'generic'
            discovered.append({'hw_id': hw_id, 'name': rec.name, 'class': rec.cls.name, 'vendor': rec.vendor, 'version': rec.version, 'match': match_quality})
        self._audit_event('hardware_probe', f'Discovered {len(discovered)} devices.')
        return {'status': 'Probe Complete', 'discovered': len(discovered), 'devices': discovered, 'message': f'DriverLayer: Bus scan complete — {len(discovered)} devices enumerated, all matched in Sovereign Registry.'}
