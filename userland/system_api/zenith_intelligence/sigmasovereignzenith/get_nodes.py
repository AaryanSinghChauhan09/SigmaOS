# Generated method: SigmaSovereignZenith.get_nodes
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import time
import random
import json
import os
from pathlib import Path

class SigmaSovereignZenith:
    def get_nodes(self) -> List[Dict]:
        """Returns the current state of all AI nodes and their quotas."""
        self._refresh_quotas()
        return [{'name': n.name, 'url': n.url, 'color': n.color, 'category': n.category, 'usage': n.used_percent, 'status': n.status} for n in self.nodes.values()]