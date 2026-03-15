# Generated method: SigmaModuleBase.log_event
import json
import os
import time
from typing import Dict, Any, List, Optional
from .statutory_data import GRAND_LIBRARY

class SigmaModuleBase:
    def log_event(self, action: str, context: Dict[str, Any]):
        if self.kernel and hasattr(self.kernel, 'bus'):
            self.kernel.bus.emit(f'legal_forms.{action}', context)