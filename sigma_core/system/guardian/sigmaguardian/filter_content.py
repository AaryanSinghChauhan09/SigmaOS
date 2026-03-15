# Generated method: SigmaGuardian.filter_content
import os
from sigma_core.system.config import SigmaConfig

class SigmaGuardian:
    def filter_content(self, items: list, rating_key: str='rating') -> list:
        """Filters a list of items based on their age rating if child mode is active."""
        if not self._child_mode:
            return items
        filtered = []
        for item in items:
            rating = item.get(rating_key, 'G')
            if rating in self.SAFE_RATINGS:
                filtered.append(item)
        return filtered