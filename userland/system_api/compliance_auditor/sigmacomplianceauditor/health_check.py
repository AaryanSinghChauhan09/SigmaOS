# Generated method: SigmaComplianceAuditor.health_check
import os
import sys
import platform
import subprocess
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaComplianceAuditor:
    def health_check(self) -> str:
        return f'OK - Last Audit Score: {self.last_audit_score}%'