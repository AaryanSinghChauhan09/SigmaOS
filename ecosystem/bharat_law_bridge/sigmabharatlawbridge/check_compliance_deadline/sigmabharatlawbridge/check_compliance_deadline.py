# Generated method: SigmaBharatLawBridge.check_compliance_deadline
from typing import Dict, List, Any, Optional
import datetime

class SigmaBharatLawBridge:
    def check_compliance_deadline(self, start_date: str, duration_days: int) -> str:
        """Calculates limitation periods and filing deadlines."""
        try:
            start = datetime.datetime.strptime(start_date, '%Y-%m-%d')
            deadline = start + datetime.timedelta(days=duration_days)
            remaining = (deadline - datetime.datetime.now()).days
            return f"Compliance Alert: Deadline for filing is {deadline.strftime('%Y-%m-%d')}. Days remaining: {remaining}."
        except ValueError:
            return 'Error: Invalid date format. Use YYYY-MM-DD.'