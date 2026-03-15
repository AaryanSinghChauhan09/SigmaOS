# Generated method: SigmaModuleBase.log_event
import os
import random
import time
from typing import Dict, Any, List, Optional

class SigmaModuleBase:
    def log_event(self, action: str, context: Dict[str, Any]):
        if self.kernel and hasattr(self.kernel, 'bus'):
            self.kernel.bus.emit(f'stealth.{action}', context)