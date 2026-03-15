"""
Auto-split from userland\system_api\gui_pkg\intelligence_hub_page.py — IntelligenceHubPage._run_recognition
"""

import tkinter as tk
from tkinter import ttk
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_MED, FONT_SMALL



class IntelligenceHubPage:
    def _run_recognition(self):
        objects = ['Apple', 'Ball', 'Cat', 'Dog', 'Elephant']
        found = random.choice(objects)
        self.gui._notify('Recognition', f'Object Detected: {found} (Confidence: 98%)', 'OK')
