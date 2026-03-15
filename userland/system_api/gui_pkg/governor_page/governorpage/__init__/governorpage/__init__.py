# Generated method: GovernorPage.__init__
import tkinter as tk
from tkinter import ttk
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_TITLE, FONT_MONO

class GovernorPage:
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, 'SOVEREIGN GOVERNOR', 'Entropy-Aware Orchestration & Cross-Device Mind-Sync')
        self.build()