# Generated method: SigmaAuditor.run_full_audit
import time
import random
import os
from typing import Dict, List, Any

class SigmaAuditor:
    def run_full_audit(self) -> Dict[str, Any]:
        """Executes all test categories and returns a comprehensive report."""
        report = {'timestamp': time.strftime('%Y-%m-%d %H:%M:%S'), 'categories': {'Installation & Boot': self.test_boot_stability(), 'Core Functionality': self.test_core_logic(), 'Performance Benchmarks': self.test_performance(), 'Security & Permissions': self.test_security_perimeter(), 'Reliability & Recovery': self.test_recovery_logic(), 'Update & Patching': self.test_update_management(), 'Virtualization & Cloud': self.test_virtualization(), 'Efficiency & Energy': self.test_energy_efficiency(), 'Scalability & Multi-User': self.test_scalability(), 'Localization & Global': self.test_localization(), 'Edge Cases & Stress': self.test_extreme_stress()}, 'overall_score': 0, 'status': 'COMPLETED'}
        scores = [cat['score'] for cat in report['categories'].values()]
        report['overall_score'] = sum(scores) / len(scores)
        self._last_report = report
        self.test_history.append(report)
        return report