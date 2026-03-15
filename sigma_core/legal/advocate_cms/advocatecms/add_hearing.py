# Generated method: AdvocateCMS.add_hearing
import os
import json
import time
from typing import Dict, Any, List, Optional

class AdvocateCMS:
    def add_hearing(self, case_id: str, hearing_date: str, purpose: str):
        if case_id in self.case_ledger:
            self.case_ledger[case_id]['hearings'].append({'date': hearing_date, 'purpose': purpose, 'recorded_at': time.time()})
            self._save_ledger()
            return True
        return False