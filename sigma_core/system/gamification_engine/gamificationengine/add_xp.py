# Generated method: GamificationEngine.add_xp
import json
import os
import random
import sys
from typing import Dict, Any, List, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class GamificationEngine:
    def add_xp(self, amount: int):
        """USP: Personalized Leveling Curve."""
        old_xp = int(self.stats.get('xp', 0))
        new_xp = old_xp + amount
        self.stats['xp'] = new_xp
        old_lvl = int(self.stats.get('level', 1))
        new_lvl = 1 + new_xp // 1000
        if new_lvl > old_lvl:
            self.stats['level'] = new_lvl
            self._on_level_up(new_lvl)
        self._save(self.stats)