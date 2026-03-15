"""
Auto-split from userland\system_api\gui_pkg\terminal_page.py — TerminalPage._term_aux
"""

import tkinter as tk
from tkinter import ttk
import threading
from .base_page import SigmaPage
from .styles import PAL, FONT_MONO



class TerminalPage:
    def _term_aux(self, cmd):
        self._log(self._term_out, f'AUX: Initializing {cmd} sub-module...', 'INFO')
        distillator = self.kernel.registry.get('neural_distillator')
        if distillator:
            self._log(self._term_out, f'Neural Context: {distillator.query_distilled_knowledge(cmd)}', 'OK')
