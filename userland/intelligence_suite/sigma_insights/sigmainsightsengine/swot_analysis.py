# Generated method: SigmaInsightsEngine.swot_analysis
import math
import time
from typing import List, Dict, Any, Optional

class SigmaInsightsEngine:
    def swot_analysis(self, strengths: List[str], weaknesses: List[str], opportunities: List[str], threats: List[str]) -> str:
        """Generates a structured SWOT analysis report."""
        report = ['\n--- STRATEGIC SWOT ANALYSIS ---', f'\n[STRENGTHS]:\n  - ' + '\n  - '.join(strengths), f'\n[WEAKNESSES]:\n  - ' + '\n  - '.join(weaknesses), f'\n[OPPORTUNITIES]:\n  - ' + '\n  - '.join(opportunities), f'\n[THREATS]:\n  - ' + '\n  - '.join(threats), '\n--- END OF REPORT ---']
        return '\n'.join(report)