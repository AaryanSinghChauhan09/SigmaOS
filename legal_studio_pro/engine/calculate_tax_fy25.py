# Generated file: calculate_tax_fy25
from typing import Dict, List, Any, Optional
import datetime
import json
import os

def calculate_tax_fy25(income_lakhs: float) -> float:
    """FY 2024-25 New Regime slabs."""
    if income_lakhs <= 3:
        return 0
    if income_lakhs <= 7:
        return (income_lakhs - 3) * 0.05
    return 0.2