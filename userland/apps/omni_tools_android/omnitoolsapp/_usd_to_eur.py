"""
Auto-split from userland\apps\omni_tools_android.py — OmniToolsApp._usd_to_eur
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



class OmniToolsApp:
    def _usd_to_eur(self) -> None:
        try:
            self.eur_entry.delete(0, tk.END)
            self.eur_entry.insert(0, fmt(float(self.usd_entry.get()) * 0.92))
        except ValueError:
            messagebox.showinfo('Currency', 'Enter a valid amount.')
