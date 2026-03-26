# Generated method: AnalyticsPage.__init__
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_TITLE, FONT_SMALL, FONT_BOLD, FONT_MED

class AnalyticsPage:
    def __init__(self, parent, gui):
        super().__init__(parent, gui, 'Productivity Intelligence', 'Sovereign Analytics Dashboard — Velocity & Real-time BI')
        self._build_ui()