# Generated method: SiloFS.destroy_silo
import os
import shutil
from typing import List, Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SiloFS:
    def destroy_silo(self, silo_id: str):
        """Principle: Zero-Persistence wipe."""
        if silo_id in self.active_silos:
            path = self.active_silos[silo_id]['path']
            if os.path.exists(path):
                shutil.rmtree(path)
            del self.active_silos[silo_id]
            self.log_event('silo_purged', {'id': silo_id})
            return True
        return False