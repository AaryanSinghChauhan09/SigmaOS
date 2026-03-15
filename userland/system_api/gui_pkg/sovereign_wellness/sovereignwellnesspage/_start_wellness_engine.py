"""
Auto-split from userland\system_api\gui_pkg\sovereign_wellness.py — SovereignWellnessPage._start_wellness_engine
"""

import tkinter as tk
from tkinter import ttk, messagebox
import time
import random
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED, FONT_MONO



class SovereignWellnessPage:
    def _start_wellness_engine(self):

        def _loop():
            if not self.winfo_exists():
                return
            curr_min = int(time.time() / 60)
            if curr_min % 20 == 0:
                self.gui._notify('Wellness: Eye Break', 'Follow 20-20-20 rule. Look away now.', 'OK')
            self.after(60000, _loop)
        self.after(60000, _loop)
