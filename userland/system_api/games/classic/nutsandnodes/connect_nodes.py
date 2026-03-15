"""
Auto-split from userland\system_api\games\classic.py — NutsAndNodes.connect_nodes
"""

import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame



class NutsAndNodes:
    def connect_nodes(self, f_id, t_id):
        a = next((n for n in self.nodes if n['id'] == f_id), None)
        if a and t_id not in a['connected']:
            a['connected'].append(t_id)
            self.score = int(self.score) + 10
