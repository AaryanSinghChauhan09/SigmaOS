# Generated method: SigmaProjects.get_resource_allocation_matrix
import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any

class SigmaProjects:
    def get_resource_allocation_matrix(self) -> Dict[str, float]:
        """USP: Maps task priorities to resource weights (Apex Feature)."""
        matrix = {'CRITICAL': 0.0, 'HIGH': 0.0, 'STANDARD': 0.0}
        for t in self._tasks.values():
            if t.status != TaskStatus.DONE:
                if t.priority == Priority.URGENT:
                    matrix['CRITICAL'] += t.estimated_h
                elif t.priority == Priority.HIGH:
                    matrix['HIGH'] += t.estimated_h
                else:
                    matrix['STANDARD'] += t.estimated_h
        return matrix