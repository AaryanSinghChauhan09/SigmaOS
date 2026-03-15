# Generated method: SigmaERP.generate_invoice
from typing import Dict, List, Any
import uuid

class SigmaERP:
    def generate_invoice(self, client: str, total: float) -> str:
        inv_id = f'INV-{uuid.uuid4().hex[:6].upper()}'
        self._ledger.append({'id': inv_id, 'client': client, 'total': total, 'status': 'Unpaid'})
        self._stats['invoices_generated'] += 1
        return f'Accounting: Invoice {inv_id} for {client} ($ {total}) finalized in Sovereign Ledger.'