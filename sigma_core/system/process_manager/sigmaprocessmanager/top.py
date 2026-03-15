"""
Auto-split from sigma_core\system\process_manager.py — SigmaProcessManager.top
"""

import time
import uuid
import random
import threading
from typing import Dict, List, Any, Optional
from enum import Enum
from dataclasses import dataclass, field



class SigmaProcessManager:
    def top(self, n: int=10) -> List[Dict[str, Any]]:
        pl = self.list_processes()
        _sorted = sorted(pl, key=lambda x: x['cpu'], reverse=True)
        res = []
        for i in range(min(len(_sorted), n)):
            res.append(_sorted[i])
        return res
