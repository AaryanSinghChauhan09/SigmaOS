# Generated method: SigmaERP.health_check
from typing import Dict, List, Any
import uuid

class SigmaERP:
    def health_check(self) -> str:
        s = self._stats
        return f"OK — Leads: {len(self._crm_leads)}, Invoices: {s['invoices_generated']}, Projects: {len(self._projects)}."