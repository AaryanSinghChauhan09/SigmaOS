# Generated method: SigmaAILifecycle._save_state
import time
import uuid
import random
from typing import Dict, List, Any, Optional
from enum import Enum

class SigmaAILifecycle:
    def _save_state(self):
        import json
        try:
            serializable = {}
            for k, v in self.active_projects.items():
                v_copy = v.copy()
                v_copy['type'] = v['type'].name
                serializable[k] = v_copy
            with open(self.state_file, 'w') as f:
                json.dump({'projects': serializable, 'stats': self._stats}, f, indent=4)
        except Exception:
            pass