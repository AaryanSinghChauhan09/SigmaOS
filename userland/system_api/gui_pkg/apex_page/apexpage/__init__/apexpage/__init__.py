# Generated method: ApexPage.__init__
import tkinter as tk
from tkinter import ttk, messagebox
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED, FONT_LOGO, FONT_MONO

class ApexPage:
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, '🏔️ Sovereign Apex', 'Multi-OS Master Hub & Fusion Grid')
        self.build()