# Generated method: SovereignConcierge._next_step
import tkinter as tk
from tkinter import ttk, messagebox
from typing import Optional, Dict, Any
import uuid
from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT

class SovereignConcierge:
    def _next_step(self):
        self.current_step += 1
        if self.current_step >= len(self.steps):
            messagebox.showinfo('SigmaOS', 'Concierge setup complete. Welcome home.')
            self.destroy()
            return
        self.title_lbl.config(text=self.steps[self.current_step]['title'])
        self.desc_lbl.config(text=self.steps[self.current_step]['desc'])
        self.progress['value'] = (self.current_step + 1) * 25
        if self.current_step == 2:
            self._mock_download()