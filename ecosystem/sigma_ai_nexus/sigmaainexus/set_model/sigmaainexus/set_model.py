# Generated method: SigmaAINexus.set_model
import time
import random
from typing import Dict, List, Any

class SigmaAINexus:
    def set_model(self, model_id: str) -> str:
        if hasattr(self.kernel.cfg, 'LOCAL_ONLY_MODE') and self.kernel.cfg.LOCAL_ONLY_MODE:
            if model_id != 'Sovereign':
                return "Error: External model disabled in LOCAL_ONLY mode. Use 'Sovereign'."
        if model_id in self._available_models:
            self._active_model = model_id
            return f'√ Intelligence shifted to {model_id}.'
        return 'Error: Unknown model ID.'