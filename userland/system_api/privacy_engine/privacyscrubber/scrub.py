# Generated method: PrivacyScrubber.scrub
import hashlib
import json
import re
from sigma_core.system.interfaces import ISigmaModule, SigmaModuleBase

class PrivacyScrubber:
    def scrub(self, data: str) -> str:
        """Replace sensitive patterns with [SCRUBBED] dynamically before saving."""
        if not isinstance(data, str):
            return data
        clean_data = data
        for pattern in self._pii_patterns:
            clean_data = re.sub(pattern, '[SCRUBBED]', clean_data, flags=re.IGNORECASE)
        return clean_data