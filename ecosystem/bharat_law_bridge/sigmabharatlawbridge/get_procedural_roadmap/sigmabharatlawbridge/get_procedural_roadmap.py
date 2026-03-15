# Generated method: SigmaBharatLawBridge.get_procedural_roadmap
from typing import Dict, List, Any, Optional
import datetime

class SigmaBharatLawBridge:
    def get_procedural_roadmap(self, scenario: str) -> List[str]:
        """Returns a step-by-step 'Legal GPS' guide for a specific scenario."""
        return self._workflows.get(scenario, ['Scenario roadmap not found. Please consult the Sovereign Manual.'])