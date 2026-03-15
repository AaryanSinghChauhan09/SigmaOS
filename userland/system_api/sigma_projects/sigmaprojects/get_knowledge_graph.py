"""
Auto-split from userland\system_api\sigma_projects.py — SigmaProjects.get_knowledge_graph
"""

import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any



class SigmaProjects:
    def get_knowledge_graph(self) -> Dict:
        """Returns node/edge data for graph visualization."""
        nodes = []
        for tid, t in self._tasks.items():
            depth = self.get_knowledge_depth(tid)
            nodes.append({'id': tid, 'label': t.title, 'priority': t.priority.value, 'depth': depth})
        edges = [{'from': l[0], 'to': l[1], 'type': l[2]} for l in self._task_links]
        return {'nodes': nodes, 'edges': edges}
