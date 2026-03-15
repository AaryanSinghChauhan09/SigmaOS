# Generated method: AetherAssistant._classify_intent
import re
import time
from typing import Dict, List, Any

class AetherAssistant:
    def _classify_intent(self, tokens: List[str]) -> str:
        """Calculates probabilistic match for intents based on token density."""
        scores: Dict[str, float] = {}
        for intent in self._intents:
            scores[intent] = 0.0
        for token in tokens:
            for intent, keywords in self._intents.items():
                if token in keywords:
                    scores[intent] = scores[intent] + 1
                for kw in keywords:
                    if kw in ' '.join(tokens):
                        scores[intent] = scores[intent] + 1.5
        best_intent = 'unknown'
        max_score = 0.0
        for k, v in scores.items():
            if v > max_score:
                max_score = v
                best_intent = k
        return best_intent if max_score > 0 else 'unknown'