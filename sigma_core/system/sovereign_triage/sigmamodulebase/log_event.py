# Generated method: SigmaModuleBase.log_event
import time
import uuid
from typing import Dict, Any, List, Optional

class SigmaModuleBase:
    def log_event(self, action: str, context: Dict[str, Any]):
        if self.kernel and hasattr(self.kernel, 'bus'):
            self.kernel.bus.emit(f'triage.{action}', context)