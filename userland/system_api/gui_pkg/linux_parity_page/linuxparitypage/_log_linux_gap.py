# Generated method: LinuxParityPage._log_linux_gap
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO, FONT_BOLD, FONT_SMALL

class LinuxParityPage:
    def _log_linux_gap(self, distro):
        if hasattr(self.kernel, 'linux_parity'):
            res = self.kernel.linux_parity.gap_analysis.generate_report(distro)
            self._log(self._linux_log, f'\n🔎 GAP ANALYSIS: SigmaOS vs {distro}', 'HEAD')
            self._log(self._linux_log, res, 'OK')