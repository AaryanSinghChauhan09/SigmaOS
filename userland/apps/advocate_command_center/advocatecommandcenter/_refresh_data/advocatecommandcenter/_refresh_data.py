# Generated method: AdvocateCommandCenter._refresh_data
import tkinter as tk
from tkinter import ttk, messagebox
from typing import Dict, Any, List, Optional, Union, Callable
from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT

class AdvocateCommandCenter:
    def _refresh_data(self):
        data = [('2026-03-15', 'Union of India vs XYZ', 'SC - Court 1', 'Final Arguments', 'CONFIRMED'), ('2026-03-16', 'Sharma vs State of UP', 'HC - Alld', 'Charge Framing', 'LISTED'), ('2026-03-18', 'Asset Recovery #404', 'DRT - Delhi', 'Evidence', 'ADJOURNED')]
        for item in self.hearing_tree.get_children():
            self.hearing_tree.delete(item)
        for entry in data:
            self.hearing_tree.insert('', 'end', values=entry)