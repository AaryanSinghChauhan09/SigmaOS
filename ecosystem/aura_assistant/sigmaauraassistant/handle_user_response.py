# Generated method: SigmaAuraAssistant.handle_user_response
from typing import Dict, List, Any, Optional
import uuid

class SigmaAuraAssistant:
    def handle_user_response(self, response: str) -> str:
        """
            Processes 'Approve', 'Deny', or 'Refine' voice commands.
            """
        if not self._active_mission:
            return 'No active mission to guide.'
        res_lower = response.lower()
        if 'approve' in res_lower or 'proceed' in res_lower or 'yes' in res_lower:
            self._stats['permissions_granted'] += 1
            return self._execute_current_step()
        elif 'refine' in res_lower or 'change' in res_lower:
            self._stats['steps_refined'] += 1
            return 'Understood. Please provide guidance on how to modify this step.'
        elif 'cancel' in res_lower or 'stop' in res_lower:
            self._active_mission = None
            return 'Mission aborted as per your guidance.'
        else:
            return "I'm sorry, I didn't catch that. Should I 'Proceed', 'Refine', or 'Cancel'?"