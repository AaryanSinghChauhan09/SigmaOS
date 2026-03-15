# Generated method: AdvocateCMS.add_case
import os
import json
import time
from typing import Dict, Any, List, Optional

class AdvocateCMS:
    def add_case(self, case_fn: str, client: str, court: str, status: str='PENDING') -> str:
        """USP: Sovereign Case Filing. Assigns a unique OS-ID to the litigation."""
        case_id = f'ADV-{int(time.time())}'
        case_blob = {'case_id': case_id, 'case_fn': case_fn, 'client': client, 'court': court, 'status': status, 'hearings': [], 'created_at': time.time()}
        self.case_ledger[case_id] = case_blob
        self._save_ledger()
        self.log_event('case_added', {'case_id': case_id})
        return case_id