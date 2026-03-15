# Generated method: SigmaBuyHatke.analyze_deal
import datetime
import random
from typing import Dict, List, Any

class SigmaBuyHatke:
    def analyze_deal(self, product_name: str, current_price: int) -> Dict:
        """USP: Price Graph Analysis. Tells if it's the right time to buy."""
        history = self.get_price_history(product_name)
        lowest = min(history)
        avg = sum(history) / len(history)
        status = 'EXCELLENT' if current_price <= lowest else 'DECENT' if current_price < avg else 'WAIT'
        diff = current_price - lowest
        return {'Product': product_name, 'Current': current_price, 'Lowest_Ever': lowest, 'Average': int(avg), 'Verdict': status, 'Savings_Potential': diff if diff > 0 else 0}