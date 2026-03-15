"""
Auto-split from userland\system_api\gui_pkg\sovereign_wellness.py — SovereignWellnessPage._toggle_grayscale
"""

import tkinter as tk
from tkinter import ttk, messagebox
import time
import random
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED, FONT_MONO



class SovereignWellnessPage:
    def _toggle_grayscale(self):
        self.gui._log_voice('Zen: Toggling Grayscale filter to minimize visual distraction.')
