"""
Auto-split from userland\system_api\ag_enterprise.py — AntigravityToolsFinder.map_tools
"""

import os
import re
import json
import time
from typing import List, Dict, Any, Optional



class AntigravityToolsFinder:
    def map_tools(self) -> List[Dict]:
        tools = []
        for root, dirs, files in os.walk(self.base_path):
            for f in files:
                if 'ag' in f.lower() or 'antigravity' in f.lower():
                    tools.append({'name': f, 'path': os.path.join(root, f)})
        return tools
