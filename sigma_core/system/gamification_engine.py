"""
SigmaOS Gamification Engine (v2.0 Apex)
========================================
USP: AI-Adaptive Challenges and Sovereign Achievement Fabric.
Integrates with Telemetry for real-world 'Life-Up' mechanics.
"""
import json
import os
import random
from typing import Dict, Any, List, Optional

try:
    from sigma_core.system.interfaces import SigmaModuleBase
except (ImportError, ValueError):
    class SigmaModuleBase:
        def __init__(self, kernel): self.kernel = kernel
        def log_event(self, a, c): pass

STATS_PATH = "userland/system_api/user_stats.json"

class GamificationEngine(SigmaModuleBase):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.kernel = kernel
        self.stats: Dict[str, Any] = self._load()
        self.daily_challenges: List[str] = []

    def _load(self) -> Dict[str, Any]:
        if not os.path.exists(STATS_PATH):
            initial = {
                "xp": 0, 
                "level": 1, 
                "achievements": [], 
                "experiments_done": 0, 
                "rank": "Initiate",
                "trust_score": 100.0,
                "carbon_karma": 100.0
            }
            self._save(initial)
            return initial
        try:
            with open(STATS_PATH, "r") as f:
                data = json.load(f)
                return data if isinstance(data, dict) else {}
        except:
            return {"xp": 0, "level": 1, "achievements": [], "experiments_done": 0, "rank": "Initiate"}

    def _save(self, data: Dict[str, Any]):
        try:
            os.makedirs(os.path.dirname(STATS_PATH), exist_ok=True)
            with open(STATS_PATH, "w") as f:
                json.dump(data, f, indent=4)
        except:
            pass

    def generate_adaptive_challenge(self):
        """USP: Generates challenges based on current OS load and telemetry."""
        if not self.kernel or not hasattr(self.kernel, "telemetry") or not self.kernel.telemetry:
            return "Challenge: Stability Test (Telemetry Link Offline)"
        
        telemetry_stats = self.kernel.telemetry.get_realtime_stats()
        cpu_stats = telemetry_stats.get("cpu", {})
        load = float(cpu_stats.get("load_percent", 0.0)) if isinstance(cpu_stats, dict) else 0.0
        
        if load < 10.0:
            challenge = "Energy Efficiency: Maintain <5% load for 5 minutes."
        elif load > 80.0:
            challenge = "Thermal Taming: Cool down system by 5°C via Resource Alchemist."
        else:
            challenge = "Shadow Protocol: Engage Stealth Guardian for 1 hour."
            
        self.daily_challenges.append(challenge)
        return f"New Adaptive Challenge: {challenge}"

    def add_xp(self, amount: int):
        xp = int(self.stats.get("xp", 0))
        new_xp = xp + amount
        self.stats["xp"] = new_xp
        
        lvl = int(self.stats.get("level", 1))
        new_lvl = 1 + (new_xp // 500)
        
        if new_lvl > 100: self.stats["rank"] = "Sovereign Apex Overlord"
        elif new_lvl > 50: self.stats["rank"] = "High Architect"
        elif new_lvl > 20: self.stats["rank"] = "Neural Sentinel"
        
        if new_lvl > lvl:
            self.stats["level"] = new_lvl
            self.unlock_achievement(f"Level {new_lvl} Sovereign")
            if self.kernel and hasattr(self.kernel, "bus") and self.kernel.bus:
                self.kernel.bus.emit("gamification.level_up", {"level": new_lvl, "rank": self.stats["rank"]})
                
        self._save(self.stats)

    def record_interaction(self, action_type: str):
        """USP: Automated XP gain for interactive OS mastery."""
        xp_map = {"MESH_OFFLOAD": 50, "STEALTH_ENGAGED": 20, "THREAT_BLOCKED": 100}
        gain = xp_map.get(action_type, 10)
        self.add_xp(gain)

    def unlock_achievement(self, title: str):
        ach = self.stats.get("achievements", [])
        if isinstance(ach, list):
            if title not in ach:
                ach.append(title)
                self.stats["achievements"] = ach
                self._save(self.stats)

    def get_status(self) -> Dict[str, Any]:
        return {
            "Level": int(self.stats.get("level", 1)),
            "Rank": self.stats.get("rank", "Initiate"),
            "Total XP": int(self.stats.get("xp", 0)),
            "Achievements": len(self.stats.get("achievements", [])),
            "Environmental Karma": self.stats.get("carbon_karma", 100.0)
        }

    def health_check(self) -> str:
        return f"OK — Rank: {self.stats.get('rank')} | Level: {self.stats.get('level')}"
