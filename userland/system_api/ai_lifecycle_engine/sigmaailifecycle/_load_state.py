"""
Auto-split from userland\system_api\ai_lifecycle_engine.py — SigmaAILifecycle._load_state
"""

import time
import uuid
import random
from typing import Dict, List, Any, Optional
from enum import Enum



class SigmaAILifecycle:
    def _load_state(self):
        import json, os
        if os.path.exists(self.state_file):
            try:
                with open(self.state_file, 'r') as f:
                    data = json.load(f)
                    self._stats = data.get('stats', self._stats)
                    raw_projects = data.get('projects', {})
                    for k, v in raw_projects.items():
                        vt = v['type']
                        v['type'] = getattr(MissionType, vt, MissionType.ML)
                        self.active_projects[k] = v
            except Exception:
                pass
