# Generated method: SigmaERP.launch_mfg_order
from typing import Dict, List, Any
import uuid

class SigmaERP:
    def launch_mfg_order(self, product: str, raw_materials: Dict[str, int]) -> str:
        for mat, qty in raw_materials.items():
            if self._inventory.get(mat, 0) < qty:
                return f"MRP Error: Insufficient '{mat}' in stock for order."
        for mat, qty in raw_materials.items():
            self._inventory[mat] -= qty
        self._stats['mfg_orders_completed'] += 1
        return f"MRP: Manufacturing order for '{product}' started. Resources deducted from Inventory."