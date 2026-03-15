# Generated method: SigmaProjects.get_velocity_forecast
import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any

class SigmaProjects:
    def get_velocity_forecast(self) -> Dict:
        """USP: Predicts capacity for the next 3 sprints."""
        ana = self.get_scrum_analytics()
        vel = ana['velocity']
        eff = ana['efficiency'] / 100.0
        return {'p50_capacity': round(vel * 1.0, 1), 'p90_capacity': round(vel * 1.2 * eff, 1), 'trend': 'STABLE' if eff > 0.8 else 'DEGRADING', 'suggestion': "Increase focus on 'Done' cycle to stabilize p90."}