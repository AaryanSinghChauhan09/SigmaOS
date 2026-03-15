# Generated method: LegalEngine.get_compliance_stats
from typing import List, Dict, Any

class LegalEngine:
    def get_compliance_stats(self) -> Dict[str, Any]:
        """USP: Analytics for legal workflow efficiency."""
        completed = sum((1 for s in self.stages if s['status'] == 'COMPLETED'))
        return {'total': len(self.stages), 'completed': completed, 'percentage': completed / len(self.stages) * 100}