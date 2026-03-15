# Generated method: SigmaContextEngine.detect_intent
from typing import Dict, List, Any
import time

class SigmaContextEngine:
    def detect_intent(self, app_activity: str) -> str:
        """USP: AI-driven heuristic to detect user goals and auto-pivot the OS."""
        for context, profile in self._context_mapping.items():
            if context.lower() in app_activity.lower():
                self._active_context = context
                self.kernel.modes.switch_mode(profile['Modes'])
                return f"Context: Intent detected as '{context}'. OS re-profiled for {profile['Modes']}."
        return 'Context: Intent steady. Maintaining current profile.'