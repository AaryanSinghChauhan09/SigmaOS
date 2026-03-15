# Generated method: AdvocateCMS.get_upcoming_hearings
import os
import json
import time
from typing import Dict, Any, List, Optional

class AdvocateCMS:
    def get_upcoming_hearings(self) -> List[Dict[str, Any]]:
        """USP: Litigation Pulse. Returns all hearings for the coming week."""
        hearings = []
        for cid, case in self.case_ledger.items():
            for h in case['hearings']:
                hearings.append({'case_id': cid, 'case_fn': case['case_fn'], 'court': case['court'], 'date': h['date'], 'purpose': h['purpose']})
        return sorted(hearings, key=lambda x: x['date'])