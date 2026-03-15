# Generated method: SigmaOmniSearch._fuzzy_match
import time
from typing import Dict, Any, List

class SigmaOmniSearch:
    def _fuzzy_match(self, term: str, target: str) -> float:
        """USP: Jaro-Winkler Simplicity for high-speed local relevance."""
        if term == target:
            return 1.0
        if term in target:
            return 0.8 + len(term) / len(target) * 0.2
        s1 = set(term)
        s2 = set(target)
        overlap = s1.intersection(s2)
        if not s1 or not s2:
            return 0.0
        return len(overlap) / len(s1.union(s2))