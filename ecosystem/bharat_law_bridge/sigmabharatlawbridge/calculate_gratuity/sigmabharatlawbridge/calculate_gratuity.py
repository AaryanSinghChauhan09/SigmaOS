# Generated method: SigmaBharatLawBridge.calculate_gratuity
from typing import Dict, List, Any, Optional
import datetime

class SigmaBharatLawBridge:
    def calculate_gratuity(self, last_drawn_salary: float, tenure_years: int) -> str:
        """Payment of Gratuity Act, 1972 formula: (15 * Salary * Tenure) / 26."""
        if tenure_years < 5:
            return 'Error: Minimum 5 years of service required for Gratuity eligibility.'
        gratuity = 15 * last_drawn_salary * tenure_years / 26
        return f'Legal Gratuity Entitlement: ₹{gratuity:,.2f} (Formula: 15/26 Rule).'