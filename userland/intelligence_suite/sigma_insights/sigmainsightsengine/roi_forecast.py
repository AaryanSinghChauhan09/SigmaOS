# Generated method: SigmaInsightsEngine.roi_forecast
import math
import time
from typing import List, Dict, Any, Optional

class SigmaInsightsEngine:
    def roi_forecast(self, investment: float, expected_return: float, period_months: int) -> Dict[str, Any]:
        """Calculates ROI and annualized growth for business stakeholders."""
        total_profit = expected_return - investment
        roi = total_profit / investment * 100 if investment > 0 else 0
        years = period_months / 12
        cagr = ((expected_return / investment) ** (1 / years) - 1) * 100 if investment > 0 and years > 0 else 0
        return {'investment': float(investment), 'net_profit': float(int(total_profit * 100)) / 100.0, 'roi_percent': float(int(roi * 100)) / 100.0, 'cagr_percent': float(int(cagr * 100)) / 100.0, 'payback_period': float(int(investment / (total_profit / period_months) * 100)) / 100.0 if total_profit > 0 else 'N/A'}