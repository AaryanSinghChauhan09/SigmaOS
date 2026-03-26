# Generated method: MailOrchestratorPage.__init__
import tkinter as tk
from tkinter import ttk
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MONO

class MailOrchestratorPage:
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, 'Mail Orchestrator', 'Sovereign Mail Merge & AI Cognitive Drafting Assistant')
        self._build_ui()