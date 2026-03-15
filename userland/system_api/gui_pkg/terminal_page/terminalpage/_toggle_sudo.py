"""
Auto-split from userland\system_api\gui_pkg\terminal_page.py — TerminalPage._toggle_sudo
"""

import tkinter as tk
from tkinter import ttk
import threading
from .base_page import SigmaPage
from .styles import PAL, FONT_MONO



class TerminalPage:
    def _toggle_sudo(self):
        curr = self._is_elevated.get()
        self._is_elevated.set(not curr)
        if not curr:
            self._log(self._term_out, 'ELEVATING PRIVILEGES: Biometric Audit Passed. [ROOT ACTIVE]', 'WARN')
            self._sudo_btn.config(fg='white', bg=PAL['red'])
            self.prompt_lbl.config(text='# ')
        else:
            self._log(self._term_out, 'DROPPING PRIVILEGES: User mode restored.', 'INFO')
            self._sudo_btn.config(fg=PAL['dim'], bg=PAL['bg3'])
            self.prompt_lbl.config(text='σ > ')
