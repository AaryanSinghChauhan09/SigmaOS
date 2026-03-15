# Generated method: SigmaERP.update_stock
from typing import Dict, List, Any
import uuid

class SigmaERP:
    def update_stock(self, sku: str, quantity: int) -> str:
        self._inventory[sku] = self._inventory.get(sku, 0) + quantity
        self._stats['stock_audits'] += 1
        return f"Inventory: SKU '{sku}' updated. Current stock: {self._inventory[sku]} units."