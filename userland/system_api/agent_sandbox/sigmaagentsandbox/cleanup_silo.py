# Generated method: SigmaAgentSandbox.cleanup_silo
import os
import shutil
import uuid
import time
import subprocess
import threading
from typing import Dict, Any, List

class SigmaAgentSandbox:
    def cleanup_silo(self, silo_id: str):
        """Zero-Persistence: Deletes the silo and all its data."""
        silo = self._active_silos.get(silo_id)
        if silo and os.path.exists(silo['path']):
            shutil.rmtree(silo['path'])
            del self._active_silos[silo_id]
            return True
        return False