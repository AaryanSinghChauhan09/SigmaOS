# Generated method: GamificationEngine.unlock_achievement
import json
import os
import random
import sys
from typing import Dict, Any, List, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class GamificationEngine:
    def unlock_achievement(self, title: str):
        ach = self.stats.get('achievements', [])
        if title not in ach:
            ach.append(title)
            self.stats['achievements'] = ach
            self._save(self.stats)
            self.log_event('achievement_unlocked', {'title': title})