# Generated method: SigmaBharatLawBridge.audit_compliance
from typing import Dict, List, Any, Optional
import datetime

class SigmaBharatLawBridge:
    def audit_compliance(self, entity_type: str) -> List[str]:
        """Returns compliance checklist for GST/MCA/SEBI."""
        return self._compliance_checks.get(entity_type, ['Standard Corporate Compliance'])