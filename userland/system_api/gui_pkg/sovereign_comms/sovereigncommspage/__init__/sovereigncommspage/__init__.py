# Generated method: SovereignCommsPage.__init__
import tkinter as tk
from tkinter import ttk
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_BOLD, FONT_SMALL, FONT_LOGO
import random

class SovereignCommsPage:
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, 'Communication Engine', 'Unified Sovereign Messaging — CRM/WhatsApp/AI Synthesis')
        self._build_ui()