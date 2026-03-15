# Generated method: SigmaSSL.get_status
from typing import Dict, List, Any

class SigmaSSL:
    def get_status(self) -> Dict:
        return {'Active_Instances': self._instances, 'Reserved_RAM': self._memory_usage, 'Binary_Support': self._supported_binaries}