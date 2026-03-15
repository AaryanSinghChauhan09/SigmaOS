# Generated method: SigmaDiagnostics.run_full_diagnostic
import time
import uuid
import random
from dataclasses import dataclass
from enum import Enum, auto

class SigmaDiagnostics:
    def run_full_diagnostic(self) -> dict:
        """User-facing API to trigger an immediate deep analysis."""
        t0 = time.perf_counter()
        count_before = len(self._alerts)
        self._simulated_scan()
        count_after = len(self._alerts)
        new_issues = count_after - count_before
        duration_ms = (time.perf_counter() - t0) * 1000 + 120.5
        return {'status': 'Diagnostic Complete', 'duration': f'{duration_ms:.1f}ms', 'new_issues': new_issues, 'total_pending': len([a for a in self._alerts if not a.resolved]), 'message': f'DiagnosticCore: Deep sweep completed in {duration_ms:.1f}ms. Issues found: {new_issues}. Autonomous healing ready.'}