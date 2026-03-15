"""
Auto-split from userland\system_api\gui_pkg\config_hub.py — ConfigHubPage._show_cfg
"""

import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO, FONT_MED, FONT_BOLD, FONT_TITLE, FONT_SMALL



class ConfigHubPage:
    def _show_cfg(self, cat):
        for w in self.c_fr.winfo_children():
            w.destroy()
        if cat == 'System':
            self._cfg_system(self.c_fr)
        elif cat == 'Display':
            self._cfg_display(self.c_fr)
        elif cat == 'Network':
            self._cfg_network(self.c_fr)
        elif cat == 'Security':
            self._cfg_security(self.c_fr)
        elif cat == 'Safety':
            self._cfg_safety(self.c_fr)
        elif cat == 'Sovereignty':
            self._cfg_sovereignty(self.c_fr)
        elif cat == 'About' or cat == 'Info':
            self._cfg_about(self.c_fr)
