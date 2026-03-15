# Generated method: SigmaBharatLawBridge.calculate_income_tax_estimate
from typing import Dict, List, Any, Optional
import datetime

class SigmaBharatLawBridge:
    def calculate_income_tax_estimate(self, annual_income: float, regime: str='New') -> str:
        """New vs Old Tax Regime slabs (FY 2024-25 Finance Bill context)."""
        if regime == 'New':
            lakhs = annual_income / 100000
            tax = 0
            if lakhs > 15:
                tax += (lakhs - 15) * 30000
            if lakhs > 12:
                tax += min(lakhs - 12, 3) * 20000
            if lakhs > 10:
                tax += min(lakhs - 10, 2) * 15000
            if lakhs > 7:
                tax += min(lakhs - 7, 3) * 10000
            if lakhs > 3:
                tax += min(lakhs - 3, 4) * 5000
            return f'Income Tax Estimate (New Regime): ₹{tax:,.2f} (Annual: ₹{annual_income:,.2f}).'
        return 'Manual Calculation required for Old Regime (Deductions based).'