# Generated method: NCERTEducationEngine.check_attainments
from typing import List, Dict, Any, Optional
import time

class NCERTEducationEngine:
    def check_attainments(self) -> List[str]:
        """USP: Real-time credentialing for scientific progress."""
        unlocked = []
        for badge, threshold in self.BADGES.items():
            if self.xp >= threshold:
                unlocked.append(badge)
        return unlocked