# Generated method: SigmaWriteSense.check_brand_compliance
from typing import Dict, List, Any
import re

class SigmaWriteSense:
    def check_brand_compliance(self, text: str) -> Dict:
        """USP: Writer.com Style Brand Voice Check."""
        compliant = 'SigmaOS' in text or 'Sovereign' in text
        return {'Voice': self._brand_voice, 'Compliant': compliant, 'Score': 100 if compliant else 40, 'Notes': "Ensure brand keywords like 'Sovereign' are included."}