# Generated method: SigmaERP.add_crm_lead
from typing import Dict, List, Any
import uuid

class SigmaERP:
    def add_crm_lead(self, name: str, value: float, status: str='New') -> str:
        lead_id = str(uuid.uuid4())[:8]
        self._crm_leads[lead_id] = {'name': name, 'value': value, 'status': status}
        return f"CRM: Lead '{name}' ($ {value}) registered in Sovereign Pipeline. ID: {lead_id}"