# Generated method: SigmaAuditor.run
import os
import sys
import importlib

class SigmaAuditor:
    def run(self):
        print('=== SIGMA OS SOVEREIGN AUDIT ===')
        self.audit_manifest()
        self.audit_gui_pkg()
        print('\nAudit Results:')
        print(f'Errors: {len(self.errors)}')
        print(f'Warnings: {len(self.warnings)}')
        for e in self.errors:
            print(f'  [X] {e}')
        for w in self.warnings:
            print(f'  [!] {w}')
        if not self.errors:
            print('\n[VERDICT] SYSTEM INTEGRITY: VERIFIED (APEX READY)')
        else:
            print('\n[VERDICT] SYSTEM INTEGRITY: COMPROMISED')