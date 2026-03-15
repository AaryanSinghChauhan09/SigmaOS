# Generated method: SigmaModuleBase.log_event
from abc import ABC, abstractmethod
from typing import Dict, Any, Optional

class SigmaModuleBase:
    def log_event(self, action: str, context: Dict[str, Any]):
        if self.kernel and hasattr(self.kernel, 'ledger'):
            self.kernel.ledger.commit(self.get_module_id(), action, context)