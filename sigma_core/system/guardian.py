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
        self._child_mode = True # FORCED PERMANENT CHILD MODE
        self._target_age = 5
        
        # Mapping international ratings to safety levels
        # Level 0: Safe for all (G, U, All Ages)
        # Level 1: Parental Guidance (PG)
        # Level 2: Restricted (12+, 15+)
        # Level 3: Adult (18+)
        self.SAFE_RATINGS = ["G", "U", "All Ages", "0+"]
        
    def set_child_mode(self, enabled: bool, age: int = 5):
        # Child Mode is now system-enforced and cannot be disabled.
        self._child_mode = True 
        self._target_age = age
        self.kernel.bus.publish("system.guardian_mode_changed", {"enabled": True, "age": age})
        print(f"[GUARDIAN] CHILD MODE SYSTEM-ENFORCED (Age: {age})")

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

    def sanitize_text(self, text: str) -> str:
        """Replaces scary/technical OS words with child-friendly ones."""
        if not self._child_mode:
            return text
            
        replacements = {
            "TERMINAL": "FUN BOX",
            "KERNEL": "OS BRAIN",
            "FAULT": "BOO-BOO",
            "ELEVATED": "SUPERPOWER",
            "SUDO": "MAGIC WORD",
            "ROOT": "SUPER BOSS",
            "RECONSTRUCT": "FIX UP",
            "TELEMETRY": "HAPPY LOGS",
            "SECURITY": "SAFETY",
            "COMPETITOR": "FRIENDLY",
            "BLAME": "SCORE",
            "PURGE": "TIDY UP",
            "FORCE": "PLEASE",
            "KILL": "NAP",
            "ATTACK": "TUG",
            "SHIELD": "RAINBOW",
            "SURGEON": "MAGIC BRUSH",
            "ABSORPTION": "HUGGING",
            "ZERO-TRUST": "HUG-READY",
            "COMPLIANCE": "GOLD STAR",
            "CYCLES": "HAPPY BEATS"
        }
        
        upper_text = text.upper()
        for scary, fun in replacements.items():
            if scary in upper_text:
                # Try to preserve case if possible, otherwise just use the replacement
                text = text.replace(scary, fun)
                text = text.replace(scary.capitalize(), fun.capitalize())
                text = text.replace(scary.lower(), fun.lower())
        return text
