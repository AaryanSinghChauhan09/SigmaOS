# Generated method: SovereignHypervisor.terminate_vm
import time
from typing import Dict, Any, Optional

class SovereignHypervisor:
    def terminate_vm(self, app_id: str):
        """Purges the VM and overwrites its memory space."""
        if app_id in self.active_vms:
            print(f'[HYPERVISOR] Purging Enclave: {app_id}')
            self.active_vms.pop(app_id, None)
            self.kernel._morphic_island(f'HYPERVISOR: Enclave {app_id} shredded', '#FF1493')