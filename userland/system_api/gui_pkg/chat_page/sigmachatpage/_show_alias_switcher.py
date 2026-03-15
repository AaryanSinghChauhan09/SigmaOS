"""
Auto-split from userland\system_api\gui_pkg\chat_page.py — SigmaChatPage._show_alias_switcher
"""

import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL



class SigmaChatPage:
    def _show_alias_switcher(self):
        new_alias = self.gui._prompt_input('Identity Shift', 'Enter new ephemeral alias:')
        if new_alias and self.engine:
            res = self.engine.switch_alias(new_alias)
            self.alias_var.set(f'(@{new_alias})')
            self._notify('Sovereign Identity', res, 'OK')
            self._insert_log('SYSTEM', 'Zero-Knowledge Identity Rotated.', is_me=False)
