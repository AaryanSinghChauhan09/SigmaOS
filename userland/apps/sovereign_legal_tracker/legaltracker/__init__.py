# Generated method: LegalTracker.__init__
import tkinter as tk
from tkinter import ttk, messagebox
import sys
import os
from typing import Dict, Any, List, Optional
from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT
from sigma_core.legal.legal_engine import LegalEngine

class LegalTracker:
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.engine = LegalEngine()
        self.title('Sovereign Legal Tracker — Litigation Gantt')
        self.geometry('1400x800')
        self.configure(bg=PAL['background'])
        self.header = tk.Frame(self)
        self.gantt_fr = tk.Frame(self)
        self.canvas = tk.Canvas(self)
        self.info_panel = tk.Frame(self)
        self.stages = self.engine.get_stages()
        self._build_ui()