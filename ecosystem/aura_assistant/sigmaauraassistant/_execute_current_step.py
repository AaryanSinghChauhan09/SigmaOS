# Generated method: SigmaAuraAssistant._execute_current_step
from typing import Dict, List, Any, Optional
import uuid

class SigmaAuraAssistant:
    def _execute_current_step(self) -> str:
        idx = self._active_mission['current_step_index']
        step_done = self._active_mission['steps'][idx]
        self._active_mission['current_step_index'] += 1
        if self._active_mission['current_step_index'] >= len(self._active_mission['steps']):
            goal = self._active_mission['goal']
            self._active_mission = None
            self._stats['goals_reached'] += 1
            return f"🔊 Success: Goal '{goal}' achieved. All sovereign protocols observed."
        next_req = self._request_permission_for_step(self._active_mission['current_step_index'])
        return f"✅ Step '{step_done}' executed successfully. {next_req}"