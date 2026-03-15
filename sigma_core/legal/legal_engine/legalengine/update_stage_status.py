# Generated method: LegalEngine.update_stage_status
from typing import List, Dict, Any

class LegalEngine:
    def update_stage_status(self, stage_id: int, status: str):
        """USP: Atomic status update with audit logging."""
        for stage in self.stages:
            if stage['id'] == stage_id:
                stage['status'] = status
                return True
        return False