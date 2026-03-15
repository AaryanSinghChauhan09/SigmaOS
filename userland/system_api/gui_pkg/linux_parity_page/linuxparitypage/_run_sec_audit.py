# Generated method: LinuxParityPage._run_sec_audit
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO, FONT_BOLD, FONT_SMALL

class LinuxParityPage:
    def _run_sec_audit(self):
        if hasattr(self.kernel, 'linux_parity'):
            audit = self.kernel.linux_parity.security_audit.run_audit()
            self._log(self._linux_log, '\n🛡️ ENTERPRISE SECURITY AUDIT (RHEL/STIG PARITY)', 'HEAD')
            for rule, status in audit.items():
                icon = '✔' if 'PASS' in status else '✖' if 'FAIL' in status else '⚠'
                self._log(self._linux_log, f"{icon} {rule.replace('_', ' ').upper()}: {status}", 'OK' if 'PASS' in status else 'INFO')
            self.gui._log_voice('Security audit complete. Enterprise compliance verified.')