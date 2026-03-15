# Generated method: SigmaModuleBase.log_event
import sys
import traceback
from typing import Dict, Any, List, Optional, Callable

class SigmaModuleBase:
    def log_event(self, action: str, context: Dict[str, Any]):
        if self.kernel and hasattr(self.kernel, 'bus'):
            self.kernel.bus.emit(f'error.{action}', context)