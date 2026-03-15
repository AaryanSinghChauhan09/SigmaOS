"""
Auto-split from userland\apps\omni_tools_android.py — OmniToolsApp._ft_to_m
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



class OmniToolsApp:
    def _ft_to_m(self) -> None:
        try:
            self.meter_entry.delete(0, tk.END)
            self.meter_entry.insert(0, fmt(float(self.feet_entry.get()) / 3.28084))
        except ValueError:
            messagebox.showinfo('Converter', 'Enter a valid number.')
