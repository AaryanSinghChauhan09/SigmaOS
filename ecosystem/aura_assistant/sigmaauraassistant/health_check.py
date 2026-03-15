# Generated method: SigmaAuraAssistant.health_check
from typing import Dict, List, Any, Optional
import uuid

class SigmaAuraAssistant:
    def health_check(self) -> str:
        s = self._stats
        mission = 'Active' if self._active_mission else 'Idle'
        return f"OK — Status: {mission}, Goals: {s['goals_reached']}, Steps Refined: {s['steps_refined']}."