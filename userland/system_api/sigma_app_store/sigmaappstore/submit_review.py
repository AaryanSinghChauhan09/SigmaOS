"""
Auto-split from userland\system_api\sigma_app_store.py — SigmaAppStore.submit_review
"""

from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import time
import hashlib
import json



class SigmaAppStore:
    def submit_review(self, app_id: str, reviewer_id: str, rating: int, comment: str) -> str:
        if app_id not in self._installed:
            return 'Error: You must install an app before reviewing it (Verified Purchase policy).'
        if not 1 <= rating <= 5:
            return 'Error: Rating must be 1–5.'
        review = AppReview(reviewer_id, rating, comment)
        self._reviews.setdefault(app_id, []).append(review)
        reviews = self._reviews[app_id]
        self._catalog[app_id].rating = round(sum((r.rating for r in reviews)) / len(reviews), 1)
        return f"✅ Review submitted for '{self._catalog[app_id].name}'. New rating: {self._catalog[app_id].rating}/5."
