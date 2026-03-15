# Generated method: FlowPage.__init__
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class FlowPage:
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, '🌿 SigmaFlowAI', 'Procedural Logic Architect & Auditor')
        self.build()