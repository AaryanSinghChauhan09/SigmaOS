# Generated method: SigmaAppStore.get_reviews
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import time
import hashlib
import json

class SigmaAppStore:
    def get_reviews(self, app_id: str) -> List[Dict]:
        return [{'reviewer': r.reviewer_id, 'rating': r.rating, 'comment': r.comment, 'verified': r.verified_purchase} for r in self._reviews.get(app_id, [])]