# Generated method: SigmaProjects.get_gantt_data
import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any

class SigmaProjects:
    def get_gantt_data(self) -> List[Dict]:
        """USP: Returns temporal mapping for Gantt Visualization."""
        data = []
        for tid, t in self._tasks.items():
            data.append({'id': tid, 'text': t.title, 'start': t.start_ts, 'duration_h': t.estimated_h, 'progress': min(100, t.actual_h / max(0.1, t.estimated_h) * 100), 'blockers': t.dependencies})
        return data