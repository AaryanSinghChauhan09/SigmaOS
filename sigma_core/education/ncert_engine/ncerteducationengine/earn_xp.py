# Generated method: NCERTEducationEngine.earn_xp
from typing import List, Dict, Any, Optional
import time

class NCERTEducationEngine:
    def earn_xp(self, amount: int=50):
        """USP: Recursive XP calibration based on session intensity."""
        self.xp += amount
        if time.time() - self.last_activity < 3600:
            self.streak += 1
        else:
            self.streak = 1
        self.last_activity = time.time()
        return self.xp