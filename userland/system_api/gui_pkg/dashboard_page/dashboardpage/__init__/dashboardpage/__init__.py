# Generated method: DashboardPage.__init__
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_MED, FONT_BOLD, FONT_SMALL

class DashboardPage:
    def __init__(self, parent, gui):
        is_child = gui._is_child_mode()
        title = 'Kiddie Playroom' if is_child else 'Sovereign Dashboard'
        subtitle = 'Everything is Happy & Safe!' if is_child else 'System Health & Core Telemetry'
        SigmaPage.__init__(self, parent, gui, title, subtitle)
        self.build()