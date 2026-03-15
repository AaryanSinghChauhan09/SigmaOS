# Generated method: LegalTracker._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import sys
import os
from typing import Dict, Any, List, Optional
from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT
from sigma_core.legal.legal_engine import LegalEngine

class LegalTracker:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['background'], height=80, padx=40)
        self.header.pack(side='top', fill='x', pady=20)
        tk.Label(self.header, text='LITIGATION TRACKER', font=FONT['h1'], fg=PAL['primary'], bg=PAL['background']).pack(side='left')
        self.gantt_fr = tk.Frame(self, bg=PAL['surface'], padx=20, pady=20)
        self.gantt_fr.pack(fill='both', expand=True, padx=40, pady=(0, 20))
        self.canvas = tk.Canvas(self.gantt_fr, bg=PAL['surface'], highlightthickness=0)
        self.canvas.pack(fill='both', expand=True)
        self._draw_gantt()
        self.info_panel = tk.Frame(self, bg=PAL['surface_variant'], height=100, padx=20, pady=15)
        self.info_panel.pack(side='bottom', fill='x')
        tk.Label(self.info_panel, text='STATUTORY TIMELINE SYNCED WITH INDIACODE.NIC.IN', font=FONT['caption'], fg=PAL['text_secondary'], bg=PAL['surface_variant']).pack()