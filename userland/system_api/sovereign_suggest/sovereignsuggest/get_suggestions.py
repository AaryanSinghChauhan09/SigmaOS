# Generated method: SovereignSuggest.get_suggestions
import time
import json
import os

class SovereignSuggest:
    def get_suggestions(self, query: str, limit: int=5) -> list[str]:
        """Returns local suggestions based on query string."""
        if not query or len(query) < 2:
            return []
        q = query.lower()
        matches = [s for s in self._dict if q in s.lower()]
        history_matches = [s for s in self._history if q in s.lower()]
        combined = list(dict.fromkeys(matches + history_matches))
        return combined[:limit]