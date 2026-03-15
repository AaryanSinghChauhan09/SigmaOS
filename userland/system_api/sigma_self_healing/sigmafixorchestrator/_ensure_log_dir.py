"""
Auto-split from userland\system_api\sigma_self_healing.py — SigmaFixOrchestrator._ensure_log_dir
"""

import os
import sys
import json
import shutil
import importlib
import subprocess
import datetime
from typing import List, Dict, Any, Optional



class SigmaFixOrchestrator:
    def _ensure_log_dir(self) -> None:
        log_dir = os.path.dirname(self.log_path)
        if not os.path.exists(log_dir):
            os.makedirs(log_dir, exist_ok=True)
