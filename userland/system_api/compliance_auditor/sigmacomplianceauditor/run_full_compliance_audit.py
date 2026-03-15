# Generated method: SigmaComplianceAuditor.run_full_compliance_audit
import os
import sys
import platform
import subprocess
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaComplianceAuditor:
    def run_full_compliance_audit(self) -> Dict[str, Any]:
        """Runs a deep audit of system hardening and sovereignty."""
        checks = {'Sovereignty': self._check_sovereignty(), 'Hardening': self._check_hardening(), 'Privacy': self._check_privacy(), 'Optimization': self._check_optimization()}
        passed = sum((1 for v in checks.values() if v['status'] == 'PASS'))
        self.last_audit_score = passed / len(checks) * 100
        return {'score': f'{self.last_audit_score:.1f}%', 'detailed_report': checks, 'recommendation': 'System is within nominal sovereign parameters.' if passed == len(checks) else 'Hardening recommended.'}