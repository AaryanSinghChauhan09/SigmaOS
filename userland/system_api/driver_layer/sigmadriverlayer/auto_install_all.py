"""
Auto-split from userland\system_api\driver_layer.py — SigmaDriverLayer.auto_install_all
"""

import time
import uuid
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaDriverLayer:
    def auto_install_all(self) -> dict:
        """Batch auto-install all discovered devices."""
        results = []
        for hw_id in _DRIVER_REGISTRY:
            results.append(self.auto_install(hw_id))
        loaded = sum((1 for r in results if r['status'] == 'Installed'))
        return {'status': 'Batch Complete', 'total': len(results), 'loaded': loaded, 'failed': len(results) - loaded, 'message': f'DriverLayer: {loaded}/{len(results)} drivers installed automatically.'}
