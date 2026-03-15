# Generated method: SigmaModeManager._run_compliance_audit
from typing import Dict, List, Any, Callable, Optional
import time

class SigmaModeManager:
    def _run_compliance_audit(self, phase: str='') -> str:
        if self.kernel and self.kernel.compliance:
            return str(self.kernel.compliance.run_full_compliance_audit())
        return 'Compliance Auditor offline.'