"""
SigmaGuardian: OS-Level Parental Control & Child Safety System.
Enforces age-appropriate content across all modules.
"""
import os
from sigma_core.system.config import SigmaConfig # type: ignore

class SigmaGuardian:
    def __init__(self, kernel):
        self.kernel = kernel
        self.cfg = SigmaConfig()
        self._child_mode = False
        self._target_age = 5
        
        # Mapping international ratings to safety levels
        # Level 0: Safe for all (G, U, All Ages)
        # Level 1: Parental Guidance (PG)
        # Level 2: Restricted (12+, 15+)
        # Level 3: Adult (18+)
        self.SAFE_RATINGS = ["G", "U", "All Ages", "0+"]
        
    def set_child_mode(self, enabled: bool, age: int = 5):
        self._child_mode = enabled
        self._target_age = age
        self.kernel.bus.publish("system.guardian_mode_changed", {"enabled": enabled, "age": age})
        print(f"[GUARDIAN] Child Mode: {'ENABLED' if enabled else 'DISABLED'} (Age: {age})")

    def is_child_mode(self) -> bool:
        return self._child_mode

    def get_target_age(self) -> int:
        return self._target_age

    def filter_content(self, items: list, rating_key: str = "rating") -> list:
        """Filters a list of items based on their age rating if child mode is active."""
        if not self._child_mode:
            return items
            
        filtered = []
        for item in items:
            rating = item.get(rating_key, "G")
            if rating in self.SAFE_RATINGS:
                filtered.append(item)
        return filtered

    def check_access(self, rating: str) -> bool:
        """Returns True if the rating is allowed in the current mode."""
        if not self._child_mode:
            return True
        return rating in self.SAFE_RATINGS

    def get_safety_report(self):
        return {
            "mode": "Child" if self._child_mode else "Administrator",
            "age_limit": self._target_age,
            "allowed_ratings": self.SAFE_RATINGS if self._child_mode else "ALL",
            "compliance": "NIST/COPPA Multi-Region"
        }
