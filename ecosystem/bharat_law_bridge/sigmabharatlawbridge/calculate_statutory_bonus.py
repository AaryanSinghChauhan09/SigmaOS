"""
Auto-split from ecosystem\bharat_law_bridge.py — SigmaBharatLawBridge.calculate_statutory_bonus
"""

from typing import Dict, List, Any, Optional
import datetime



class SigmaBharatLawBridge:
    def calculate_statutory_bonus(self, annual_salary: float, bonus_percentage: float=8.33) -> str:
        """Payment of Bonus Act formula (8.33% to 20%)."""
        if bonus_percentage < 8.33:
            bonus_percentage = 8.33
        bonus = annual_salary * bonus_percentage / 100
        return f'Statutory Bonus Entitlement: ₹{bonus:,.2f} (Calculated at {bonus_percentage}%).'
