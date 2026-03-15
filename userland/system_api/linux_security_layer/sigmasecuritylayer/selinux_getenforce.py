# Generated method: SigmaSecurityLayer.selinux_getenforce
import time
import json
from pathlib import Path
from typing import Dict, List, Any

class SigmaSecurityLayer:
    def selinux_getenforce(self) -> str:
        return self.state['selinux']['mode'].capitalize()