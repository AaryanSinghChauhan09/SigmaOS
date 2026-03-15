# Generated method: LinuxParityPage._apply_distro_tuning
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO, FONT_BOLD, FONT_SMALL

class LinuxParityPage:
    def _apply_distro_tuning(self, distro):
        self.gui._log_voice(f'Applying {distro} tuning... kernel-level parity engaged.')
        if hasattr(self.kernel, 'linux_parity'):
            self.kernel.linux_parity.apply_distro_mimic(distro)
        self.kernel.modes.switch_mode(f'{distro}_Desktop' if distro != 'Kali' else 'Kali_Security')
        self._log(self._linux_log, f'✔ Switched to {distro} compatibility profile.', 'OK')