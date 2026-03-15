# Generated method: SigmaBuyHatke.compare_platforms
import datetime
import random
from typing import Dict, List, Any

class SigmaBuyHatke:
    def compare_platforms(self, product_name: str) -> Dict[str, int]:
        """USP: Multi-platform price comparison simulator."""
        base = random.randint(10000, 20000)
        return {'Amazon': base, 'Flipkart': base - 499, 'Reliance_Digital': base + 1200, 'Croma': base - 150}