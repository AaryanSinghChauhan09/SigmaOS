# Generated method: ClawPage.__init__
import tkinter as tk
from tkinter import ttk, scrolledtext
from .base_page import SigmaPage
from .styles import PAL, FONT_TITLE, FONT_MED, FONT_MONO
from sigma_core.ai.sovereign_claw import SovereignClaw

class ClawPage:
    def __init__(self, parent, gui):
        super().__init__(parent, gui, 'SOVEREIGN CLAW', 'Action Intelligence & AI Automation')
        self.claw = SovereignClaw(gui.kernel)
        self._build_interface()