# Generated method: SiloFS.health_check
import os
import shutil
from typing import List, Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SiloFS:
    def health_check(self) -> str:
        return f'OK — Active Silos: {len(self.active_silos)} | Backend: Layered_FS'