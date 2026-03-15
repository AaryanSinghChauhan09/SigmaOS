# Generated method: SigmaPackageManager.list_installed
import time
import uuid
import random
from typing import Dict, List, Any

class SigmaPackageManager:
    def list_installed(self) -> List[str]:
        return [f'{p} ({v})' for p, v in self._installed.items()]