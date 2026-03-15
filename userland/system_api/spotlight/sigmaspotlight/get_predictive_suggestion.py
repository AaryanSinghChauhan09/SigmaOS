# Generated method: SigmaSpotlight.get_predictive_suggestion
from typing import List, Dict, Any
import time

class SigmaSpotlight:
    def get_predictive_suggestion(self) -> str:
        """USP: Anticipatory OS - predicts what the user needs next."""
        if len(self._history) > 0 and 'law' in self._history[-1].lower():
            return 'Summarize BNSS Section 154'
        return 'Optimize System for Productivity'