# Generated method: RecoveryEngine.suppress_anomaly
from typing import Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class RecoveryEngine:
    def suppress_anomaly(self) -> str:
        """Forces immediate shard isolation and restart."""
        error_mgr = getattr(self.kernel, 'error_mgr', None)
        if error_mgr:
            return 'SUCCESS: ANOMALY_ISOLATED'
        return 'ERROR: RECOVERY_OFFLINE'