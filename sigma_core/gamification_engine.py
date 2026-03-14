"""
SigmaOS Gamification Engine (v1.1 Sovereign)
==============================================
USP: Achievement-tracked academic progression.
100% Native JSON | Linter-Optimized.
"""
import json
import os
from typing import Dict, Any, List

STATS_PATH = "userland/system_api/user_stats.json"

class GamificationEngine:
    def __init__(self):
        self.stats: Dict[str, Any] = self._load()

    def _load(self) -> Dict[str, Any]:
        if not os.path.exists(STATS_PATH):
            initial = {"xp": 0, "level": 1, "achievements": [], "experiments_done": 0}
            self._save(initial)
            return initial
        try:
            with open(STATS_PATH, "r") as f:
                data = json.load(f)
                return data if isinstance(data, dict) else {}
        except:
            return {"xp": 0, "level": 1, "achievements": [], "experiments_done": 0}

    def _save(self, data: Dict[str, Any]):
        try:
            with open(STATS_PATH, "w") as f:
                json.dump(data, f, indent=4)
        except:
            pass

    def add_xp(self, amount: int):
        xp = int(self.stats.get("xp", 0))
        new_xp = xp + amount
        self.stats["xp"] = new_xp
        
        lvl = int(self.stats.get("level", 1))
        new_lvl = 1 + (new_xp // 500)
        if new_lvl > lvl:
            self.stats["level"] = new_lvl
            self.unlock_achievement(f"Level {new_lvl} Researcher")
        self._save(self.stats)

    def record_experiment(self, name: str):
        done = int(self.stats.get("experiments_done", 0))
        self.stats["experiments_done"] = done + 1
        self.add_xp(50)
        
        if done + 1 == 10:
            self.unlock_achievement("Science Apprentice")
        self._save(self.stats)

    def unlock_achievement(self, title: str):
        ach = self.stats.get("achievements")
        if isinstance(ach, list):
            if title not in ach:
                ach.append(title)
                self.stats["achievements"] = ach
                self._save(self.stats)

    def get_status(self) -> Dict[str, Any]:
        return {
            "Level": int(self.stats.get("level", 1)),
            "Total XP": int(self.stats.get("xp", 0)),
            "Labs Done": int(self.stats.get("experiments_done", 0)),
            "Badges": len(self.stats.get("achievements", []))
        }
