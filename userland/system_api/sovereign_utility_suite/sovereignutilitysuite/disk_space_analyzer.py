"""
Auto-split from userland\system_api\sovereign_utility_suite.py — SovereignUtilitySuite.disk_space_analyzer
"""

import os
import random
import time
import json
import hashlib
import re
import difflib
import base64
import statistics
from typing import Dict, Any, List, Optional
from datetime import datetime



class SovereignUtilitySuite:
    def disk_space_analyzer(self, directory: str) -> List[Dict[str, Any]]:
        """USP: WinDirStat / TreeSize Parity. Visualizes local storage distribution."""
        self.stats['utils_executed'] += 1
        items = []
        try:
            for entry in os.scandir(directory):
                info = entry.stat()
                raw_mb = float(info.st_size) / (1024.0 * 1024.0)
                items.append({'Name': entry.name, 'Size_MB': round(raw_mb, 2), 'Type': 'DIR' if entry.is_dir() else 'FILE'})
        except Exception:
            pass
        return sorted(items, key=lambda x: x['Size_MB'], reverse=True)
