# Generated method: AutomationHubPage.__init__
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_DIM, FONT_BOLD

class AutomationHubPage:
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, 'OMNI AUTOMATOR STUDIO', 'Zero-Trust Agentic Automation & Workflow Forging')
        self.build()