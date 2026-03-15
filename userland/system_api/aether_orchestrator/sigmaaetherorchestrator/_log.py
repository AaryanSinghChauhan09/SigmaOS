# Generated method: SigmaAetherOrchestrator._log
import json
from dataclasses import dataclass
from typing import Dict, List, Any, Optional

class SigmaAetherOrchestrator:
    def _log(self, msg: str):
        if self.kernel:
            self.kernel.bus.emit('aether.log', {'msg': msg})