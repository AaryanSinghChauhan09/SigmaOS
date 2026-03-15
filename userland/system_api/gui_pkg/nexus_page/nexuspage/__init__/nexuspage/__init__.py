# Generated method: NexusPage.__init__
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED, FONT_LOGO

class NexusPage:
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, '🧬 Sigma AI Nexus', 'Universal Intelligence Gateway & Model Registry')
        self.build()