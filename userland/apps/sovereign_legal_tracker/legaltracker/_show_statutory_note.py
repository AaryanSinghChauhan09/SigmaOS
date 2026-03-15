# Generated method: LegalTracker._show_statutory_note
import tkinter as tk
from tkinter import ttk, messagebox
import sys
import os
from typing import Dict, Any, List, Optional
from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT
from sigma_core.legal.legal_engine import LegalEngine

class LegalTracker:
    def _show_statutory_note(self, stage):
        messagebox.showinfo('Statutory Note', f"{stage['name']}\n\nAct Reference: {stage['act']}\n\nLogic: {stage['note']}")