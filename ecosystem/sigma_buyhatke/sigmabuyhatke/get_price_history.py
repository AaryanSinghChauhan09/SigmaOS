"""
Auto-split from ecosystem\sigma_buyhatke.py — SigmaBuyHatke.get_price_history
"""

import datetime
import random
from typing import Dict, List, Any



class SigmaBuyHatke:
    def get_price_history(self, product_name: str) -> List[int]:
        """Returns the price history trend for a product."""
        key = product_name.replace(' ', '_')
        return self._price_history.get(key, [random.randint(500, 5000) for _ in range(5)])
